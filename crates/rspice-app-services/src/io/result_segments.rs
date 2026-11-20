//! Keeping the segments a saved project, recovery or session reads beside its file.
//!
//! A saved file names each stored run's segment by its logical digest and holds none of its
//! samples. Before the file is written, every segment it names goes into its folder
//! ([`segment_folder`]): a clone where the volume can share blocks, else a copy, so the file
//! never names a segment its folder lacks. After it is written, the folder
//! lets go of the segments the file no longer names.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use rspice_formats::project_results::{ProjectSimulationResults, StoredSegment, segment_folder};
use rspice_results::store_reader::{
    PlaceError, PlaceWays, SegmentDigest, kept_segment, kept_segment_name, parse_digest,
    place_segment, remove_unkept_segments, segment_verified,
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
        .filter(|segment| {
            !segment_verified(
                &folder.join(kept_segment_name(&segment.digest)),
                &segment.digest,
            )
        })
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

/// The folder that holds every instance's session folder, beside the application's other local
/// data.
#[cfg(not(any(test, feature = "test-support")))]
fn session_root() -> Option<PathBuf> {
    dirs::data_local_dir().map(|data| data.join("RSpice").join("results").join("session"))
}

/// A test keeps its sessions' results under the temporary folder, never in the reader's own.
#[cfg(any(test, feature = "test-support"))]
fn session_root() -> Option<PathBuf> {
    Some(std::env::temp_dir().join("rspice-session-tests"))
}

/// The lock file beside an instance's session folder, held for as long as the instance runs.
fn session_lock_path(folder: &Path) -> PathBuf {
    let mut name = folder.as_os_str().to_owned();
    name.push(".lock");
    PathBuf::from(name)
}

/// An instance's session folder and its hold on the folder's lock: while the hold lasts, no
/// other instance touches the folder.
struct InstanceFolder {
    folder: PathBuf,
    _lock: std::fs::File,
}

/// What this instance knows of the sessions it reads and stores.
#[derive(Default)]
struct SessionSegments {
    own: Option<InstanceFolder>,
    /// The segments the session on disk reads: the one read at launch, then each one stored. Its
    /// folder keeps them until the next session stored is on disk.
    previous: std::collections::HashSet<SegmentDigest>,
    /// The session's segments going into its folder while closing waits.
    placing: Option<SegmentPlacement>,
    /// Closing chose not to wait for them: the session is stored without its runs.
    without_runs: bool,
}

#[cfg(not(any(test, feature = "test-support")))]
static SESSION: Mutex<Option<SessionSegments>> = Mutex::new(None);

// Each test thread is an instance of its own, so tests running together never share a folder.
#[cfg(any(test, feature = "test-support"))]
thread_local! {
    static SESSION: std::cell::RefCell<Option<SessionSegments>> =
        const { std::cell::RefCell::new(None) };
}

fn with_session<T>(act: impl FnOnce(&mut SessionSegments) -> T) -> T {
    #[cfg(not(any(test, feature = "test-support")))]
    {
        act(lock(&SESSION).get_or_insert_with(SessionSegments::default))
    }
    #[cfg(any(test, feature = "test-support"))]
    {
        SESSION.with(|session| {
            act(session
                .borrow_mut()
                .get_or_insert_with(SessionSegments::default))
        })
    }
}

/// Make this instance's folder: lock a new lock file, then make the folder it guards.
fn create_instance_folder(root: &Path) -> std::io::Result<InstanceFolder> {
    std::fs::create_dir_all(root)?;
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos());
    let mut attempt = 0u32;
    loop {
        let folder = root.join(format!("{}-{stamp:x}-{attempt}", std::process::id()));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(session_lock_path(&folder))
        {
            Ok(lock) => {
                lock.try_lock().map_err(std::io::Error::other)?;
                std::fs::create_dir(&folder)?;
                return Ok(InstanceFolder {
                    folder,
                    _lock: lock,
                });
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => attempt += 1,
            Err(error) => return Err(error),
        }
    }
}

/// This instance's own session folder, made the first time it is asked for.
fn own_session_folder(session: &mut SessionSegments) -> Option<PathBuf> {
    if session.own.is_none() {
        let root = session_root()?;
        session.own = create_instance_folder(&root)
            .inspect_err(|error| log::warn!("{}: {error}", root.display()))
            .ok();
    }
    session.own.as_ref().map(|own| own.folder.clone())
}

/// The session folders of the other instances, each with whether that instance still runs.
fn other_session_folders(own: &Path) -> Vec<(PathBuf, bool)> {
    let Some(entries) = session_root().and_then(|root| std::fs::read_dir(root).ok()) else {
        return Vec::new();
    };
    let mut folders: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|folder| folder.is_dir() && folder != own)
        .map(|folder| {
            let running = std::fs::File::open(session_lock_path(&folder))
                .is_ok_and(|lock| lock.try_lock().is_err());
            (folder, running)
        })
        .collect();
    folders.sort();
    folders
}

/// Before the stored session is read: this instance takes over what the instances that have
/// ended kept, moving it into its own folder, and looks for the session's segments there first,
/// then in the folders of the instances still running. Any of them may have stored the session;
/// a segment is read only when its rows are the digest its run names.
pub fn read_stored_session_segments() {
    let Some(own) = with_session(own_session_folder) else {
        return;
    };
    for (folder, running) in other_session_folders(&own) {
        if running {
            rspice_formats::project_results::keep_segments_in(&folder);
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&folder) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let target = own.join(entry.file_name());
            if target.exists() {
                let _ = std::fs::remove_file(entry.path());
            } else if let Err(error) = std::fs::rename(entry.path(), &target) {
                log::warn!("{}: {error}", entry.path().display());
            }
        }
        if std::fs::remove_dir(&folder).is_ok() {
            let _ = std::fs::remove_file(session_lock_path(&folder));
        }
    }
    rspice_formats::project_results::keep_segments_in(&own);
}

/// The stored session just read reads `segments`: this instance's folder keeps them until a
/// session it stores is on disk.
pub fn stored_session_reads(segments: &[StoredSegment]) {
    with_session(|session| {
        session.previous = segments.iter().map(|segment| segment.digest).collect();
    });
}

/// Keep every one of `segments` in this instance's session folder before the session is
/// stored, so a relaunch reads them after this process's own store is gone. Fails when closing
/// chose not to wait for them.
pub fn keep_session_segments(segments: Vec<StoredSegment>) -> Result<(), String> {
    let (folder, placing, without_runs) = with_session(|session| {
        (
            own_session_folder(session),
            session.placing.take(),
            session.without_runs,
        )
    });
    if let Some(placing) = placing {
        placing.cancel_and_wait(std::time::Duration::from_secs(5));
    }
    if without_runs {
        return Err("closing did not wait for the results to be kept".to_owned());
    }
    let folder =
        folder.ok_or_else(|| "no local folder is available to keep the results in".to_owned())?;
    keep_segments_now_in(segments, folder)
}

/// After the session was stored with `segments`: this instance's folder lets go of what neither
/// it nor the session on disk reads. The stored session is written out after this, and a crash
/// before then leaves the one on disk, whose segments stay. Another instance's folder is never
/// touched.
pub fn release_unread_session_segments(segments: &[StoredSegment]) {
    let read: std::collections::HashSet<SegmentDigest> =
        segments.iter().map(|segment| segment.digest).collect();
    with_session(|session| {
        let kept = read.union(&session.previous).copied().collect();
        if let Some(own) = &session.own {
            remove_unkept_segments(&own.folder, &kept);
        }
        session.previous = read;
    });
}

/// What keeping the session's results asks of closing.
#[derive(Debug, Clone, PartialEq)]
pub enum SessionResultsAtClose {
    /// The session's folder holds them: closing goes on.
    Kept,
    /// They are cloned into it or checked there, which takes a moment.
    Placing,
    /// They are copied to the volume named, the bytes copied and the bytes to copy so far.
    Copying {
        volume: String,
        copied: u64,
        total: u64,
    },
}

/// Closing: start putting `segments` into this instance's session folder off the frame, unless
/// it holds them. Closing waits while this says they are not kept yet.
pub fn place_session_segments_for_close(segments: Vec<StoredSegment>) -> SessionResultsAtClose {
    with_session(|session| {
        if session.without_runs {
            return SessionResultsAtClose::Kept;
        }
        if session.placing.is_none() {
            let Some(folder) = own_session_folder(session) else {
                return SessionResultsAtClose::Kept;
            };
            let plan = segments_to_keep_in(segments, folder);
            if plan.is_empty() {
                return SessionResultsAtClose::Kept;
            }
            session.placing = Some(SegmentPlacement::start(plan, place_ways()));
        }
        placement_at_close(session)
    })
}

/// Where keeping the session's results for closing stands. A placement that ended is let go:
/// what it could not keep, storing the session tries once more.
pub fn session_results_at_close() -> SessionResultsAtClose {
    with_session(placement_at_close)
}

fn placement_at_close(session: &mut SessionSegments) -> SessionResultsAtClose {
    let Some(placing) = &session.placing else {
        return SessionResultsAtClose::Kept;
    };
    if let Some(outcome) = placing.outcome() {
        if let Err(error) = outcome {
            log::warn!("the session's results were not kept: {error}");
        }
        session.placing = None;
        return SessionResultsAtClose::Kept;
    }
    match placing.copy_progress() {
        Some((copied, total)) => SessionResultsAtClose::Copying {
            volume: placing.volume().to_owned(),
            copied,
            total,
        },
        None => SessionResultsAtClose::Placing,
    }
}

/// Closing does not wait for the session's results: the copy stops and leaves nothing, and the
/// session is stored without its runs.
pub fn close_without_session_results() {
    with_session(|session| {
        if let Some(placing) = session.placing.take() {
            placing.cancel_and_wait(std::time::Duration::from_secs(5));
        }
        session.without_runs = true;
    });
}

/// Closing was called off: the copy stops and leaves nothing.
pub fn stop_placing_session_segments() {
    with_session(|session| {
        if let Some(placing) = session.placing.take() {
            placing.cancel();
        }
    });
}

/// Run `act` as another instance of the application: with a session folder of its own, let go
/// of when `act` returns, as the instance ending.
#[cfg(any(test, feature = "test-support"))]
pub fn as_another_instance<T>(act: impl FnOnce() -> T) -> T {
    let this = SESSION.with(|session| session.borrow_mut().take());
    let acted = act();
    let other = SESSION.with(|session| std::mem::replace(&mut *session.borrow_mut(), this));
    drop(other);
    acted
}

/// Locate results: look for kept segments in `folder` too, and read again every run of the
/// history that could not read its samples, its signals sharing one axis as on a fresh run. The
/// runs that now read every sample; a run whose segment `folder` does not hold stays as it was.
/// A save keeps the segments found beside the project, so they are found there from then on.
pub fn locate_results(simulation: &mut crate::state::SimulationState, folder: &Path) -> usize {
    use rspice_formats::project_results::{read_again, read_events_again};
    use rspice_results::store_reader::ColumnReference;
    rspice_formats::project_results::keep_segments_in(folder);
    let unread: Vec<usize> = (0..simulation.retained.runs.len())
        .filter(|&index| simulation.retained.runs[index].unread_results().is_some())
        .collect();
    let mut located = 0;
    for index in unread {
        let run = &mut simulation.retained.runs[index];
        let mut axes: Vec<(ColumnReference, rspice_results::waveform::WaveformValues)> = Vec::new();
        for analysis in &mut run.data.analyses {
            for waveform in &mut analysis.data.waveforms {
                let x = &mut waveform.data.x;
                if let Some(reference) = ColumnReference::of(x) {
                    match axes.iter().find(|(met, _)| *met == reference) {
                        Some((_, axis)) => *x = axis.clone(),
                        None => {
                            if let Some(axis) = read_again(x) {
                                *x = axis;
                            }
                            axes.push((reference, x.clone()));
                        }
                    }
                }
                let complex = waveform
                    .data
                    .complex
                    .iter_mut()
                    .flat_map(|complex| [&mut complex.real, &mut complex.imag]);
                for values in std::iter::once(&mut waveform.data.y).chain(complex) {
                    if let Some(again) = read_again(values) {
                        *values = again;
                    }
                }
            }
            if let Some(
                rspice_results::analysis_payload::AnalysisResultPayload::TransientEvents {
                    digital_traces,
                    real_traces,
                    ..
                },
            ) = analysis.data.result_payload.as_mut()
            {
                for trace in digital_traces {
                    if let Some(again) = read_events_again(&trace.points) {
                        trace.points = again;
                    }
                }
                for trace in real_traces {
                    if let Some(again) = read_events_again(&trace.points) {
                        trace.points = again;
                    }
                }
            }
        }
        if run.unread_results().is_none() {
            located += 1;
        }
    }
    located
}
