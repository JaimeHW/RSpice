//! Whether a kept file is the segment its name says.
//!
//! A kept segment is named by its digest, and a digest is a check, not a claim: the `END` a file
//! carries records what its writer computed, and only hashing the stored bits again shows the file
//! still holds them. That reads the whole file, so it is done once per file as it is now: the
//! outcome is remembered by the file's path and identity (its length and times, and on unix its
//! device and inode), and a file changed in any of those is checked again.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use crate::format::POINTERS_AT;
use crate::{Digest, FileIo, SegmentReader, SegmentState, StoreError};

/// What tells one state of a file from another without reading it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Identity {
    len: u64,
    modified: Option<SystemTime>,
    created: Option<SystemTime>,
    /// The device, the inode and the status-change time in nanoseconds.
    #[cfg(unix)]
    inode: (u64, u64, i128),
}

impl Identity {
    fn of(path: &Path) -> Result<Self, StoreError> {
        let metadata = std::fs::metadata(path)?;
        Ok(Self {
            len: metadata.len(),
            modified: metadata.modified().ok(),
            created: metadata.created().ok(),
            #[cfg(unix)]
            inode: {
                use std::os::unix::fs::MetadataExt;
                (
                    metadata.dev(),
                    metadata.ino(),
                    i128::from(metadata.ctime()) * 1_000_000_000
                        + i128::from(metadata.ctime_nsec()),
                )
            },
        })
    }
}

/// Each file checked, by path: the identity it was checked at and the digest its bits hash to.
type Checked = BTreeMap<PathBuf, (Identity, Result<Digest, StoreError>)>;

static CHECKED: Mutex<Checked> = Mutex::new(BTreeMap::new());

fn checked() -> std::sync::MutexGuard<'static, Checked> {
    CHECKED
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Check that the file at `path` is a finished segment whose stored bits hash to `digest`: its
/// rows, summaries and events hash to the digest its `END` records, and that digest is `digest`.
/// The first check of a file reads all of it; later ones, while the file is unchanged, read only
/// its metadata.
pub fn verify_segment(path: &Path, digest: &Digest) -> Result<(), StoreError> {
    check_segment(path, digest, &mut |_, _| true).expect("a check nobody stops runs to its end")
}

/// [`verify_segment`], telling `progress` the work read of all of it as a first check reads the
/// file. `None` when `progress` answered `false`: the check stopped, and nothing of it is
/// remembered.
pub fn check_segment(
    path: &Path,
    digest: &Digest,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<(), StoreError>> {
    let before = match Identity::of(path) {
        Ok(identity) => identity,
        Err(error) => return Some(Err(error)),
    };
    let remembered = checked()
        .get(path)
        .filter(|(identity, _)| *identity == before)
        .map(|(_, outcome)| outcome.clone());
    let outcome = match remembered {
        Some(outcome) => outcome,
        None => {
            let outcome = held_digest(path, progress)?;
            // A file that changed while it was read is checked again next time.
            if Identity::of(path).is_ok_and(|after| after == before) {
                checked().insert(path.to_path_buf(), (before, outcome.clone()));
            }
            outcome
        }
    };
    Some(match outcome {
        Ok(held) if held == *digest => Ok(()),
        Ok(held) => Err(StoreError::DigestMismatch {
            expected: *digest,
            held,
        }),
        Err(error) => Err(error),
    })
}

/// Carry what [`verify_segment`] found of the file at `from` over to `to`, which it was just
/// renamed to: the same file, so its bits need no second read. A rename may change the file's
/// status-change time, so `to` is remembered at its identity now, if its length and modification
/// time are still the ones checked.
pub(crate) fn renamed(from: &Path, to: &Path) {
    let mut checked = checked();
    let Some((identity, outcome)) = checked.remove(from) else {
        return;
    };
    if let Ok(now) = Identity::of(to)
        && now.len == identity.len
        && now.modified == identity.modified
    {
        checked.insert(to.to_path_buf(), (now, outcome));
    }
}

/// Whether [`verify_segment`] already found the file at `path`, as it is now, to hold `digest`.
/// Reads nothing but the file's metadata, so it never stands in for the check itself: a file it
/// says no to may still hold `digest`.
pub fn segment_verified(path: &Path, digest: &Digest) -> bool {
    let Ok(now) = Identity::of(path) else {
        return false;
    };
    checked()
        .get(path)
        .is_some_and(|(identity, outcome)| *identity == now && outcome.as_ref() == Ok(digest))
}

/// The digest the stored bits of the finished segment at `path` hash to, when it is the one its
/// `END` records. `None` when `progress` stopped the read.
fn held_digest(
    path: &Path,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<Digest, StoreError>> {
    let mut reader = match FileIo::open(path)
        .map_err(StoreError::from)
        .and_then(SegmentReader::open)
    {
        Ok(reader) => reader,
        Err(error) => return Some(Err(error)),
    };
    let SegmentState::Complete {
        digest: recorded, ..
    } = reader.state().clone()
    else {
        return Some(Err(reader.end_damage().cloned().unwrap_or_else(|| {
            StoreError::corrupt(
                POINTERS_AT + 16,
                "the segment has no END: the run that wrote it never finished",
            )
        })));
    };
    let held = match reader.digest_with(progress) {
        Ok(held) => held?,
        Err(error) => return Some(Err(error)),
    };
    Some(if held == recorded {
        Ok(held)
    } else {
        Err(StoreError::DigestMismatch {
            expected: recorded,
            held,
        })
    })
}
