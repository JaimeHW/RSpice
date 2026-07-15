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
    let staged_digest: [u8; 32] = Sha256::digest(bytes).into();
    let predecessor_permissions = std::fs::metadata(path)
        .ok()
        .map(|metadata| metadata.permissions());
    let (temp, mut file) = create_unique_sibling(path)?;

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

fn read_windows_recovery_record(path: &Path) -> io::Result<Option<RecoveryRecord>> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let file_len = file.metadata()?.len();
    if file_len < RECOVERY_HEADER_LEN as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' is truncated", path.display()),
        ));
    }
    let mut header = [0_u8; RECOVERY_HEADER_LEN];
    file.read_exact(&mut header)?;
    let allowed_flags = RECOVERY_FLAG_BEFORE_PRESENT | RECOVERY_FLAG_READ_ONLY;
    if header[..16] != RECOVERY_HEADER_MAGIC
        || header[89..96].iter().any(|byte| *byte != 0)
        || header[88] & !allowed_flags != 0
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has an invalid header", path.display()),
        ));
    }
    let expected_header_checksum: [u8; 32] = Sha256::new()
        .chain_update(b"rspice-windows-recovery-header\0v1\0")
        .chain_update(&header[..RECOVERY_HEADER_PREFIX_LEN])
        .finalize()
        .into();
    let mut header_checksum = [0_u8; 32];
    header_checksum.copy_from_slice(&header[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN]);
    if header_checksum != expected_header_checksum {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' failed header verification",
                path.display()
            ),
        ));
    }

    let mut generation = [0_u8; 8];
    generation.copy_from_slice(&header[16..24]);
    let generation = u64::from_le_bytes(generation);
    if generation == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has generation zero", path.display()),
        ));
    }
    let mut transaction = [0_u8; 16];
    transaction.copy_from_slice(&header[24..40]);
    let mut backup_id = [0_u8; 16];
    backup_id.copy_from_slice(&header[40..56]);
    let mut target_binding = [0_u8; 32];
    target_binding.copy_from_slice(&header[56..88]);
    let before_present = header[88] & RECOVERY_FLAG_BEFORE_PRESENT != 0;
    let mut predecessor_len = [0_u8; 8];
    predecessor_len.copy_from_slice(&header[96..104]);
    let predecessor_len = u64::from_le_bytes(predecessor_len);
    let mut successor_len = [0_u8; 8];
    successor_len.copy_from_slice(&header[104..112]);
    let successor_len = u64::from_le_bytes(successor_len);
    if predecessor_len > MAX_HASHED_FILE_BYTES || successor_len > MAX_HASHED_FILE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' declares an image larger than the supported {MAX_HASHED_FILE_BYTES} byte limit",
                path.display()
            ),
        ));
    }
    let mut predecessor_digest = [0_u8; 32];
    predecessor_digest.copy_from_slice(&header[112..144]);
    let mut successor_digest = [0_u8; 32];
    successor_digest.copy_from_slice(&header[144..176]);
    let empty_digest: [u8; 32] = Sha256::digest([]).into();
    if !before_present && (predecessor_len != 0 || predecessor_digest != empty_digest) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' has bytes for an absent predecessor",
                path.display()
            ),
        ));
    }
    let payload_end = (RECOVERY_HEADER_LEN as u64)
        .checked_add(predecessor_len)
        .and_then(|len| len.checked_add(successor_len))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "recovery size overflow"))?;
    if file_len < payload_end {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("recovery bundle '{}' has a truncated image", path.display()),
        ));
    }
    if hash_exact_section(&mut file, predecessor_len)? != predecessor_digest
        || hash_exact_section(&mut file, successor_len)? != successor_digest
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' failed image verification",
                path.display()
            ),
        ));
    }

    let trailing = file_len - payload_end;
    if trailing > RECOVERY_RESOLUTION_LEN as u64 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "recovery bundle '{}' has unexpected trailing data",
                path.display()
            ),
        ));
    }
    let transaction = Uuid::from_bytes(transaction);
    let state = if trailing == RECOVERY_RESOLUTION_LEN as u64 {
        let mut trailer = [0_u8; RECOVERY_RESOLUTION_LEN];
        file.read_exact(&mut trailer)?;
        decode_recovery_resolution(&trailer, generation, transaction, header_checksum)
            .map(RecoveryRecordState::Resolved)
            .unwrap_or(RecoveryRecordState::Prepared)
    } else {
        // A torn append can only lose resolution, never either retained image.
        // Treat it as Prepared and infer the outcome from exact target bytes.
        RecoveryRecordState::Prepared
    };
    Ok(Some(RecoveryRecord {
        generation,
        transaction,
        backup_id: Uuid::from_bytes(backup_id),
        target_binding,
        before_present,
        predecessor_len,
        successor_len,
        predecessor_digest,
        successor_digest,
        predecessor_read_only: header[88] & RECOVERY_FLAG_READ_ONLY != 0,
        header_checksum,
        state,
    }))
}

fn seal_file_image(
    source_path: &Path,
    destination: &mut File,
    expected_len: u64,
    expected_digest: [u8; 32],
) -> io::Result<()> {
    let mut source = File::open(source_path)?;
    let mut total = 0_u64;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = source.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        total = total.checked_add(read as u64).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, "recovery image size overflow")
        })?;
        if total > MAX_HASHED_FILE_BYTES {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "recovery image '{}' exceeds the supported {MAX_HASHED_FILE_BYTES} byte limit",
                    source_path.display()
                ),
            ));
        }
        hasher.update(&buffer[..read]);
        destination.write_all(&buffer[..read])?;
    }
    let actual_digest: [u8; 32] = hasher.finalize().into();
    if total != expected_len || actual_digest != expected_digest {
        return Err(io::Error::other(format!(
            "'{}' changed while its recovery image was being sealed",
            source_path.display()
        )));
    }
    Ok(())
}

fn target_binding_digest(path: &Path) -> io::Result<[u8; 32]> {
    let absolute = std::path::absolute(path)?;
    let mut hasher = Sha256::new();
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt as _;
        hasher.update(b"rspice-unix-recovery-target\0v1\0");
        hasher.update(absolute.as_os_str().as_bytes());
    }
    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt as _;
        hasher.update(b"rspice-windows-recovery-target\0v1\0");
        for unit in absolute.as_os_str().encode_wide() {
            hasher.update(unit.to_le_bytes());
        }
    }
    Ok(hasher.finalize().into())
}

fn select_active_windows_recovery(
    target: &Path,
) -> Result<Option<ActiveRecoveryRecord>, PublicationFailure> {
    let binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    let slots = recovery_slot_paths(target);
    let mut valid = Vec::new();
    let mut invalid = Vec::new();

    for (slot_index, path) in slots.iter().enumerate() {
        match read_windows_recovery_record(path) {
            Ok(Some(record)) if record.target_binding == binding => {
                valid.push(ActiveRecoveryRecord {
                    slot_index,
                    path: path.clone(),
                    record,
                });
            }
            Ok(Some(_)) => invalid.push((
                path.clone(),
                "target binding does not match the canonical pathname".to_owned(),
            )),
            Ok(None) => {}
            Err(error) => invalid.push((path.clone(), error.to_string())),
        }
    }

    if !invalid.is_empty() {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "foreign or corrupted recovery evidence exists for '{}': {invalid:?}",
                target.display()
            ),
            recovery_paths: invalid.into_iter().map(|(path, _)| path).collect(),
        });
    }

    match valid.len() {
        0 => Ok(None),
        1 => Ok(valid.pop()),
        2 => {
            let right = valid.pop().expect("two records have a right member");
            let left = valid.pop().expect("two records have a left member");
            if left.record.generation == right.record.generation {
                if left.record == right.record {
                    return Ok(Some(left));
                }
                return Err(PublicationFailure::Uncertain {
                    message: format!(
                        "recovery slots for '{}' contain ambiguous generation {} transactions",
                        target.display(),
                        left.record.generation
                    ),
                    recovery_paths: vec![left.path, right.path],
                });
            }
            Ok(Some(if left.record.generation > right.record.generation {
                left
            } else {
                right
            }))
        }
        _ => unreachable!("there are exactly two recovery slots"),
    }
}

#[cfg(windows)]
fn install_windows_recovery_bundle(
    target: &Path,
    staged: &Path,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
) -> Result<ActiveRecoveryRecord, PublicationFailure> {
    install_recovery_bundle(
        target,
        staged,
        Some(predecessor_digest),
        successor_digest,
        Uuid::new_v4(),
    )
}

fn install_recovery_bundle(
    target: &Path,
    staged: &Path,
    predecessor_digest: Option<[u8; 32]>,
    successor_digest: [u8; 32],
    backup_id: Uuid,
) -> Result<ActiveRecoveryRecord, PublicationFailure> {
    let active = select_active_windows_recovery(target)?;
    let generation =
        active
            .as_ref()
            .map_or(Ok(1), |active| {
                active.record.generation.checked_add(1).ok_or_else(|| {
                    io::Error::other("native recovery generation counter is exhausted")
                })
            })
            .map_err(PublicationFailure::Safe)?;
    let slot_index = active.as_ref().map_or(0, |active| 1 - active.slot_index);
    let path = recovery_slot_paths(target)[slot_index].clone();
    let transaction = Uuid::new_v4();
    let target_binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    let successor_metadata = std::fs::metadata(staged).map_err(PublicationFailure::Safe)?;
    let predecessor_metadata = match predecessor_digest {
        Some(_) => Some(std::fs::metadata(target).map_err(PublicationFailure::Safe)?),
        None => {
            if !matches!(target.try_exists(), Ok(false)) {
                return Err(PublicationFailure::Safe(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "recovery expected missing destination '{}'",
                        target.display()
                    ),
                )));
            }
            None
        }
    };
    let predecessor_len = predecessor_metadata
        .as_ref()
        .map_or(0, std::fs::Metadata::len);
    let successor_len = successor_metadata.len();
    if predecessor_len > MAX_HASHED_FILE_BYTES || successor_len > MAX_HASHED_FILE_BYTES {
        return Err(PublicationFailure::Safe(io::Error::new(
            io::ErrorKind::FileTooLarge,
            format!(
                "recovery images exceed the supported {MAX_HASHED_FILE_BYTES} byte per-image limit"
            ),
        )));
    }
    let predecessor_read_only = predecessor_metadata
        .as_ref()
        .is_some_and(|metadata| metadata.permissions().readonly());
    let predecessor_digest = predecessor_digest.unwrap_or_else(|| Sha256::digest([]).into());
    let before_present = predecessor_metadata.is_some();
    let header = encode_recovery_header(RecoveryHeaderInput {
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
    });
    let mut header_checksum = [0_u8; 32];
    header_checksum.copy_from_slice(&header[RECOVERY_HEADER_PREFIX_LEN..]);
    let expected = RecoveryRecord {
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
        header_checksum,
        state: RecoveryRecordState::Prepared,
    };

    let (temporary, mut file) = create_unique_sibling(&path).map_err(PublicationFailure::Safe)?;
    let sealed = (|| -> io::Result<()> {
        file.write_all(&header)?;
        if before_present {
            seal_file_image(target, &mut file, predecessor_len, predecessor_digest)?;
        }
        seal_file_image(staged, &mut file, successor_len, successor_digest)?;
        file.flush()?;
        file.sync_all()
    })();
    drop(file);
    if let Err(error) = sealed {
        remove_unpublished_temp(&temporary);
        return Err(PublicationFailure::Safe(error));
    }
    match read_windows_recovery_record(&temporary) {
        Ok(Some(record)) if record == expected => {}
        observation => {
            remove_unpublished_temp(&temporary);
            return Err(PublicationFailure::Safe(io::Error::other(format!(
                "new recovery bundle failed reopen verification: {observation:?}"
            ))));
        }
    }

    // A recovery slot is an ownership record, never an overwrite target. If a
    // foreign or raced pathname exists, preserve both files and fail closed.
    let raw_result = move_no_replace_raw(&temporary, &path);
    let installed = read_windows_recovery_record(&path);
    if matches!(installed, Ok(Some(ref record)) if record == &expected) {
        sync_parent_of(&path).map_err(|error| PublicationFailure::Uncertain {
            message: format!(
                "recovery generation {generation} was installed but its directory entry could not be synchronized: {error}"
            ),
            recovery_paths: vec![path.clone()],
        })?;
        return Ok(ActiveRecoveryRecord {
            slot_index,
            path,
            record: expected,
        });
    }
    if matches!(read_windows_recovery_record(&temporary), Ok(Some(ref record)) if record == &expected)
    {
        remove_unpublished_temp(&temporary);
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(
                    "recovery-slot move reported success without installing the bundle",
                )
            },
        )));
    }
    let raw_message = raw_result
        .map(|()| "the platform call reported success".to_owned())
        .unwrap_or_else(|error| error.to_string());
    Err(PublicationFailure::Uncertain {
        message: format!(
            "recovery generation {generation} installation has an unverified postcondition ({raw_message}); installed observation: {installed:?}"
        ),
        recovery_paths: recovery_paths(&[&temporary, &path]),
    })
}

fn append_windows_recovery_resolution(
    active: &ActiveRecoveryRecord,
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let observed = read_windows_recovery_record(&active.path).map_err(|error| {
        PublicationFailure::Uncertain {
            message: format!("active recovery bundle could not be reopened: {error}"),
            recovery_paths: vec![active.path.clone()],
        }
    })?;
    let Some(mut observed) = observed else {
        return Err(PublicationFailure::Uncertain {
            message: "active recovery bundle disappeared before resolution".to_owned(),
            recovery_paths: vec![active.path.clone()],
        });
    };
    let observed_state = observed.state;
    observed.state = RecoveryRecordState::Prepared;
    let mut expected_identity = active.record.clone();
    expected_identity.state = RecoveryRecordState::Prepared;
    if observed != expected_identity {
        return Err(PublicationFailure::Uncertain {
            message: "active recovery generation changed before resolution".to_owned(),
            recovery_paths: vec![active.path.clone()],
        });
    }
    match observed_state {
        RecoveryRecordState::Resolved(existing) if existing == resolution => return Ok(()),
        RecoveryRecordState::Resolved(existing) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "recovery transaction is already resolved as {existing:?}, not {resolution:?}"
                ),
                recovery_paths: vec![active.path.clone()],
            });
        }
        RecoveryRecordState::Prepared => {}
    }
    let payload_end = (RECOVERY_HEADER_LEN as u64)
        .checked_add(active.record.predecessor_len)
        .and_then(|len| len.checked_add(active.record.successor_len))
        .ok_or_else(|| PublicationFailure::Uncertain {
            message: "recovery bundle payload size overflow".to_owned(),
            recovery_paths: vec![active.path.clone()],
        })?;
    let trailer = encode_recovery_resolution(&active.record, resolution);
    let write_result = (|| -> io::Result<()> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&active.path)?;
        file.set_len(payload_end)?;
        file.seek(std::io::SeekFrom::Start(payload_end))?;
        file.write_all(&trailer)?;
        file.flush()?;
        file.sync_all()
    })();
    if let Err(error) = write_result {
        return Err(PublicationFailure::Uncertain {
            message: format!("recovery resolution could not be synchronized: {error}"),
            recovery_paths: vec![active.path.clone()],
        });
    }
    let mut expected_resolved = active.record.clone();
    expected_resolved.state = RecoveryRecordState::Resolved(resolution);
    match read_windows_recovery_record(&active.path) {
        Ok(Some(record)) if record == expected_resolved => Ok(()),
        observation => Err(PublicationFailure::Uncertain {
            message: format!(
                "recovery resolution did not survive reopen verification: {observation:?}"
            ),
            recovery_paths: vec![active.path.clone()],
        }),
    }
}

#[cfg(windows)]
fn recovery_sidecar_roles(
    target: &Path,
    record: &RecoveryRecord,
) -> (PathBuf, ObservedFileRole, PathBuf, ObservedFileRole) {
    let backup = recovery_backup_path(target, record.backup_id);
    let rollback = recovery_rollback_path(target, record.backup_id);
    let backup_role =
        observe_file_role(&backup, record.predecessor_digest, record.successor_digest);
    let rollback_role = observe_file_role(
        &rollback,
        record.predecessor_digest,
        record.successor_digest,
    );
    (backup, backup_role, rollback, rollback_role)
}

#[cfg(windows)]
fn exact_or_missing(role: ObservedFileRole, exact: ObservedFileRole) -> bool {
    matches!(role, ObservedFileRole::Missing | ObservedFileRole::Both) || role == exact
}

#[cfg(windows)]
fn recovery_blocked(
    target: &Path,
    active: &ActiveRecoveryRecord,
    message: impl Into<String>,
) -> PublicationFailure {
    let slots = recovery_slot_paths(target);
    let backup = recovery_backup_path(target, active.record.backup_id);
    let rollback = recovery_rollback_path(target, active.record.backup_id);
    PublicationFailure::Uncertain {
        message: message.into(),
        recovery_paths: recovery_paths(&[target, &slots[0], &slots[1], &backup, &rollback]),
    }
}

#[cfg(windows)]
fn resolve_windows_recovery_slot(
    target: &Path,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "publication has no durable recovery generation".to_owned(),
            recovery_paths: vec![target.to_path_buf()],
        })?;
    if active.record.successor_digest != successor_digest {
        return Err(recovery_blocked(
            target,
            &active,
            "active recovery generation does not describe the staged successor",
        ));
    }
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let (_backup, backup_role, _rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    let endpoint_is_valid = match resolution {
        PublicationResolution::Committed => {
            matches!(
                target_role,
                ObservedFileRole::Replacement | ObservedFileRole::Both
            ) && exact_or_missing(backup_role, ObservedFileRole::Replaced)
                && matches!(rollback_role, ObservedFileRole::Missing)
        }
        PublicationResolution::Aborted => {
            (matches!(
                target_role,
                ObservedFileRole::Replaced | ObservedFileRole::Both
            ) || (!active.record.before_present
                && matches!(target_role, ObservedFileRole::Missing)))
                && exact_or_missing(backup_role, ObservedFileRole::Replaced)
                && exact_or_missing(rollback_role, ObservedFileRole::Replacement)
        }
    };
    if !endpoint_is_valid {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "recovery resolution {resolution:?} is inconsistent with target {target_role:?}, backup {backup_role:?}, and rollback evidence {rollback_role:?}"
            ),
        ));
    }
    append_windows_recovery_resolution(&active, resolution)
}

fn remove_verified_recovery_sidecar(
    path: &Path,
    expected_role: ObservedFileRole,
    predecessor_digest: [u8; 32],
    successor_digest: [u8; 32],
) -> io::Result<()> {
    let role = observe_file_role(path, predecessor_digest, successor_digest);
    if matches!(role, ObservedFileRole::Missing) {
        return Ok(());
    }
    if role != expected_role && !matches!(role, ObservedFileRole::Both) {
        return Err(io::Error::other(format!(
            "recovery sidecar '{}' changed before cleanup ({role:?})",
            path.display()
        )));
    }
    std::fs::remove_file(path)?;
    sync_parent_of(path)
}

#[cfg(windows)]
fn reconcile_windows_recovery_slots(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return retire_settled_windows_recovery(target);
    };
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let (backup, backup_role, rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    let action = recovery_reconciliation(
        active.record.state,
        target_role,
        active.record.before_present,
    );
    match action {
        RecoveryReconciliation::PreserveAndBlock => Err(recovery_blocked(
            target,
            &active,
            format!(
                "recovery generation {} ({:?}) cannot be reconciled with target state {target_role:?}; no pathname was changed",
                active.record.generation, active.record.state
            ),
        )),
        RecoveryReconciliation::Resolve(PublicationResolution::Aborted) => {
            if !exact_or_missing(backup_role, ObservedFileRole::Replaced)
                || !exact_or_missing(rollback_role, ObservedFileRole::Replacement)
            {
                return Err(recovery_blocked(
                    target,
                    &active,
                    format!(
                        "prepared abort has foreign backup evidence ({backup_role:?}, {rollback_role:?})"
                    ),
                ));
            }
            append_windows_recovery_resolution(&active, PublicationResolution::Aborted)?;
            remove_verified_recovery_sidecar(
                &backup,
                ObservedFileRole::Replaced,
                active.record.predecessor_digest,
                active.record.successor_digest,
            )
            .and_then(|()| {
                remove_verified_recovery_sidecar(
                    &rollback,
                    ObservedFileRole::Replacement,
                    active.record.predecessor_digest,
                    active.record.successor_digest,
                )
            })
            .map_err(|error| recovery_blocked(target, &active, error.to_string()))?;
            retire_settled_windows_recovery(target)
        }
        RecoveryReconciliation::Resolve(PublicationResolution::Committed) => {
            unreachable!("Prepared recovery is never inferred committed")
        }
        RecoveryReconciliation::AcceptResolved => {
            let evidence_is_valid = match active.record.state {
                RecoveryRecordState::Resolved(PublicationResolution::Committed) => {
                    exact_or_missing(backup_role, ObservedFileRole::Replaced)
                        && matches!(rollback_role, ObservedFileRole::Missing)
                }
                RecoveryRecordState::Resolved(PublicationResolution::Aborted) => {
                    exact_or_missing(backup_role, ObservedFileRole::Replaced)
                        && exact_or_missing(rollback_role, ObservedFileRole::Replacement)
                }
                RecoveryRecordState::Prepared => false,
            };
            if !evidence_is_valid {
                return Err(recovery_blocked(
                    target,
                    &active,
                    format!(
                        "resolved recovery has foreign backup evidence ({backup_role:?}, {rollback_role:?})"
                    ),
                ));
            }
            remove_verified_recovery_sidecar(
                &backup,
                ObservedFileRole::Replaced,
                active.record.predecessor_digest,
                active.record.successor_digest,
            )
            .and_then(|()| {
                remove_verified_recovery_sidecar(
                    &rollback,
                    ObservedFileRole::Replacement,
                    active.record.predecessor_digest,
                    active.record.successor_digest,
                )
            })
            .map_err(|error| recovery_blocked(target, &active, error.to_string()))?;
            retire_settled_windows_recovery(target)
        }
    }
}

#[cfg(windows)]
fn remove_windows_retirement_tombstone(
    target: &Path,
    path: &Path,
    expected: Option<&RecoveryRecord>,
) -> Result<(), PublicationFailure> {
    let record = match read_windows_recovery_record(path) {
        Ok(Some(record)) => record,
        Ok(None) => return Ok(()),
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "retirement tombstone '{}' is foreign or corrupted: {error}",
                    path.display()
                ),
                recovery_paths: vec![path.to_path_buf()],
            });
        }
    };
    let binding = target_binding_digest(target).map_err(PublicationFailure::Safe)?;
    if record.target_binding != binding || expected.is_some_and(|expected| expected != &record) {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "retirement tombstone '{}' is not owned by the recovery generation being retired",
                path.display()
            ),
            recovery_paths: vec![path.to_path_buf()],
        });
    }
    let target_role = observe_file_role(target, record.predecessor_digest, record.successor_digest);
    let (backup, backup_role, rollback, rollback_role) = recovery_sidecar_roles(target, &record);
    if recovery_reconciliation(record.state, target_role, record.before_present)
        != RecoveryReconciliation::AcceptResolved
        || !matches!(backup_role, ObservedFileRole::Missing)
        || !matches!(rollback_role, ObservedFileRole::Missing)
    {
        return Err(PublicationFailure::Uncertain {
            message: format!(
                "retirement tombstone '{}' is not settled against the exact canonical endpoint ({target_role:?}, {backup_role:?}, {rollback_role:?})",
                path.display()
            ),
            recovery_paths: recovery_paths(&[path, target, &backup, &rollback]),
        });
    }
    match std::fs::remove_file(path) {
        Ok(()) => sync_parent_of(path).map_err(PublicationFailure::Safe),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(PublicationFailure::Safe(error)),
    }
}

#[cfg(windows)]
fn retire_windows_recovery_slot(
    target: &Path,
    slot: &Path,
    expected: &RecoveryRecord,
) -> Result<(), PublicationFailure> {
    let tombstone = recovery_retired_path(slot);
    if matches!(slot.try_exists(), Ok(false)) {
        return remove_windows_retirement_tombstone(target, &tombstone, Some(expected));
    }
    match read_windows_recovery_record(slot) {
        Ok(Some(record)) if &record == expected => {}
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "recovery slot '{}' changed before authenticated retirement: {observation:?}",
                    slot.display()
                ),
                recovery_paths: recovery_paths(&[slot, &tombstone]),
            });
        }
    }
    remove_windows_retirement_tombstone(target, &tombstone, None)?;

    let raw_result = move_no_replace_raw(slot, &tombstone);
    match slot.try_exists() {
        Ok(false) => {}
        Ok(true) => {
            return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
                || {
                    io::Error::other(format!(
                        "recovery retirement move reported success without consuming '{}'",
                        slot.display()
                    ))
                },
            )));
        }
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "could not verify removal of recovery slot '{}': {error}",
                    slot.display()
                ),
                recovery_paths: recovery_paths(&[slot, &tombstone]),
            });
        }
    }

    match read_windows_recovery_record(&tombstone) {
        Ok(Some(record)) if &record == expected => {}
        Ok(None) => {
            return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
                || {
                    io::Error::other(format!(
                        "resolved recovery slot '{}' disappeared during retirement",
                        slot.display()
                    ))
                },
            )));
        }
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "resolved recovery slot changed while being retired: {observation:?}"
                ),
                recovery_paths: recovery_paths(&[&tombstone]),
            });
        }
    }

    remove_windows_retirement_tombstone(target, &tombstone, Some(expected))
}

#[cfg(windows)]
fn retire_settled_windows_recovery(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        for slot in recovery_slot_paths(target) {
            remove_windows_retirement_tombstone(target, &recovery_retired_path(&slot), None)?;
        }
        return Ok(());
    };
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if recovery_reconciliation(
        active.record.state,
        target_role,
        active.record.before_present,
    ) != RecoveryReconciliation::AcceptResolved
    {
        return Err(recovery_blocked(
            target,
            &active,
            "Prepared or unsettled recovery evidence cannot be retired",
        ));
    }
    let (_backup, backup_role, _rollback, rollback_role) =
        recovery_sidecar_roles(target, &active.record);
    if !matches!(backup_role, ObservedFileRole::Missing)
        || !matches!(rollback_role, ObservedFileRole::Missing)
    {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "resolved recovery cannot be retired before sidecar cleanup ({backup_role:?}, {rollback_role:?})"
            ),
        ));
    }

    let slots = recovery_slot_paths(target);
    let inactive_index = 1 - active.slot_index;
    match read_windows_recovery_record(&slots[inactive_index]) {
        Ok(Some(inactive)) => {
            retire_windows_recovery_slot(target, &slots[inactive_index], &inactive)?;
            sync_parent_of(&slots[inactive_index]).map_err(PublicationFailure::Safe)?;
        }
        Ok(None) => remove_windows_retirement_tombstone(
            target,
            &recovery_retired_path(&slots[inactive_index]),
            None,
        )?,
        Err(error) => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "inactive recovery slot '{}' is foreign or corrupted: {error}",
                    slots[inactive_index].display()
                ),
                recovery_paths: vec![slots[inactive_index].clone()],
            });
        }
    }

    let verified =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "newest recovery generation disappeared before final retirement".to_owned(),
            recovery_paths: recovery_paths(&[target, &slots[active.slot_index]]),
        })?;
    if verified.path != active.path || verified.record != active.record {
        return Err(recovery_blocked(
            target,
            &verified,
            "newest recovery generation changed during retirement",
        ));
    }
    retire_windows_recovery_slot(target, &slots[active.slot_index], &active.record)?;
    sync_parent_of(&slots[active.slot_index]).map_err(PublicationFailure::Safe)
}

#[cfg(windows)]
fn publish_retaining_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
) -> Result<Option<PathBuf>, PublicationFailure> {
    let predecessor_digest = current_digest(target).map_err(PublicationFailure::Safe)?;
    let Some(predecessor_digest) = predecessor_digest else {
        move_no_replace_checked(staged, target, staged_digest)?;
        return Ok(None);
    };

    let recovery =
        install_windows_recovery_bundle(target, staged, predecessor_digest, staged_digest)?;
    let displaced = recovery_backup_path(target, recovery.record.backup_id);
    match replace_file_checked(
        target,
        staged,
        &displaced,
        predecessor_digest,
        staged_digest,
    ) {
        Ok(()) => match current_digest(&displaced) {
            Ok(Some(digest)) if digest == predecessor_digest => Ok(Some(displaced)),
            observation => Err(recovery_blocked(
                target,
                &recovery,
                format!(
                    "ReplaceFileW successor is visible, but its actual backup does not match the bundled predecessor: {observation:?}"
                ),
            )),
        },
        Err(PublicationFailure::Safe(error)) => {
            resolve_windows_recovery_slot(
                target,
                staged_digest,
                PublicationResolution::Aborted,
            )
            .map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW was rolled back, but the Prepared recovery generation could not be durably aborted",
                    failure,
                    &[target, staged, &displaced, &recovery.path],
                )
            })?;
            Err(PublicationFailure::Safe(error))
        }
        Err(PublicationFailure::Uncertain {
            message,
            recovery_paths: paths,
        }) => Err(PublicationFailure::Uncertain {
            message,
            recovery_paths: merge_recovery_paths(paths, [recovery.path, displaced]),
        }),
    }
}

#[cfg(windows)]
fn rollback_retaining_successor(
    target: &Path,
    predecessor: &Path,
) -> Result<PathBuf, PublicationFailure> {
    let successor_digest = current_digest(target)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!("rollback target '{}' no longer exists", target.display()),
            ))
        })?;
    let predecessor_digest = current_digest(predecessor)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "rollback predecessor '{}' no longer exists",
                    predecessor.display()
                ),
            ))
        })?;
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "rollback has no active recovery generation".to_owned(),
            recovery_paths: vec![target.to_path_buf(), predecessor.to_path_buf()],
        })?;
    let recorded_predecessor = recovery_backup_path(target, active.record.backup_id);
    if predecessor != recorded_predecessor {
        return Err(recovery_blocked(
            target,
            &active,
            format!(
                "rollback predecessor '{}' is not the active transaction backup '{}'",
                predecessor.display(),
                recorded_predecessor.display()
            ),
        ));
    }
    let successor = recovery_rollback_path(target, active.record.backup_id);
    if !matches!(successor.try_exists(), Ok(false)) {
        return Err(recovery_blocked(
            target,
            &active,
            "rollback successor path already exists",
        ));
    }
    replace_file_checked(
        target,
        predecessor,
        &successor,
        successor_digest,
        predecessor_digest,
    )?;
    Ok(successor)
}

#[cfg(windows)]
fn replace_file_checked(
    replaced: &Path,
    replacement: &Path,
    backup: &Path,
    replaced_digest: [u8; 32],
    replacement_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use windows_sys::Win32::Storage::FileSystem::ReplaceFileW;

    let replaced_w = wide(replaced);
    let replacement_w = wide(replacement);
    let backup_w = wide(backup);
    // REPLACEFILE_WRITE_THROUGH is explicitly unsupported. Durability is
    // established by synchronizing both staged bytes and the parent metadata
    // boundary around this call instead of relying on that inert flag.
    let result = unsafe {
        ReplaceFileW(
            replaced_w.as_ptr(),
            replacement_w.as_ptr(),
            backup_w.as_ptr(),
            0,
            std::ptr::null(),
            std::ptr::null(),
        )
    };
    let reported_success = result != 0;
    let call_error = if reported_success {
        io::Error::other(format!(
            "ReplaceFileW reported success without its verified postcondition for '{}'",
            replaced.display()
        ))
    } else {
        io::Error::last_os_error()
    };
    let observation = observe_replace(
        replaced,
        replacement,
        backup,
        replaced_digest,
        replacement_digest,
    );

    if reported_success && observation.success_is_complete() {
        return Ok(());
    }

    match replacement_failure_recovery(observation) {
        ReplaceFailureRecovery::InputIntact => Err(PublicationFailure::Safe(call_error)),
        ReplaceFailureRecovery::RestoreBackupIntoReplaced => {
            move_no_replace_checked(backup, replaced, replaced_digest).map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW left the replaced path missing and its backup could not be restored",
                    failure,
                    &[replaced, replacement, backup],
                )
            })?;
            let restored = observe_replace(
                replaced,
                replacement,
                backup,
                replaced_digest,
                replacement_digest,
            );
            if restored.input_is_intact() {
                Err(PublicationFailure::Safe(call_error))
            } else {
                Err(PublicationFailure::Uncertain {
                    message: format!(
                        "ReplaceFileW backup restoration produced an unexpected state: {restored:?}"
                    ),
                    recovery_paths: recovery_paths(&[replaced, replacement, backup]),
                })
            }
        }
        ReplaceFailureRecovery::RestoreCompletedReplacement => {
            move_no_replace_checked(replaced, replacement, replacement_digest).map_err(
                |failure| {
                    recovery_step_failure(
                        "ReplaceFileW failed after completing replacement and the successor could not be returned to staging",
                        failure,
                        &[replaced, replacement, backup],
                    )
                },
            )?;
            move_no_replace_checked(backup, replaced, replaced_digest).map_err(|failure| {
                recovery_step_failure(
                    "ReplaceFileW completed despite reporting failure and its predecessor backup could not be restored",
                    failure,
                    &[replaced, replacement, backup],
                )
            })?;
            let restored = observe_replace(
                replaced,
                replacement,
                backup,
                replaced_digest,
                replacement_digest,
            );
            if restored.input_is_intact() {
                Err(PublicationFailure::Safe(call_error))
            } else {
                Err(PublicationFailure::Uncertain {
                    message: format!(
                        "ReplaceFileW completed despite reporting failure and rollback produced an unexpected state: {restored:?}"
                    ),
                    recovery_paths: recovery_paths(&[replaced, replacement, backup]),
                })
            }
        }
        ReplaceFailureRecovery::Uncertain => Err(PublicationFailure::Uncertain {
            message: format!(
                "ReplaceFileW for '{}' returned an unverified state after {call_error}: {observation:?}",
                replaced.display()
            ),
            recovery_paths: recovery_paths(&[replaced, replacement, backup]),
        }),
    }
}

#[cfg(windows)]
fn move_no_replace_raw(from: &Path, to: &Path) -> io::Result<()> {
    use windows_sys::Win32::Storage::FileSystem::{MOVEFILE_WRITE_THROUGH, MoveFileExW};
    let from = wide(from);
    let to = wide(to);
    let moved = unsafe { MoveFileExW(from.as_ptr(), to.as_ptr(), MOVEFILE_WRITE_THROUGH) };
    if moved == 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn wide(path: &Path) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt as _;
    path.as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(unix)]
fn unix_recovery_blocked(
    target: &Path,
    active: &ActiveRecoveryRecord,
    message: impl Into<String>,
) -> PublicationFailure {
    let backup = recovery_backup_path(target, active.record.backup_id);
    PublicationFailure::Uncertain {
        message: message.into(),
        recovery_paths: recovery_paths(&[target, &active.path, &backup]),
    }
}

#[cfg(unix)]
fn retire_unix_recovery_record_exact(
    active: &ActiveRecoveryRecord,
) -> Result<(), PublicationFailure> {
    match read_windows_recovery_record(&active.path) {
        Ok(Some(record)) if record == active.record => {}
        observation => {
            return Err(PublicationFailure::Uncertain {
                message: format!(
                    "native recovery record changed before authenticated retirement: {observation:?}"
                ),
                recovery_paths: recovery_paths(&[&active.path]),
            });
        }
    }
    match std::fs::remove_file(&active.path) {
        Ok(()) => sync_parent_of(&active.path).map_err(PublicationFailure::Safe),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            Err(PublicationFailure::Uncertain {
                message: "native recovery record disappeared during authenticated retirement"
                    .to_owned(),
                recovery_paths: Vec::new(),
            })
        }
        Err(error) => Err(PublicationFailure::Safe(error)),
    }
}

#[cfg(unix)]
fn unix_aborted_endpoint_is_settled(role: ObservedFileRole, _before_present: bool) -> bool {
    // The Aborted marker is appended only after the transaction verified that
    // its successor was returned to staging (or removed from a new target).
    // Any later readable/missing canonical state belongs to the outside world
    // and must be preserved while stale RSpice evidence is retired.
    !matches!(role, ObservedFileRole::Unreadable)
}

#[cfg(unix)]
fn retire_settled_unix_recovery(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return Ok(());
    };
    for (slot_index, slot) in recovery_slot_paths(target).iter().enumerate() {
        if slot_index != active.slot_index && !matches!(slot.try_exists(), Ok(false)) {
            return Err(unix_recovery_blocked(
                target,
                &active,
                format!(
                    "inactive native recovery slot '{}' is still present",
                    slot.display()
                ),
            ));
        }
    }

    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let endpoint_is_settled = match active.record.state {
        RecoveryRecordState::Resolved(PublicationResolution::Committed) => matches!(
            target_role,
            ObservedFileRole::Replacement | ObservedFileRole::Both
        ),
        RecoveryRecordState::Resolved(PublicationResolution::Aborted) => {
            unix_aborted_endpoint_is_settled(target_role, active.record.before_present)
        }
        RecoveryRecordState::Prepared => false,
    };
    let backup = recovery_backup_path(target, active.record.backup_id);
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if !endpoint_is_settled || !matches!(backup_role, ObservedFileRole::Missing) {
        return Err(unix_recovery_blocked(
            target,
            &active,
            format!(
                "native recovery is not settled against its exact endpoint ({:?}, {target_role:?}, {backup_role:?})",
                active.record.state
            ),
        ));
    }
    retire_unix_recovery_record_exact(&active)
}

#[cfg(unix)]
fn resolve_unix_recovery_slot(
    target: &Path,
    displaced: Option<&Path>,
    successor_digest: [u8; 32],
    resolution: PublicationResolution,
) -> Result<(), PublicationFailure> {
    let active =
        select_active_windows_recovery(target)?.ok_or_else(|| PublicationFailure::Uncertain {
            message: "native publication has no durable recovery generation".to_owned(),
            recovery_paths: recovery_paths(&[target]),
        })?;
    if active.record.successor_digest != successor_digest {
        return Err(unix_recovery_blocked(
            target,
            &active,
            "native recovery successor does not match the transaction being resolved",
        ));
    }
    let backup = recovery_backup_path(target, active.record.backup_id);
    if displaced.is_some_and(|path| path != backup) {
        return Err(unix_recovery_blocked(
            target,
            &active,
            "displaced predecessor is not the active native recovery sidecar",
        ));
    }
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let exact_endpoint = match resolution {
        PublicationResolution::Committed => {
            matches!(
                target_role,
                ObservedFileRole::Replacement | ObservedFileRole::Both
            ) && if active.record.before_present {
                matches!(
                    backup_role,
                    ObservedFileRole::Replaced | ObservedFileRole::Both
                )
            } else {
                matches!(backup_role, ObservedFileRole::Missing)
            }
        }
        PublicationResolution::Aborted => {
            if active.record.before_present {
                !matches!(target_role, ObservedFileRole::Unreadable)
                    && matches!(
                        backup_role,
                        ObservedFileRole::Replacement
                            | ObservedFileRole::Both
                            | ObservedFileRole::Missing
                    )
            } else {
                !matches!(target_role, ObservedFileRole::Unreadable)
                    && matches!(
                        backup_role,
                        ObservedFileRole::Replacement | ObservedFileRole::Missing
                    )
            }
        }
    };
    if !exact_endpoint {
        return Err(unix_recovery_blocked(
            target,
            &active,
            format!(
                "native recovery resolution {resolution:?} does not match exact endpoint roles {target_role:?}/{backup_role:?}"
            ),
        ));
    }
    append_windows_recovery_resolution(&active, resolution)
}

#[cfg(unix)]
fn settle_prepared_unix_abort(
    target: &Path,
    active: &ActiveRecoveryRecord,
) -> Result<(), PublicationFailure> {
    append_windows_recovery_resolution(active, PublicationResolution::Aborted)?;
    let backup = recovery_backup_path(target, active.record.backup_id);
    remove_verified_recovery_sidecar(
        &backup,
        ObservedFileRole::Replacement,
        active.record.predecessor_digest,
        active.record.successor_digest,
    )
    .map_err(|error| unix_recovery_blocked(target, active, error.to_string()))?;
    retire_settled_unix_recovery(target)
}

#[cfg(unix)]
fn reconcile_unix_recovery_slots(target: &Path) -> Result<(), PublicationFailure> {
    let Some(active) = select_active_windows_recovery(target)? else {
        return Ok(());
    };
    if !matches!(active.record.state, RecoveryRecordState::Prepared) {
        let backup = recovery_backup_path(target, active.record.backup_id);
        let target_role = observe_file_role(
            target,
            active.record.predecessor_digest,
            active.record.successor_digest,
        );
        let backup_role = observe_file_role(
            &backup,
            active.record.predecessor_digest,
            active.record.successor_digest,
        );
        let (endpoint_is_settled, expected_role) = match active.record.state {
            RecoveryRecordState::Resolved(PublicationResolution::Committed) => (
                matches!(
                    target_role,
                    ObservedFileRole::Replacement | ObservedFileRole::Both
                ),
                ObservedFileRole::Replaced,
            ),
            RecoveryRecordState::Resolved(PublicationResolution::Aborted) => (
                unix_aborted_endpoint_is_settled(target_role, active.record.before_present),
                ObservedFileRole::Replacement,
            ),
            RecoveryRecordState::Prepared => unreachable!(),
        };
        if !endpoint_is_settled
            || (!matches!(backup_role, ObservedFileRole::Missing)
                && backup_role != expected_role
                && !matches!(backup_role, ObservedFileRole::Both))
        {
            return Err(unix_recovery_blocked(
                target,
                &active,
                format!(
                    "resolved native recovery cannot be cleaned against endpoint roles {target_role:?}/{backup_role:?}"
                ),
            ));
        }
        remove_verified_recovery_sidecar(
            &backup,
            expected_role,
            active.record.predecessor_digest,
            active.record.successor_digest,
        )
        .map_err(|error| unix_recovery_blocked(target, &active, error.to_string()))?;
        return retire_settled_unix_recovery(target);
    }

    let backup = recovery_backup_path(target, active.record.backup_id);
    let target_role = observe_file_role(
        target,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    let backup_role = observe_file_role(
        &backup,
        active.record.predecessor_digest,
        active.record.successor_digest,
    );
    if active.record.before_present {
        if matches!(
            target_role,
            ObservedFileRole::Replaced | ObservedFileRole::Both
        ) && matches!(
            backup_role,
            ObservedFileRole::Replacement | ObservedFileRole::Both | ObservedFileRole::Missing
        ) {
            return settle_prepared_unix_abort(target, &active);
        }
        if matches!(target_role, ObservedFileRole::Replacement)
            && matches!(backup_role, ObservedFileRole::Replaced)
        {
            exchange_paths_checked(
                target,
                &backup,
                active.record.successor_digest,
                active.record.predecessor_digest,
            )?;
            sync_parent_of(target).map_err(|error| {
                unix_recovery_blocked(
                    target,
                    &active,
                    format!("restart rollback could not be synchronized: {error}"),
                )
            })?;
            return settle_prepared_unix_abort(target, &active);
        }
    } else {
        if matches!(target_role, ObservedFileRole::Missing)
            && matches!(
                backup_role,
                ObservedFileRole::Replacement | ObservedFileRole::Missing
            )
        {
            return settle_prepared_unix_abort(target, &active);
        }
        if matches!(target_role, ObservedFileRole::Replacement)
            && matches!(backup_role, ObservedFileRole::Missing)
        {
            move_no_replace_checked(target, &backup, active.record.successor_digest)?;
            sync_parent_of(target).map_err(|error| {
                unix_recovery_blocked(
                    target,
                    &active,
                    format!("new-target restart rollback could not be synchronized: {error}"),
                )
            })?;
            return settle_prepared_unix_abort(target, &active);
        }
    }
    Err(unix_recovery_blocked(
        target,
        &active,
        format!(
            "Prepared native recovery cannot be reconciled with exact endpoint roles {target_role:?}/{backup_role:?}; no pathname was changed"
        ),
    ))
}

#[cfg(unix)]
fn prepare_unix_recovery(
    staged: &Path,
    target: &Path,
    predecessor_digest: Option<[u8; 32]>,
    successor_digest: [u8; 32],
) -> Result<(ActiveRecoveryRecord, PathBuf), PublicationFailure> {
    let backup_id = Uuid::new_v4();
    let backup = recovery_backup_path(target, backup_id);
    move_no_replace_checked(staged, &backup, successor_digest)?;
    sync_parent_of(&backup).map_err(|error| PublicationFailure::Uncertain {
        message: format!(
            "native successor sidecar was prepared but its directory entry could not be synchronized: {error}"
        ),
        recovery_paths: recovery_paths(&[staged, &backup, target]),
    })?;
    match install_recovery_bundle(
        target,
        &backup,
        predecessor_digest,
        successor_digest,
        backup_id,
    ) {
        Ok(active) => Ok((active, backup)),
        Err(PublicationFailure::Safe(error)) => {
            move_no_replace_checked(&backup, staged, successor_digest).map_err(|failure| {
                recovery_step_failure(
                    "native recovery installation failed and staging could not be restored",
                    failure,
                    &[staged, &backup, target],
                )
            })?;
            sync_parent_of(staged).map_err(|sync_error| PublicationFailure::Uncertain {
                message: format!(
                    "native recovery installation failed ({error}) and restored staging could not be synchronized: {sync_error}"
                ),
                recovery_paths: recovery_paths(&[staged, target]),
            })?;
            Err(PublicationFailure::Safe(error))
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => Err(error),
    }
}

#[cfg(unix)]
fn abort_unpublished_unix_recovery(
    staged: &Path,
    target: &Path,
    active: &ActiveRecoveryRecord,
    backup: &Path,
    successor_digest: [u8; 32],
    publication_error: io::Error,
) -> Result<Option<PathBuf>, PublicationFailure> {
    move_no_replace_checked(backup, staged, successor_digest).map_err(|failure| {
        recovery_step_failure(
            "native publication failed before mutation and staging could not be restored",
            failure,
            &[staged, backup, target, &active.path],
        )
    })?;
    sync_parent_of(staged).map_err(|error| PublicationFailure::Uncertain {
        message: format!(
            "native publication failed ({publication_error}) and restored staging could not be synchronized: {error}"
        ),
        recovery_paths: recovery_paths(&[staged, target, &active.path]),
    })?;
    append_windows_recovery_resolution(active, PublicationResolution::Aborted)?;
    retire_settled_unix_recovery(target)?;
    Err(PublicationFailure::Safe(publication_error))
}

#[cfg(unix)]
fn publish_retaining_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
) -> Result<Option<PathBuf>, PublicationFailure> {
    let predecessor_digest = current_digest(target).map_err(PublicationFailure::Safe)?;
    let (active, backup) =
        prepare_unix_recovery(staged, target, predecessor_digest, staged_digest)?;
    let publication = match predecessor_digest {
        Some(predecessor_digest) => {
            exchange_paths_capturing_predecessor(&backup, target, staged_digest, predecessor_digest)
                .map(|()| Some(backup.clone()))
        }
        None => move_no_replace_checked(&backup, target, staged_digest).map(|()| None),
    };
    match publication {
        Ok(displaced) => Ok(displaced),
        Err(PublicationFailure::Safe(error)) => {
            abort_unpublished_unix_recovery(staged, target, &active, &backup, staged_digest, error)
        }
        Err(error @ PublicationFailure::Uncertain { .. }) => Err(error),
    }
}

#[cfg(unix)]
fn rollback_retaining_successor(
    target: &Path,
    predecessor: &Path,
) -> Result<PathBuf, PublicationFailure> {
    let successor_digest = current_digest(target)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!("rollback target '{}' no longer exists", target.display()),
            ))
        })?;
    let predecessor_digest = current_digest(predecessor)
        .map_err(PublicationFailure::Safe)?
        .ok_or_else(|| {
            PublicationFailure::Safe(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "rollback predecessor '{}' no longer exists",
                    predecessor.display()
                ),
            ))
        })?;
    exchange_paths_checked(target, predecessor, successor_digest, predecessor_digest)?;
    Ok(predecessor.to_path_buf())
}

#[cfg(unix)]
fn exchange_paths_capturing_predecessor(
    staged: &Path,
    target: &Path,
    staged_digest: [u8; 32],
    observed_predecessor_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    let raw_result =
        renameat_with(CWD, staged, CWD, target, RenameFlags::EXCHANGE).map_err(io::Error::from);
    let observed_staged = current_digest(staged);
    let observed_target = current_digest(target);

    // The canonical endpoint now contains the staged successor and the swap
    // pathname captured some complete predecessor. It may differ from the
    // earlier observation because a non-cooperating writer won the final race;
    // the caller hashes these captured bytes and rolls them back as a conflict.
    if matches!(observed_target, Ok(Some(digest)) if digest == staged_digest)
        && matches!(observed_staged, Ok(Some(_)))
    {
        return Ok(());
    }

    // The staged inode is still named at its input path, so the exchange did
    // not consume it. Any target change is external and must remain untouched.
    if matches!(observed_staged, Ok(Some(digest)) if digest == staged_digest) {
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(format!(
                    "atomic exchange of '{}' and '{}' reported success without consuming staging (previous target digest {observed_predecessor_digest:02x?})",
                    staged.display(),
                    target.display()
                ))
            },
        )));
    }

    let raw_message = match raw_result {
        Ok(()) => "the platform call reported success".to_owned(),
        Err(error) => error.to_string(),
    };
    Err(PublicationFailure::Uncertain {
        message: format!(
            "atomic predecessor capture for '{}' has an unverified postcondition ({raw_message}); swap digest: {observed_staged:?}; target digest: {observed_target:?}",
            target.display()
        ),
        recovery_paths: recovery_paths(&[staged, target]),
    })
}

#[cfg(unix)]
fn exchange_paths_checked(
    left: &Path,
    right: &Path,
    left_digest: [u8; 32],
    right_digest: [u8; 32],
) -> Result<(), PublicationFailure> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    let raw_result =
        renameat_with(CWD, left, CWD, right, RenameFlags::EXCHANGE).map_err(io::Error::from);
    let observed_left = current_digest(left);
    let observed_right = current_digest(right);
    if matches!(observed_left, Ok(Some(digest)) if digest == right_digest)
        && matches!(observed_right, Ok(Some(digest)) if digest == left_digest)
    {
        return Ok(());
    }
    if matches!(observed_left, Ok(Some(digest)) if digest == left_digest)
        && matches!(observed_right, Ok(Some(digest)) if digest == right_digest)
    {
        return Err(PublicationFailure::Safe(raw_result.err().unwrap_or_else(
            || {
                io::Error::other(format!(
                    "atomic exchange of '{}' and '{}' reported success without changing either path",
                    left.display(),
                    right.display()
                ))
            },
        )));
    }

    let raw_message = match raw_result {
        Ok(()) => "the platform call reported success".to_owned(),
        Err(error) => error.to_string(),
    };
    Err(PublicationFailure::Uncertain {
        message: format!(
            "atomic exchange of '{}' and '{}' has an unverified postcondition ({raw_message}); left digest: {observed_left:?}; right digest: {observed_right:?}",
            left.display(),
            right.display()
        ),
        recovery_paths: recovery_paths(&[left, right]),
    })
}

#[cfg(unix)]
fn move_no_replace_raw(from: &Path, to: &Path) -> io::Result<()> {
    use rustix::fs::{CWD, RenameFlags, renameat_with};

    renameat_with(CWD, from, CWD, to, RenameFlags::NOREPLACE).map_err(io::Error::from)
}

#[cfg(unix)]
fn sync_parent_directory(parent: &Path) -> io::Result<()> {
    File::open(parent)?.sync_all()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_staging_preserves_target_and_removes_unique_temp() {
        let root = unique_temp_dir("failed-stage");
        let target = root.join("state.json");
        std::fs::write(&target, b"durable predecessor").expect("write predecessor");

        let result = atomic_write_with::<io::Error>(&target, |file| {
            file.write_all(b"partial successor")?;
            Err(io::Error::other("injected serializer failure"))
        });

        assert!(result.is_err());
        assert_eq!(
            std::fs::read(&target).expect("read predecessor"),
            b"durable predecessor"
        );
        assert_only_target_and_lease(&root, &target);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn successful_publication_replaces_target_without_fixed_temp_names() {
        let root = unique_temp_dir("success");
        let target = root.join("state.json");
        std::fs::write(&target, b"old").expect("write predecessor");

        atomic_write_bytes(&target, b"new durable bytes").expect("publish successor");

        assert_eq!(
            std::fs::read(&target).expect("read successor"),
            b"new durable bytes"
        );
        assert_only_target_and_lease(&root, &target);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn compare_exchange_rejects_external_change_without_touching_target() {
        let root = unique_temp_dir("cas-conflict");
        let target = root.join("state.json");
        std::fs::write(&target, b"accepted").expect("write accepted bytes");
        let accepted: [u8; 32] = Sha256::digest(b"accepted").into();
        std::fs::write(&target, b"external revision").expect("write external revision");

        let result = compare_exchange_bytes(
            &target,
            ExpectedContent::Digest(accepted),
            b"local successor",
        );

        assert!(matches!(result, Err(CompareExchangeError::Conflict { .. })));
        assert_eq!(
            std::fs::read(&target).expect("read target"),
            b"external revision"
        );
        assert_only_target_and_lease(&root, &target);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn compare_exchange_publishes_exact_successor_and_releases_predecessor_evidence() {
        let root = unique_temp_dir("cas-success");
        let target = root.join("state.json");
        std::fs::write(&target, b"accepted").expect("write accepted bytes");
        let accepted: [u8; 32] = Sha256::digest(b"accepted").into();

        compare_exchange_bytes(
            &target,
            ExpectedContent::Digest(accepted),
            b"verified successor",
        )
        .expect("publish exact successor");

        assert_eq!(
            std::fs::read(&target).expect("read successor"),
            b"verified successor"
        );
        #[cfg(windows)]
        assert_no_windows_recovery_artifacts(&target);
        assert_only_target_and_lease(&root, &target);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn replace_failure_recovery_covers_every_documented_postcondition() {
        use ObservedFileRole::{Missing, Other, Replaced, Replacement, Unreadable};
        use ReplaceFailureRecovery::{
            InputIntact, RestoreBackupIntoReplaced, RestoreCompletedReplacement, Uncertain,
        };

        let cases = [
            (
                "1175 unable to open replacement keeps both named inputs",
                ReplaceObservation {
                    replaced: Replaced,
                    replacement: Replacement,
                    backup: Missing,
                },
                InputIntact,
            ),
            (
                "1176 failure with a requested backup keeps both named inputs",
                ReplaceObservation {
                    replaced: Replaced,
                    replacement: Replacement,
                    backup: Missing,
                },
                InputIntact,
            ),
            (
                "1176 failure without backup evidence is indeterminate",
                ReplaceObservation {
                    replaced: Missing,
                    replacement: Replacement,
                    backup: Missing,
                },
                Uncertain,
            ),
            (
                "1177 partial merge with backup can restore the replaced path",
                ReplaceObservation {
                    replaced: Missing,
                    replacement: Replacement,
                    backup: Replaced,
                },
                RestoreBackupIntoReplaced,
            ),
            (
                "1177 partial merge without backup evidence is indeterminate",
                ReplaceObservation {
                    replaced: Missing,
                    replacement: Replacement,
                    backup: Missing,
                },
                Uncertain,
            ),
            (
                "other documented errors leave both named inputs unchanged",
                ReplaceObservation {
                    replaced: Replaced,
                    replacement: Replacement,
                    backup: Missing,
                },
                InputIntact,
            ),
            (
                "failure reported after a complete replacement is rolled back",
                ReplaceObservation {
                    replaced: Replacement,
                    replacement: Missing,
                    backup: Replaced,
                },
                RestoreCompletedReplacement,
            ),
            (
                "unreadable or foreign bytes are never guessed away",
                ReplaceObservation {
                    replaced: Unreadable,
                    replacement: Other,
                    backup: Replaced,
                },
                Uncertain,
            ),
        ];

        for (label, observation, expected) in cases {
            assert_eq!(
                replacement_failure_recovery(observation),
                expected,
                "{label}"
            );
        }
    }

    #[cfg(windows)]
    #[test]
    fn recovery_reconciliation_exhaustively_refuses_inference_and_resurrection() {
        use ObservedFileRole::{Both, Missing, Other, Replaced, Replacement, Unreadable};
        use PublicationResolution::{Aborted, Committed};
        use RecoveryReconciliation::{AcceptResolved, PreserveAndBlock, Resolve};
        use RecoveryRecordState::{Prepared, Resolved};

        let roles = [Missing, Replaced, Replacement, Both, Other, Unreadable];
        let cases = [
            (
                Prepared,
                [
                    PreserveAndBlock,
                    Resolve(Aborted),
                    PreserveAndBlock,
                    Resolve(Aborted),
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
                [
                    Resolve(Aborted),
                    Resolve(Aborted),
                    PreserveAndBlock,
                    Resolve(Aborted),
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
            ),
            (
                Resolved(Committed),
                [
                    PreserveAndBlock,
                    PreserveAndBlock,
                    AcceptResolved,
                    AcceptResolved,
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
                [
                    PreserveAndBlock,
                    PreserveAndBlock,
                    AcceptResolved,
                    AcceptResolved,
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
            ),
            (
                Resolved(Aborted),
                [
                    PreserveAndBlock,
                    AcceptResolved,
                    PreserveAndBlock,
                    AcceptResolved,
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
                [
                    AcceptResolved,
                    AcceptResolved,
                    PreserveAndBlock,
                    AcceptResolved,
                    PreserveAndBlock,
                    PreserveAndBlock,
                ],
            ),
        ];

        for (state, present_expectations, absent_expectations) in cases {
            for (index, role) in roles.into_iter().enumerate() {
                assert_eq!(
                    recovery_reconciliation(state, role, true),
                    present_expectations[index],
                    "present predecessor: {state:?}, {role:?}"
                );
                assert_eq!(
                    recovery_reconciliation(state, role, false),
                    absent_expectations[index],
                    "absent predecessor: {state:?}, {role:?}"
                );
            }
        }
    }

    #[cfg(windows)]
    #[test]
    fn recovery_resolution_torn_at_every_byte_remains_prepared() {
        let root = unique_temp_dir("torn-resolution");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let before = b"sealed predecessor";
        let after = b"sealed successor";
        std::fs::write(&target, before).expect("write predecessor");
        std::fs::write(&staged, after).expect("write successor");
        let before_digest = Sha256::digest(before).into();
        let after_digest = Sha256::digest(after).into();
        let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
            .expect("install bundle");
        let base = std::fs::read(&active.path).expect("read prepared bundle");
        let trailer = encode_recovery_resolution(&active.record, PublicationResolution::Committed);

        for prefix_len in 0..RECOVERY_RESOLUTION_LEN {
            let mut torn = base.clone();
            torn.extend_from_slice(&trailer[..prefix_len]);
            std::fs::write(&active.path, torn).expect("write torn resolution");
            let record = read_windows_recovery_record(&active.path)
                .expect("read torn bundle")
                .expect("bundle exists");
            assert_eq!(
                record.state,
                RecoveryRecordState::Prepared,
                "resolution prefix length {prefix_len}"
            );
        }

        let mut complete = base;
        complete.extend_from_slice(&trailer);
        std::fs::write(&active.path, complete).expect("write complete resolution");
        assert_eq!(
            read_windows_recovery_record(&active.path)
                .expect("read complete bundle")
                .expect("bundle exists")
                .state,
            RecoveryRecordState::Resolved(PublicationResolution::Committed)
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn recovery_slots_never_replace_an_occupied_or_invalid_inactive_slot() {
        let root = unique_temp_dir("slot-generations");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        std::fs::write(&target, b"before").expect("write predecessor");
        std::fs::write(&staged, b"after-one").expect("write successor one");
        let before_digest = Sha256::digest(b"before").into();
        let first_after = Sha256::digest(b"after-one").into();
        let first = install_windows_recovery_bundle(&target, &staged, before_digest, first_after)
            .expect("install first generation");
        assert_eq!((first.slot_index, first.record.generation), (0, 1));
        append_windows_recovery_resolution(&first, PublicationResolution::Aborted)
            .expect("abort first generation");

        std::fs::write(&staged, b"after-two").expect("write successor two");
        let second_after = Sha256::digest(b"after-two").into();
        let second = install_windows_recovery_bundle(&target, &staged, before_digest, second_after)
            .expect("install second generation");
        assert_eq!((second.slot_index, second.record.generation), (1, 2));
        append_windows_recovery_resolution(&second, PublicationResolution::Aborted)
            .expect("abort second generation");

        std::fs::write(&staged, b"after-three").expect("write successor three");
        let third_after = Sha256::digest(b"after-three").into();
        assert!(matches!(
            install_windows_recovery_bundle(&target, &staged, before_digest, third_after),
            Err(PublicationFailure::Safe(_))
        ));

        let inactive = recovery_slot_paths(&target)[0].clone();
        let foreign = b"torn inactive generation";
        std::fs::write(&inactive, foreign).expect("corrupt inactive slot");
        assert!(matches!(
            select_active_windows_recovery(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert!(matches!(
            install_windows_recovery_bundle(&target, &staged, before_digest, third_after),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&inactive).expect("read preserved foreign slot"),
            foreign
        );
        assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn recovery_slot_binding_ambiguity_and_generation_overflow_fail_closed() {
        fn reseal_header(bytes: &mut [u8]) {
            let checksum: [u8; 32] = Sha256::new()
                .chain_update(b"rspice-windows-recovery-header\0v1\0")
                .chain_update(&bytes[..RECOVERY_HEADER_PREFIX_LEN])
                .finalize()
                .into();
            bytes[RECOVERY_HEADER_PREFIX_LEN..RECOVERY_HEADER_LEN].copy_from_slice(&checksum);
        }

        let root = unique_temp_dir("slot-identity");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        std::fs::write(&target, b"before").expect("write predecessor");
        std::fs::write(&staged, b"after").expect("write successor");
        let before_digest = Sha256::digest(b"before").into();
        let after_digest = Sha256::digest(b"after").into();
        let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
            .expect("install generation");
        let inactive = recovery_slot_paths(&target)[1].clone();
        let original = std::fs::read(&active.path).expect("read active bundle");

        let mut wrong_binding = original.clone();
        wrong_binding[56] ^= 0x80;
        reseal_header(&mut wrong_binding);
        std::fs::write(&inactive, &wrong_binding).expect("write wrong-binding inactive slot");
        assert!(matches!(
            select_active_windows_recovery(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&inactive).expect("wrong-binding record is preserved"),
            wrong_binding
        );

        let mut ambiguous = original.clone();
        ambiguous[24..40].copy_from_slice(Uuid::new_v4().as_bytes());
        reseal_header(&mut ambiguous);
        std::fs::write(&inactive, ambiguous).expect("write equal-generation transaction");
        assert!(matches!(
            select_active_windows_recovery(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));

        std::fs::remove_file(&inactive).expect("remove ambiguous slot");
        let mut exhausted = original;
        exhausted[16..24].copy_from_slice(&u64::MAX.to_le_bytes());
        reseal_header(&mut exhausted);
        std::fs::write(&active.path, exhausted).expect("write maximum generation");
        assert!(matches!(
            install_windows_recovery_bundle(&target, &staged, before_digest, after_digest),
            Err(PublicationFailure::Safe(_))
        ));
        assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
        assert_eq!(
            select_active_windows_recovery(&target)
                .expect("maximum generation remains valid")
                .expect("active exists")
                .record
                .generation,
            u64::MAX
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn prepared_successor_and_missing_target_are_preserved_without_replay() {
        for (label, remove_target) in [("successor", false), ("missing", true)] {
            let root = unique_temp_dir(label);
            let target = root.join("state.json");
            let staged = root.join("staged.json");
            let before = b"before";
            let after = b"after";
            std::fs::write(&target, before).expect("write predecessor");
            std::fs::write(&staged, after).expect("write successor");
            install_windows_recovery_bundle(
                &target,
                &staged,
                Sha256::digest(before).into(),
                Sha256::digest(after).into(),
            )
            .expect("install prepared bundle");
            if remove_target {
                std::fs::remove_file(&target).expect("remove canonical target");
            } else {
                std::fs::write(&target, after).expect("expose uncommitted successor");
            }

            assert!(matches!(
                reconcile_windows_recovery_slots(&target),
                Err(PublicationFailure::Uncertain { .. })
            ));
            if remove_target {
                assert!(!target.exists(), "missing target must not be resurrected");
            } else {
                assert_eq!(
                    std::fs::read(&target).expect("read preserved successor"),
                    after,
                    "existing successor must not be overwritten"
                );
            }
            std::fs::remove_dir_all(root).expect("remove fixture");
        }
    }

    #[cfg(windows)]
    #[test]
    fn prepared_predecessor_is_aborted_and_raced_backup_is_never_discarded() {
        let root = unique_temp_dir("prepared-abort");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        std::fs::write(&target, b"before").expect("write predecessor");
        std::fs::write(&staged, b"after").expect("write successor");
        let before_digest = Sha256::digest(b"before").into();
        let after_digest = Sha256::digest(b"after").into();
        let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
            .expect("install prepared bundle");

        reconcile_windows_recovery_slots(&target).expect("settle exact predecessor as aborted");
        assert!(
            select_active_windows_recovery(&target)
                .expect("read retired slots")
                .is_none(),
            "settled abort must retire its recovery generation"
        );
        assert!(!active.path.exists());

        std::fs::write(&staged, b"after-two").expect("write next successor");
        let next_after = Sha256::digest(b"after-two").into();
        let next = install_windows_recovery_bundle(&target, &staged, before_digest, next_after)
            .expect("install next prepared generation");
        std::fs::write(&target, b"after-two").expect("expose successor");
        let backup = recovery_backup_path(&target, next.record.backup_id);
        std::fs::write(&backup, b"external raced backup").expect("write raced backup");
        assert!(matches!(
            resolve_windows_recovery_slot(&target, next_after, PublicationResolution::Committed),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&backup).expect("raced evidence preserved"),
            b"external raced backup"
        );
        assert_eq!(
            read_windows_recovery_record(&next.path)
                .expect("read prepared bundle")
                .expect("bundle exists")
                .state,
            RecoveryRecordState::Prepared
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn startup_settlement_retires_committed_records_and_retirement_is_idempotent() {
        let root = unique_temp_dir("startup-retirement");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        std::fs::write(&target, b"before").expect("write predecessor");
        std::fs::write(&staged, b"after").expect("write successor");
        let before_digest = Sha256::digest(b"before").into();
        let after_digest = Sha256::digest(b"after").into();
        let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
            .expect("install prepared bundle");
        let backup = recovery_backup_path(&target, active.record.backup_id);
        std::fs::write(&backup, b"before").expect("write displaced predecessor");
        std::fs::write(&target, b"after").expect("publish successor");
        resolve_windows_recovery_slot(&target, after_digest, PublicationResolution::Committed)
            .expect("resolve committed generation");

        reconcile_windows_recovery_slots(&target).expect("settle startup generation");
        assert_no_windows_recovery_artifacts(&target);
        retire_settled_windows_recovery(&target).expect("first idempotent retirement");
        retire_settled_windows_recovery(&target).expect("second idempotent retirement");
        reconcile_windows_recovery_slots(&target).expect("idempotent reconciliation");
        assert_eq!(std::fs::read(&target).expect("read successor"), b"after");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn retirement_never_deletes_or_replaces_foreign_tombstones() {
        let root = unique_temp_dir("foreign-retirement-tombstone");
        let target = root.join("state.json");
        std::fs::write(&target, b"before").expect("write target");
        let inactive_tombstone = recovery_retired_path(&recovery_slot_paths(&target)[0]);
        let foreign = b"unrelated user tombstone data";
        std::fs::write(&inactive_tombstone, foreign).expect("write foreign tombstone");

        assert!(matches!(
            retire_settled_windows_recovery(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&inactive_tombstone).expect("read preserved tombstone"),
            foreign
        );

        std::fs::remove_file(&inactive_tombstone).expect("remove first fixture tombstone");
        let staged = root.join("staged.json");
        std::fs::write(&staged, b"after").expect("write staged successor");
        let active = install_windows_recovery_bundle(
            &target,
            &staged,
            Sha256::digest(b"before").into(),
            Sha256::digest(b"after").into(),
        )
        .expect("install recovery generation");
        append_windows_recovery_resolution(&active, PublicationResolution::Aborted)
            .expect("resolve aborted generation");
        let active_tombstone = recovery_retired_path(&active.path);
        std::fs::write(&active_tombstone, foreign).expect("write active foreign tombstone");

        assert!(matches!(
            retire_settled_windows_recovery(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&active_tombstone).expect("read active foreign tombstone"),
            foreign
        );
        assert!(active.path.exists(), "owned slot must remain named");
        assert_eq!(std::fs::read(&target).expect("target unchanged"), b"before");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn external_edit_after_successful_cas_reconciles_without_stale_record() {
        let root = unique_temp_dir("post-save-external-edit");
        let target = root.join("state.json");
        std::fs::write(&target, b"accepted").expect("write accepted destination");
        compare_exchange_bytes(
            &target,
            ExpectedContent::Digest(Sha256::digest(b"accepted").into()),
            b"saved successor",
        )
        .expect("publish successor");
        assert_no_windows_recovery_artifacts(&target);

        std::fs::write(&target, b"legitimate external edit").expect("write external edit");
        reconcile_publication(&target).expect("reconcile after external edit");
        assert_eq!(
            std::fs::read(&target).expect("read external edit"),
            b"legitimate external edit"
        );
        assert_no_windows_recovery_artifacts(&target);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(windows)]
    #[test]
    fn committed_generation_never_overwrites_an_external_predecessor_restore() {
        let root = unique_temp_dir("committed-restore");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        std::fs::write(&target, b"before").expect("write predecessor");
        std::fs::write(&staged, b"after").expect("write successor");
        let before_digest = Sha256::digest(b"before").into();
        let after_digest = Sha256::digest(b"after").into();
        let active = install_windows_recovery_bundle(&target, &staged, before_digest, after_digest)
            .expect("install prepared bundle");
        let backup = recovery_backup_path(&target, active.record.backup_id);
        std::fs::write(&backup, b"before").expect("write exact displaced predecessor");
        std::fs::write(&target, b"after").expect("publish successor");
        resolve_windows_recovery_slot(&target, after_digest, PublicationResolution::Committed)
            .expect("commit exact endpoint");
        std::fs::remove_file(&backup).expect("simulate completed cleanup");
        std::fs::write(&target, b"before").expect("externally restore predecessor");

        assert!(matches!(
            reconcile_windows_recovery_slots(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(std::fs::read(&target).expect("read target"), b"before");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn expected_content_observation_never_grants_unconditional_overwrite() {
        let root = unique_temp_dir("observe-cas");
        let target = root.join("state.json");
        assert_eq!(
            observe_expected_content(&target).expect("observe missing destination"),
            ExpectedContent::Missing
        );
        std::fs::write(&target, b"accepted bytes").expect("write destination");
        assert_eq!(
            observe_expected_content(&target).expect("observe exact destination"),
            ExpectedContent::Digest(Sha256::digest(b"accepted bytes").into())
        );
        reconcile_publication(&target).expect("explicit reconciliation is idempotent");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn compare_exchange_requires_missing_destination_when_requested() {
        let root = unique_temp_dir("cas-missing");
        let target = root.join("state.json");
        std::fs::write(&target, b"someone won").expect("write competing file");

        let result = compare_exchange_bytes(&target, ExpectedContent::Missing, b"local successor");

        assert!(matches!(result, Err(CompareExchangeError::Conflict { .. })));
        assert_eq!(std::fs::read(&target).unwrap(), b"someone won");
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn destination_lease_serializes_writers_and_reuses_one_stable_inode() {
        let root = unique_temp_dir("lease");
        let target = root.join("state.json");
        let first = DestinationLease::acquire(&target).expect("acquire first lease");
        assert!(matches!(
            DestinationLease::acquire(&target),
            Err(CompareExchangeError::LeaseBusy(path)) if path == lease_path(&target)
        ));

        drop(first);
        let reacquired = DestinationLease::acquire(&target).expect("reacquire released lease");
        drop(reacquired);
        assert_eq!(
            std::fs::read_dir(&root).expect("read fixture").count(),
            1,
            "the permanent lease inode is bounded to one file per destination"
        );
        assert!(lease_path(&target).is_file());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn foreign_lease_collision_is_rejected_without_modification() {
        let root = unique_temp_dir("foreign-lease");
        let target = root.join("state.json");
        let lease = lease_path(&target);
        let foreign = b"unrelated user data at the collision path";
        std::fs::write(&lease, foreign).expect("write foreign lease collision");

        assert!(matches!(
            DestinationLease::acquire(&target),
            Err(CompareExchangeError::Io(_))
        ));
        assert_eq!(
            std::fs::read(&lease).expect("read foreign collision"),
            foreign
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn hard_linked_lease_collision_never_modifies_its_other_name() {
        let root = unique_temp_dir("hard-linked-lease");
        let target = root.join("state.json");
        let victim = root.join("important-user-data.bin");
        let victim_bytes = b"must survive lease acquisition exactly";
        std::fs::write(&victim, victim_bytes).expect("write victim");
        std::fs::hard_link(&victim, lease_path(&target)).expect("create hard-link collision");

        assert!(matches!(
            DestinationLease::acquire(&target),
            Err(CompareExchangeError::Io(_))
        ));
        assert_eq!(std::fs::read(&victim).expect("read victim"), victim_bytes);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_link_lease_collision_is_never_followed() {
        use std::os::unix::fs::symlink;

        let root = unique_temp_dir("symlink-lease");
        let target = root.join("state.json");
        let victim = root.join("important-user-data.bin");
        let victim_bytes = b"must survive symlink lease acquisition exactly";
        std::fs::write(&victim, victim_bytes).expect("write victim");
        symlink(&victim, lease_path(&target)).expect("create symlink collision");

        assert!(matches!(
            DestinationLease::acquire(&target),
            Err(CompareExchangeError::Io(_))
        ));
        assert_eq!(std::fs::read(&victim).expect("read victim"), victim_bytes);
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn prepared_unix_exchange_is_rolled_back_during_restart_reconciliation() {
        let root = unique_temp_dir("unix-prepared-exchange");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let before = b"external predecessor";
        let after = b"uncommitted successor";
        std::fs::write(&target, before).expect("write predecessor");
        std::fs::write(&staged, after).expect("write successor");
        let before_digest = Sha256::digest(before).into();
        let after_digest = Sha256::digest(after).into();
        let (active, backup) =
            prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
                .expect("prepare durable recovery");
        exchange_paths_checked(&backup, &target, after_digest, before_digest)
            .expect("simulate crash after exchange");

        reconcile_unix_recovery_slots(&target).expect("rollback prepared exchange");

        assert_eq!(std::fs::read(&target).expect("read predecessor"), before);
        assert!(!backup.exists());
        assert!(!active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn prepared_unix_new_target_is_removed_during_restart_reconciliation() {
        let root = unique_temp_dir("unix-prepared-new-target");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let after = b"uncommitted successor";
        std::fs::write(&staged, after).expect("write successor");
        let after_digest = Sha256::digest(after).into();
        let (active, backup) = prepare_unix_recovery(&staged, &target, None, after_digest)
            .expect("prepare missing-target recovery");
        move_no_replace_checked(&backup, &target, after_digest)
            .expect("simulate crash after publish");

        reconcile_unix_recovery_slots(&target).expect("rollback prepared new target");

        assert!(!target.exists());
        assert!(!backup.exists());
        assert!(!active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn committed_unix_exchange_survives_restart_and_retires_both_evidence_paths() {
        let root = unique_temp_dir("unix-committed-exchange");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let before = b"accepted predecessor";
        let after = b"committed successor";
        std::fs::write(&target, before).expect("write predecessor");
        std::fs::write(&staged, after).expect("write successor");
        let before_digest = Sha256::digest(before).into();
        let after_digest = Sha256::digest(after).into();
        let (active, backup) =
            prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
                .expect("prepare durable recovery");
        exchange_paths_checked(&backup, &target, after_digest, before_digest)
            .expect("publish successor");
        sync_parent_of(&target).expect("sync publication");
        append_windows_recovery_resolution(&active, PublicationResolution::Committed)
            .expect("commit recovery generation");

        reconcile_unix_recovery_slots(&target).expect("settle committed exchange");

        assert_eq!(std::fs::read(&target).expect("read successor"), after);
        assert!(!backup.exists());
        assert!(!active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn unix_reconciliation_preserves_foreign_endpoint_and_all_recovery_evidence() {
        let root = unique_temp_dir("unix-foreign-endpoint");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let before = b"accepted predecessor";
        let after = b"uncommitted successor";
        std::fs::write(&target, before).expect("write predecessor");
        std::fs::write(&staged, after).expect("write successor");
        let before_digest = Sha256::digest(before).into();
        let after_digest = Sha256::digest(after).into();
        let (active, backup) =
            prepare_unix_recovery(&staged, &target, Some(before_digest), after_digest)
                .expect("prepare durable recovery");
        std::fs::write(&target, b"foreign bytes").expect("write external endpoint");

        assert!(matches!(
            reconcile_unix_recovery_slots(&target),
            Err(PublicationFailure::Uncertain { .. })
        ));
        assert_eq!(
            std::fs::read(&target).expect("read foreign endpoint"),
            b"foreign bytes"
        );
        assert_eq!(std::fs::read(&backup).expect("read successor"), after);
        assert!(active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn unix_exchange_captures_and_restores_a_last_instant_external_edit() {
        let root = unique_temp_dir("unix-raced-predecessor");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let accepted = b"accepted predecessor";
        let external = b"last instant external edit";
        let successor = b"local successor";
        std::fs::write(&target, accepted).expect("write accepted predecessor");
        std::fs::write(&staged, successor).expect("write successor");
        let accepted_digest = Sha256::digest(accepted).into();
        let successor_digest = Sha256::digest(successor).into();
        let (active, backup) =
            prepare_unix_recovery(&staged, &target, Some(accepted_digest), successor_digest)
                .expect("prepare durable recovery");
        std::fs::write(&target, external).expect("race external predecessor");

        exchange_paths_capturing_predecessor(&backup, &target, successor_digest, accepted_digest)
            .expect("capture raced predecessor");
        assert_eq!(std::fs::read(&backup).expect("read capture"), external);
        rollback_retaining_successor(&target, &backup).expect("restore raced predecessor");
        sync_parent_of(&target).expect("sync rollback");
        resolve_unix_recovery_slot(
            &target,
            Some(&backup),
            successor_digest,
            PublicationResolution::Aborted,
        )
        .expect("resolve aborted transaction");
        remove_verified_recovery_sidecar(
            &backup,
            ObservedFileRole::Replacement,
            accepted_digest,
            successor_digest,
        )
        .expect("remove local successor evidence");
        retire_settled_unix_recovery(&target).expect("retire aborted record");

        assert_eq!(
            std::fs::read(&target).expect("read restored edit"),
            external
        );
        assert!(!backup.exists());
        assert!(!active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn unix_failed_exchange_preserves_an_external_deletion_and_restores_staging() {
        let root = unique_temp_dir("unix-raced-deletion");
        let target = root.join("state.json");
        let staged = root.join("staged.json");
        let accepted = b"accepted predecessor";
        let successor = b"local successor";
        std::fs::write(&target, accepted).expect("write accepted predecessor");
        std::fs::write(&staged, successor).expect("write successor");
        let accepted_digest = Sha256::digest(accepted).into();
        let successor_digest = Sha256::digest(successor).into();
        let (active, backup) =
            prepare_unix_recovery(&staged, &target, Some(accepted_digest), successor_digest)
                .expect("prepare durable recovery");
        std::fs::remove_file(&target).expect("race external deletion");
        let error = exchange_paths_capturing_predecessor(
            &backup,
            &target,
            successor_digest,
            accepted_digest,
        )
        .expect_err("exchange must report intact staging");
        let PublicationFailure::Safe(error) = error else {
            panic!("intact staging must be a safe publication failure");
        };
        assert!(matches!(
            abort_unpublished_unix_recovery(
                &staged,
                &target,
                &active,
                &backup,
                successor_digest,
                error,
            ),
            Err(PublicationFailure::Safe(_))
        ));

        assert!(
            !target.exists(),
            "external deletion must not be resurrected"
        );
        assert_eq!(std::fs::read(&staged).expect("read staging"), successor);
        assert!(!backup.exists());
        assert!(!active.path.exists());
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    #[cfg(unix)]
    #[test]
    fn replacement_preserves_existing_unix_mode() {
        use std::os::unix::fs::PermissionsExt as _;

        let root = unique_temp_dir("permissions");
        let target = root.join("state.json");
        std::fs::write(&target, b"old").expect("write predecessor");
        std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o640))
            .expect("set predecessor mode");

        atomic_write_bytes(&target, b"new").expect("publish successor");

        assert_eq!(
            std::fs::metadata(&target)
                .expect("read metadata")
                .permissions()
                .mode()
                & 0o777,
            0o640
        );
        std::fs::remove_dir_all(root).expect("remove fixture");
    }

    fn unique_temp_dir(label: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "rspice-durable-file-{label}-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        std::fs::create_dir_all(&root).expect("create fixture");
        root
    }

    fn assert_only_target_and_lease(root: &Path, target: &Path) {
        let mut actual = std::fs::read_dir(root)
            .expect("read fixture")
            .map(|entry| entry.expect("entry").path())
            .collect::<Vec<_>>();
        let mut expected = vec![target.to_path_buf(), lease_path(target)];
        #[cfg(windows)]
        expected.extend(
            recovery_slot_paths(target)
                .into_iter()
                .filter(|path| path.exists()),
        );
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[cfg(windows)]
    fn assert_no_windows_recovery_artifacts(target: &Path) {
        for slot in recovery_slot_paths(target) {
            assert!(!slot.exists(), "stale recovery slot: {}", slot.display());
            let retired = recovery_retired_path(&slot);
            assert!(
                !retired.exists(),
                "stale retirement tombstone: {}",
                retired.display()
            );
        }
    }
}
