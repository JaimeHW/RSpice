//! Kept segments left for a check of their own instead of read whole as they are opened.
//!
//! A kept file is read as the segment its name says only once its stored bits hash to the
//! digest ([`rspice_results_store::verify_segment`]), and that first check reads all of it.
//! Opening a project would read every segment it names before the project showed. While
//! [`opening_unchecked`] runs, a kept file not checked yet is left unread instead: it is queued
//! on this thread for a check ([`take_unchecked_segments`]), and until that check ends it reads
//! as being checked, never as missing: [`checking`] says its digest is being checked and
//! [`check_progress`] how far the check got. [`check`] is the check, run wherever the caller runs
//! its long work. A read of the file's samples outside [`opening_unchecked`] joins its check: it
//! waits for the check under way, or runs the check itself when none has started, so the file
//! is read whole once.
//!
//! The thread that draws the window never waits for a check ([`never_wait_for_checks_here`]): a
//! read there that meets a file being checked reads as being checked at once, and a file not
//! checked yet is left for a check of its own, as an open leaves it. The waker
//! ([`set_check_waker`]) asks for the frame that starts a check left that way and for the one
//! after each check ends, which reads the file again.

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
#[cfg(any(unix, windows))]
use std::sync::Condvar;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

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

/// A file waiting for or under its check.
struct Awaiting {
    segment: UncheckedSegment,
    progress: Arc<Progress>,
    /// Whether its check has started: a job's, or a reader's that found none under way.
    #[cfg_attr(not(any(unix, windows)), allow(dead_code))]
    started: bool,
}

/// Every file waiting for or under its check, by path.
static AWAITING: Mutex<BTreeMap<PathBuf, Awaiting>> = Mutex::new(BTreeMap::new());

/// Told each time a check ends, so whoever waits for one looks again.
#[cfg(any(unix, windows))]
static ENDED: Condvar = Condvar::new();

fn awaiting() -> MutexGuard<'static, BTreeMap<PathBuf, Awaiting>> {
    AWAITING.lock().unwrap_or_else(|p| p.into_inner())
}

thread_local! {
    /// How deep this thread is in [`opening_unchecked`].
    static OPENING: Cell<u32> = const { Cell::new(0) };
    /// Whether this thread never waits for a check: the thread that draws the window.
    static NEVER_WAITS: Cell<bool> = const { Cell::new(false) };
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
pub(super) fn deferring() -> bool {
    OPENING.with(Cell::get) > 0
}

/// Make this thread one that never waits for the check of a kept file: the thread that draws
/// the window. A read here that meets a file being checked reads as being checked at once, and
/// a file not checked yet is left for a check of its own instead of read whole here.
pub fn never_wait_for_checks_here() {
    NEVER_WAITS.with(|never| never.set(true));
    // The thread that never waits for a check never waits for the reader service either.
    crate::waveform::never_wait_for_reads_here();
}

/// Whether a read here leaves each kept file not checked yet for a check of its own and waits
/// for none: inside [`opening_unchecked`], and on a thread that never waits for a check.
#[cfg(any(unix, windows))]
pub(super) fn leaving_checks() -> bool {
    deferring() || NEVER_WAITS.with(Cell::get)
}

/// What asks for a frame when a check is left to start or a check ends.
static WAKER: Mutex<Option<Box<dyn Fn() + Send + Sync>>> = Mutex::new(None);

/// Ask `waker` for a frame each time a file is left for a check, so the frame starts its job,
/// and each time a check ends, so the frame reads the file again: what the window installs.
pub fn set_check_waker(waker: impl Fn() + Send + Sync + 'static) {
    *WAKER.lock().unwrap_or_else(|p| p.into_inner()) = Some(Box::new(waker));
}

fn wake() {
    if let Some(waker) = WAKER.lock().unwrap_or_else(|p| p.into_inner()).as_ref() {
        waker();
    }
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
            Awaiting {
                segment: segment.clone(),
                progress: Arc::new(Progress::default()),
                started: false,
            },
        );
    }
    QUEUED.with(|queued| queued.borrow_mut().push(segment));
    wake();
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
    awaiting()
        .values()
        .any(|held| held.segment.digest.0 == digest)
}

/// The file of digest `digest` waiting for or under its check, if one is: where a save copies
/// the segment from while the check runs, without reading it first.
pub fn file_being_checked(digest: [u8; 32]) -> Option<PathBuf> {
    awaiting()
        .values()
        .find(|held| held.segment.digest.0 == digest)
        .map(|held| held.segment.path.clone())
}

/// How far the check of a file of digest `digest` got, as work done of work known; `None` when
/// no file of it is waiting for one.
pub fn check_progress(digest: [u8; 32]) -> Option<(u64, u64)> {
    awaiting()
        .values()
        .find(|held| held.segment.digest.0 == digest)
        .map(|held| {
            (
                held.progress.done.load(Ordering::Relaxed),
                held.progress.total.load(Ordering::Relaxed),
            )
        })
}

/// What a caller of [`claim`] does about a file's check.
#[cfg(any(unix, windows))]
enum Claim {
    /// Runs it: nobody had started it.
    Run(UncheckedSegment, Arc<Progress>),
    /// Nothing: it ended, or the file was never left for one.
    Ended,
}

/// Start the check of the file at `path` when nobody has; wait for the one under way to end
/// when somebody has.
#[cfg(any(unix, windows))]
fn claim(path: &Path) -> Claim {
    let mut awaiting = awaiting();
    loop {
        match awaiting.get_mut(path) {
            None => return Claim::Ended,
            Some(held) if !held.started => {
                held.started = true;
                return Claim::Run(held.segment.clone(), Arc::clone(&held.progress));
            }
            Some(_) => awaiting = ENDED.wait(awaiting).unwrap_or_else(|p| p.into_inner()),
        }
    }
}

/// The end of a claimed check, however it ends: the file waits no longer, and whoever waits for
/// it looks again.
#[cfg(any(unix, windows))]
struct Ends<'a>(&'a Path);

#[cfg(any(unix, windows))]
impl Drop for Ends<'_> {
    fn drop(&mut self) {
        awaiting().remove(self.0);
        ENDED.notify_all();
        wake();
    }
}

/// Wait for the check of the file at `path` to end, running it here when nobody has started it:
/// what a read of the file's samples does off the window's thread, so the file is read whole
/// once. Returns at once for a file not waiting for a check.
#[cfg(any(unix, windows))]
pub(super) fn join(path: &Path) {
    debug_assert!(
        !leaving_checks(),
        "a thread that never waits for a check joined one"
    );
    if let Claim::Run(segment, shared) = claim(path) {
        let _ = run(&segment, &shared, &mut |_, _| true);
    }
}

/// Check `segment`: read the whole file and hash its stored bits, telling `progress` the work
/// done of the work known, and remember what it found, so the file is read as the segment from
/// then on, or as damaged. A file that is not the segment is replaced by `replace_from` where
/// there is one. A read that joined the check first ran it already; then this answers what it
/// found without reading the file again. `None` when `progress` answered `false`: the check
/// stopped, and the file is checked again the next time it is read.
#[cfg(any(unix, windows))]
pub fn check(
    segment: &UncheckedSegment,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<(), String>> {
    match claim(&segment.path) {
        Claim::Run(claimed, shared) => run(&claimed, &shared, progress),
        // The check that ended remembered what it found, so this reads nothing again.
        Claim::Ended => {
            rspice_results_store::check_segment(&segment.path, &segment.digest, progress)
                .map(|checked| checked.map_err(|error| error.to_string()))
        }
    }
}

/// Run the check of `segment`, claimed by the caller, reporting to `shared` as well as to
/// `progress`.
#[cfg(any(unix, windows))]
fn run(
    segment: &UncheckedSegment,
    shared: &Progress,
    progress: &mut dyn FnMut(u64, u64) -> bool,
) -> Option<Result<(), String>> {
    let _ends = Ends(&segment.path);
    let mut report = |done: u64, total: u64| {
        shared.done.store(done, Ordering::Relaxed);
        shared.total.store(total, Ordering::Relaxed);
        progress(done, total)
    };
    let checked = rspice_results_store::check_segment(&segment.path, &segment.digest, &mut report);
    match checked {
        None => None,
        Some(Ok(())) => Some(Ok(())),
        Some(Err(error)) => match (&segment.replace_from, segment.path.parent()) {
            (Some(source), Some(folder)) => replace(source, folder, segment, &mut report),
            _ => Some(Err(error.to_string())),
        },
    }
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
