//! Keeping the segments a saved project, recovery or session reads beside its file.
//!
//! A saved file names each stored run's segment by its logical digest and holds none of its
//! samples. Before the file is written, every segment it names goes into its folder
//! ([`segment_folder`]): a clone where the volume can share blocks, else a hard link, else a
//! copy, so the file never names a segment its folder lacks. After it is written, the folder
//! lets go of the segments the file no longer names.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use rspice_formats::project_results::{ProjectSimulationResults, StoredSegment, segment_folder};
use rspice_results::store_reader::{
    PlaceError, PlaceWays, SegmentDigest, kept_segment, kept_segment_name, parse_digest,
    place_segment, remove_unkept_segments,
};
use rspice_results::visualization_document::VisualizationDocument;

/// The segments a save must still put in its folder before it writes its file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SegmentsToKeep {
    pub folder: PathBuf,
    /// Each segment's digest and the file it is read from now.
    pub segments: Vec<(SegmentDigest, PathBuf)>,
    /// Their bytes: what a copy of all of them moves.
    pub bytes: u64,
}

impl SegmentsToKeep {
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }
}

/// Each segment a file holding `results` and `documents` reads, once: its runs', and those of
/// the runs its documents archived, which outlive the runs' leaving the history.
pub fn file_segments(
    results: &ProjectSimulationResults,
    documents: &[VisualizationDocument],
) -> Vec<StoredSegment> {
    let mut segments = results.stored_segments();
    for hex in documents
        .iter()
        .flat_map(VisualizationDocument::archived_segments)
    {
        let Ok(digest) = parse_digest(hex) else {
            continue;
        };
        if segments.iter().all(|segment| segment.digest != digest) {
            let source = kept_segment(digest).map(|segment| segment.path().to_path_buf());
            segments.push(StoredSegment { digest, source });
        }
    }
    segments
}

/// What saving a file that reads `segments` to `file` has to keep that its folder does not
/// hold yet. A segment no file holds any more (deleted outside RSpice) stays named, and missing.
pub fn segments_to_keep(segments: Vec<StoredSegment>, file: &Path) -> SegmentsToKeep {
    segments_to_keep_in(segments, segment_folder(file))
}

/// What keeping `segments` in `folder` has to place that it does not hold yet.
fn segments_to_keep_in(segments: Vec<StoredSegment>, folder: PathBuf) -> SegmentsToKeep {
    let segments: Vec<_> = segments
        .into_iter()
        .filter(|segment| !folder.join(kept_segment_name(&segment.digest)).is_file())
        .filter_map(|segment| Some((segment.digest, segment.source?)))
        .collect();
    let bytes = segments
        .iter()
        .filter_map(|(_, source)| std::fs::metadata(source).ok())
        .map(|metadata| metadata.len())
        .sum();
    SegmentsToKeep {
        folder,
        segments,
        bytes,
    }
}

thread_local! {
    static PLACE_BY_COPY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The ways a save started on this thread keeps its segments: every way, unless a test made it
/// copy as a folder on another volume must.
pub fn place_ways() -> PlaceWays {
    if PLACE_BY_COPY.with(std::cell::Cell::get) {
        PlaceWays::COPY
    } else {
        PlaceWays::ALL
    }
}

/// Make saves started on this thread copy their segments, as to a folder on another volume.
#[cfg(any(test, feature = "test-support"))]
pub fn place_by_copy_for_tests(copy: bool) {
    PLACE_BY_COPY.with(|cell| cell.set(copy));
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static HOLD: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Make placements started on this thread wait to keep anything until they are cancelled, so a
/// test cancels a copy before it ends however fast the volume is.
#[cfg(any(test, feature = "test-support"))]
pub fn hold_placements_for_tests(hold: bool) {
    HOLD.with(|cell| cell.set(hold));
}

fn held_for_tests() -> bool {
    #[cfg(any(test, feature = "test-support"))]
    {
        HOLD.with(std::cell::Cell::get)
    }
    #[cfg(not(any(test, feature = "test-support")))]
    false
}

/// Keep `plan`'s segments by `ways`. `copied` is told, while a segment is copied, the bytes kept
/// so far across all of them; a clone or a link tells it nothing. `cancel` stops a copy and
/// leaves nothing of it.
pub fn keep_segments(
    plan: &SegmentsToKeep,
    ways: PlaceWays,
    cancel: &AtomicBool,
    copied: &mut dyn FnMut(u64),
) -> Result<(), PlaceError> {
    let mut done = 0;
    for (digest, source) in &plan.segments {
        place_segment(source, &plan.folder, digest, ways, cancel, &mut |bytes| {
            copied(done + bytes);
        })?;
        done += std::fs::metadata(source).map_or(0, |metadata| metadata.len());
    }
    Ok(())
}

/// A save's first phase: its segments kept on a thread of their own, so a copy to a folder on
/// another volume never holds the window. The save writes its file only once they are all in
/// place; a cancel abandons it with nothing of the copy left.
#[derive(Clone)]
pub struct SegmentPlacement {
    shared: Arc<PlacementShared>,
    total: u64,
    volume: String,
}

#[derive(Default)]
struct PlacementShared {
    cancel: AtomicBool,
    copying: AtomicBool,
    copied: AtomicU64,
    outcome: Mutex<Option<Result<(), PlaceError>>>,
}

impl std::fmt::Debug for SegmentPlacement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SegmentPlacement")
            .field("total", &self.total)
            .field("volume", &self.volume)
            .field("copied", &self.shared.copied.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl SegmentPlacement {
    /// Start keeping `plan`'s segments by `ways`.
    pub fn start(plan: SegmentsToKeep, ways: PlaceWays) -> Self {
        let shared = Arc::new(PlacementShared::default());
        let total = plan.bytes;
        let volume = volume_name(&plan.folder);
        let worker = Arc::clone(&shared);
        let held = held_for_tests();
        let spawned = std::thread::Builder::new()
            .name("rspice-keep-results".to_owned())
            .spawn(move || {
                while held && !worker.cancel.load(Ordering::Relaxed) {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                let outcome = keep_segments(&plan, ways, &worker.cancel, &mut |bytes| {
                    worker.copied.store(bytes, Ordering::Relaxed);
                    worker.copying.store(true, Ordering::Relaxed);
                });
                *lock(&worker.outcome) = Some(outcome);
            });
        if let Err(error) = spawned {
            *lock(&shared.outcome) = Some(Err(PlaceError::Io(format!(
                "the results could not be kept beside the project: {error}"
            ))));
        }
        Self {
            shared,
            total,
            volume,
        }
    }

    /// The bytes kept so far and the bytes to keep, while a copy runs; `None` while the
    /// segments are cloned or linked, which takes no time, and once the placement ended.
    pub fn copy_progress(&self) -> Option<(u64, u64)> {
        (self.shared.copying.load(Ordering::Relaxed) && self.outcome().is_none()).then(|| {
            (
                self.shared.copied.load(Ordering::Relaxed).min(self.total),
                self.total,
            )
        })
    }

    /// The volume the segments are kept on, as a reader knows it.
    pub fn volume(&self) -> &str {
        &self.volume
    }

    /// How the placement ended, once it has.
    pub fn outcome(&self) -> Option<Result<(), PlaceError>> {
        lock(&self.shared.outcome).clone()
    }

    /// Stop a copy between blocks; the thread deletes what it wrote and ends cancelled.
    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Relaxed);
    }

    /// Cancel and wait, at most `patience`, for the thread to delete what it wrote: what quitting
    /// in the middle of a copy does. Returns whether the placement ended.
    pub fn cancel_and_wait(&self, patience: std::time::Duration) -> bool {
        self.cancel();
        let until = std::time::Instant::now() + patience;
        while self.outcome().is_none() {
            if std::time::Instant::now() >= until {
                return false;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        true
    }
}

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

/// The volume `folder` is on, named as the reader sees it: the name it is mounted under
/// (`Backup` for `/Volumes/Backup` or `/media/me/Backup`), or its drive (`D:`), or the folder
/// itself on the startup disk.
pub fn volume_name(folder: &Path) -> String {
    let existing = folder
        .ancestors()
        .find(|path| path.exists())
        .unwrap_or(folder);
    let mount = mount_point(existing);
    #[cfg(unix)]
    {
        let named = if cfg!(target_os = "macos") {
            mount.parent() == Some(Path::new("/Volumes"))
        } else {
            mount.parent().is_some()
        };
        match mount.file_name() {
            Some(name) if named => name.to_string_lossy().into_owned(),
            _ => folder.display().to_string(),
        }
    }
    #[cfg(not(unix))]
    {
        mount.display().to_string()
    }
}

/// The folder `path`'s volume is mounted at: its farthest ancestor on the same device.
#[cfg(unix)]
fn mount_point(path: &Path) -> PathBuf {
    use std::os::unix::fs::MetadataExt as _;
    let device = |path: &Path| std::fs::metadata(path).map(|metadata| metadata.dev()).ok();
    let own = device(path);
    let mut mount = path.to_path_buf();
    for ancestor in path.ancestors().skip(1) {
        if device(ancestor) != own {
            break;
        }
        mount = ancestor.to_path_buf();
    }
    mount
}

/// The drive `path` is on.
#[cfg(not(unix))]
fn mount_point(path: &Path) -> PathBuf {
    path.components().next().map_or_else(
        || path.to_path_buf(),
        |prefix| PathBuf::from(prefix.as_os_str()),
    )
}

/// After a file that reads `segments` was saved to `file`: delete the segments its folder
/// keeps that it no longer reads, and any partial copy.
pub fn release_unread_segments(segments: &[StoredSegment], file: &Path) {
    release_unread_segments_in(segments, &segment_folder(file));
}

fn release_unread_segments_in(segments: &[StoredSegment], folder: &Path) {
    if !folder.is_dir() {
        return;
    }
    let read = segments.iter().map(|segment| segment.digest).collect();
    remove_unkept_segments(folder, &read);
}

/// Keep every one of `segments` beside `file` now, on this thread: what a save that did not
/// place them first does before it writes.
pub fn keep_segments_now(segments: Vec<StoredSegment>, file: &Path) -> Result<(), String> {
    keep_segments_now_in(segments, segment_folder(file))
}

fn keep_segments_now_in(segments: Vec<StoredSegment>, folder: PathBuf) -> Result<(), String> {
    let plan = segments_to_keep_in(segments, folder);
    if plan.is_empty() {
        return Ok(());
    }
    keep_segments(&plan, place_ways(), &AtomicBool::new(false), &mut |_| {})
        .map_err(|error| format!("the results could not be kept beside the project: {error}"))
}

/// The folder that keeps the segments the stored session reads, beside the application's other
/// local data.
pub fn session_segment_folder() -> Option<PathBuf> {
    dirs::data_local_dir().map(|data| data.join("RSpice").join("results").join("session"))
}

/// Keep every segment `results` reads in the session's folder before the session is stored, so
/// a relaunch reads them after this process's own store is gone.
pub fn keep_session_segments(segments: Vec<StoredSegment>) -> Result<(), String> {
    let folder = session_segment_folder()
        .ok_or_else(|| "no local folder is available to keep the results in".to_owned())?;
    keep_segments_now_in(segments, folder)
}

/// After the session was stored with `results`, let its folder go of the segments they no
/// longer read.
pub fn release_unread_session_segments(segments: &[StoredSegment]) {
    if let Some(folder) = session_segment_folder() {
        release_unread_segments_in(segments, &folder);
    }
}
