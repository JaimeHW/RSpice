//! Kept segments left for a check of their own instead of read whole as they are opened.
//!
//! A kept file is read as the segment its name says only once its stored bits hash to the
//! digest ([`rspice_results_store::verify_segment`]), and that first check reads all of it.
//! Opening a project would read every segment it names before the project showed. While
//! [`opening_unchecked`] runs, a kept file not checked yet is left unread instead: it is queued
//! on this thread for a check ([`take_unchecked_segments`]), and it reads as missing until that
//! check finds it is the segment. Meanwhile [`checking`] says its digest is being checked and
//! [`check_progress`] how far the check got, so its run says it is being checked rather than
//! missing. [`check`] is the check, run wherever the caller runs its long work.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use rspice_results_store::Digest;

/// A kept file waiting for its first check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncheckedSegment {
    pub digest: Digest,
    /// The kept file.
    pub path: PathBuf,
    /// A copy of the segment that goes in the file's place when the check finds the file is not
    /// the segment: a save's own rows, where its folder held a file under the name already.
    pub replace_from: Option<PathBuf>,
}

#[derive(Default)]
struct Progress {
    done: AtomicU64,
    total: AtomicU64,
}

/// Every file waiting for or under its check, by path: its digest and how far its check got.
static AWAITING: Mutex<BTreeMap<PathBuf, (Digest, Arc<Progress>)>> = Mutex::new(BTreeMap::new());

fn awaiting() -> std::sync::MutexGuard<'static, BTreeMap<PathBuf, (Digest, Arc<Progress>)>> {
    AWAITING.lock().unwrap_or_else(|p| p.into_inner())
}

thread_local! {
    /// How deep this thread is in [`opening_unchecked`].
    static OPENING: Cell<u32> = const { Cell::new(0) };
    /// The files this thread left for a check and has not handed on yet.
    static QUEUED: RefCell<Vec<UncheckedSegment>> = const { RefCell::new(Vec::new()) };
}

/// Run `open`, leaving each kept file it finds unchecked for a check of its own instead of
/// reading it whole.
pub fn opening_unchecked<T>(open: impl FnOnce() -> T) -> T {
    struct Depth;
    impl Drop for Depth {
        fn drop(&mut self) {
            OPENING.with(|depth| depth.set(depth.get().saturating_sub(1)));
        }
    }
    OPENING.with(|depth| depth.set(depth.get() + 1));
    let _depth = Depth;
    open()
}

/// Whether this thread is in [`opening_unchecked`].
#[cfg(any(unix, windows))]
pub(super) fn deferring() -> bool {
    OPENING.with(Cell::get) > 0
}

/// Leave `segment` for a check: queued on this thread, and said to be checked until it is.
/// A file already waiting is not queued twice.
pub fn defer_check(segment: UncheckedSegment) {
    {
        let mut awaiting = awaiting();
        if awaiting.contains_key(&segment.path) {
            return;
        }
        awaiting.insert(
            segment.path.clone(),
            (segment.digest, Arc::new(Progress::default())),
        );
    }
    QUEUED.with(|queued| queued.borrow_mut().push(segment));
}

/// The files this thread left for a check since it last asked, each to be checked once.
pub fn take_unchecked_segments() -> Vec<UncheckedSegment> {
    QUEUED.with(|queued| std::mem::take(&mut *queued.borrow_mut()))
}

/// Whether the file at `path` is waiting for or under its check.
pub fn awaiting_check(path: &Path) -> bool {
    awaiting().contains_key(path)
}

/// Whether a file of digest `digest` is waiting for or under its check.
pub fn checking(digest: [u8; 32]) -> bool {
    awaiting().values().any(|(held, _)| held.0 == digest)
}

/// How far the check of a file of digest `digest` got, as work done of work known; `None` when
/// no file of it is waiting for one.
pub fn check_progress(digest: [u8; 32]) -> Option<(u64, u64)> {
    awaiting()
        .values()
        .find(|(held, _)| held.0 == digest)
        .map(|(_, progress)| {
            (
                progress.done.load(Ordering::Relaxed),
                progress.total.load(Ordering::Relaxed),
            )
        })
}

/// Check `segment`: read the whole file and hash its stored bits, telling `progress` the work
/// done of the work known, and remember what it found, so the file is read as the segment from
/// then on, or as damaged. A file that is not the segment is replaced by `replace_from` where
/// there is one. `None` when `progress` answered `false`: the check stopped, and the file is
/// checked again the next time it is opened.
#[cfg(any(unix, windows))]
pub fn check(
    segment: &UncheckedSegment,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<(), String>> {
    let shared = awaiting()
        .get(&segment.path)
        .map(|(_, shared)| Arc::clone(shared))
        .unwrap_or_default();
    let mut report = |done: u64, total: u64| {
        shared.done.store(done, Ordering::Relaxed);
        shared.total.store(total, Ordering::Relaxed);
        progress(done, total)
    };
    let checked = rspice_results_store::check_segment(&segment.path, &segment.digest, &mut report);
    let outcome = match checked {
        None => None,
        Some(Ok(())) => Some(Ok(())),
        Some(Err(error)) => match (&segment.replace_from, segment.path.parent()) {
            (Some(source), Some(folder)) => replace(source, folder, segment, &mut report),
            _ => Some(Err(error.to_string())),
        },
    };
    awaiting().remove(&segment.path);
    outcome
}

/// Put the segment at `source` in place of the damaged file `segment` names.
#[cfg(any(unix, windows))]
fn replace(
    source: &Path,
    folder: &Path,
    segment: &UncheckedSegment,
    report: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<(), String>> {
    use rspice_results_store::{PlaceError, PlaceWays, place_segment};
    let total = std::fs::metadata(source).map_or(0, |metadata| metadata.len());
    let stop = std::sync::atomic::AtomicBool::new(false);
    let placed = place_segment(
        source,
        folder,
        &segment.digest,
        PlaceWays::ALL,
        &stop,
        &mut |bytes| {
            if !report(bytes, total) {
                stop.store(true, Ordering::Relaxed);
            }
        },
    );
    match placed {
        Ok(_) => Some(Ok(())),
        Err(PlaceError::Cancelled) => None,
        Err(PlaceError::Io(error)) => Some(Err(error)),
    }
}
