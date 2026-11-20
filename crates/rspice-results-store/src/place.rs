//! Putting a finished segment where a project, a recovery or a session keeps it.
//!
//! A kept segment is named by its logical digest ([`kept_segment_name`]), so a folder holds each
//! run once whoever saved it, and a reader finds a run by its digest alone. It gets there as a
//! clone of the file where the volume can share its blocks copy-on-write (APFS), else as a copy;
//! never as a hard link, which would let a later write to the source (a resumed run appending to
//! it) change the kept file under its digest's name. The new file is written as `<name>.partial`,
//! made durable, checked against the digest and only then renamed, so a crash, a cancel or a
//! damaged source never leaves a file under the name that is not the whole segment. A file the
//! folder already holds under the name is checked too, once ([`verify_segment`]), and one that
//! does not hold the digest is replaced from the source.

use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{Digest, kept_segment_name, verify_segment};

/// The bytes a copy moves between checks for a cancel.
const COPY_BLOCK: usize = 8 << 20;

/// How a segment reached its place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Placed {
    /// The folder already held it.
    Present,
    /// A clone sharing the source's blocks until either is written.
    Cloned,
    /// A copy of every byte.
    Copied,
}

/// The ways [`place_segment`] may try before it copies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlaceWays {
    pub clone: bool,
}

impl PlaceWays {
    /// A clone, then a copy.
    pub const ALL: Self = Self { clone: true };
    /// A copy only: what a folder on another volume gets.
    pub const COPY: Self = Self { clone: false };
}

/// Why a segment was not placed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaceError {
    /// The copy was cancelled; nothing was left in the folder.
    Cancelled,
    /// The source could not be read or the folder written.
    Io(String),
}

impl std::fmt::Display for PlaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("the results copy was cancelled"),
            Self::Io(cause) => f.write_str(cause),
        }
    }
}

impl std::error::Error for PlaceError {}

/// Put the finished segment at `source`, whose logical digest is `digest`, into `folder`, by
/// the first of `ways` that works and else by a copy. `copied` is told the bytes a copy has
/// written so far; `cancel` stops a copy between blocks and deletes what it wrote. A file already
/// under the name stays when it holds `digest` and is replaced when it does not; a source whose
/// bits do not hash to `digest` is refused and leaves the folder as it was.
pub fn place_segment(
    source: &Path,
    folder: &Path,
    digest: &Digest,
    ways: PlaceWays,
    cancel: &AtomicBool,
    copied: &mut dyn FnMut(u64),
) -> Result<Placed, PlaceError> {
    let name = kept_segment_name(digest);
    let target = folder.join(&name);
    if target.is_file() && verify_segment(&target, digest).is_ok() {
        return Ok(Placed::Present);
    }
    let io = |what: &str, path: &Path, error: std::io::Error| {
        PlaceError::Io(format!("{what} {}: {error}", path.display()))
    };
    fs::create_dir_all(folder).map_err(|error| io("could not create", folder, error))?;
    let partial = folder.join(format!("{name}.partial"));
    let _ = fs::remove_file(&partial);
    let placed = if ways.clone && clone_file(source, &partial) {
        Placed::Cloned
    } else {
        copy_file(source, &partial, cancel, copied).inspect_err(|_| {
            let _ = fs::remove_file(&partial);
        })?;
        Placed::Copied
    };
    // A clone of a run's own segment shares bytes that were never made durable. Flushing needs
    // a handle that may write: Windows refuses it on a read-only one.
    let durable = fs::OpenOptions::new()
        .write(true)
        .open(&partial)
        .and_then(|file| file.sync_all());
    if let Err(error) = durable {
        let _ = fs::remove_file(&partial);
        return Err(io("could not keep", &target, error));
    }
    // What goes under the name is what was written, checked: a source that does not hold the
    // digest, or a copy the volume damaged, is never kept.
    if let Err(error) = verify_segment(&partial, digest) {
        let _ = fs::remove_file(&partial);
        return Err(PlaceError::Io(format!(
            "could not keep {}: {error}",
            target.display()
        )));
    }
    if let Err(error) = fs::rename(&partial, &target) {
        let _ = fs::remove_file(&partial);
        return Err(io("could not keep", &target, error));
    }
    // Remembered as checked under its kept name, so the next save and the next read of it do not
    // read it all again.
    crate::verify::renamed(&partial, &target);
    sync_folder(folder);
    Ok(placed)
}

/// Delete each kept segment in `folder` that `keep` does not name, and every partial copy.
/// Returns the files deleted. A file that cannot be deleted (open elsewhere) stays for a later
/// call.
pub fn remove_unkept_segments(folder: &Path, keep: &HashSet<Digest>) -> Vec<std::path::PathBuf> {
    let Ok(entries) = fs::read_dir(folder) else {
        return Vec::new();
    };
    let kept: HashSet<String> = keep.iter().map(kept_segment_name).collect();
    let mut deleted = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let stale =
            name.ends_with(".rsr.partial") || (name.ends_with(".rsr") && !kept.contains(name));
        if stale && fs::remove_file(&path).is_ok() {
            deleted.push(path);
        }
    }
    deleted
}

fn copy_file(
    source: &Path,
    target: &Path,
    cancel: &AtomicBool,
    copied: &mut dyn FnMut(u64),
) -> Result<(), PlaceError> {
    let io = |what: &str, path: &Path, error: std::io::Error| {
        PlaceError::Io(format!("{what} {}: {error}", path.display()))
    };
    let mut from = File::open(source).map_err(|error| io("could not read", source, error))?;
    let mut to = File::create(target).map_err(|error| io("could not write", target, error))?;
    let mut block = vec![0_u8; COPY_BLOCK];
    let mut written = 0_u64;
    loop {
        if cancel.load(Ordering::Relaxed) {
            return Err(PlaceError::Cancelled);
        }
        let read = from
            .read(&mut block)
            .map_err(|error| io("could not read", source, error))?;
        if read == 0 {
            return Ok(());
        }
        to.write_all(&block[..read])
            .map_err(|error| io("could not write", target, error))?;
        written += read as u64;
        copied(written);
    }
}

/// Clone `source` to `target` sharing its blocks, where the volume can.
#[cfg(target_os = "macos")]
fn clone_file(source: &Path, target: &Path) -> bool {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let (Ok(source), Ok(target)) = (
        CString::new(source.as_os_str().as_bytes()),
        CString::new(target.as_os_str().as_bytes()),
    ) else {
        return false;
    };
    // SAFETY: both paths are NUL-terminated strings that outlive the call.
    unsafe { libc::clonefile(source.as_ptr(), target.as_ptr(), 0) == 0 }
}

#[cfg(not(target_os = "macos"))]
fn clone_file(_source: &Path, _target: &Path) -> bool {
    false
}

/// Make a rename in `folder` durable, where the host can.
fn sync_folder(folder: &Path) {
    #[cfg(unix)]
    if let Ok(folder) = File::open(folder) {
        let _ = folder.sync_all();
    }
    #[cfg(not(unix))]
    let _ = folder;
}
