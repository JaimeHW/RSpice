//! The segments waveforms and event histories read, and the kept segments found by digest.
//!
//! A run's own segment lives in its process's store folder and goes with the last column or
//! history that reads it. A kept segment is one a project, a recovery or a session holds in a
//! store folder of its own, named by its logical digest
//! ([`rspice_results_store::kept_segment_name`]). A saved result names the segments it reads by
//! digest alone, so reading it back needs no paths: [`kept_segment`] looks in every folder
//! [`keep_segments_in`] has named, and a segment no folder holds reads as missing, by name.

use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, Weak};

use rspice_results_store::{Digest, EventSpan, SegmentReader, StoreError};

use super::{SegmentBytes, StoreReader};
use crate::events::points::EventHistorySource;
use crate::waveform::{ColumnSource, ColumnTile, RowsAround};

/// A finished segment that columns and event histories read.
pub struct Segment {
    reader: StoreReader,
    path: PathBuf,
    /// Whether the last reader deletes the file: a run's own segment, which nothing reads once
    /// its process ends.
    temporary: bool,
    /// Each analog signal's first value slot and its id, in slot order: built once, on the
    /// first column named, so naming every column of a wide segment reads its schema once.
    slot_signals: OnceLock<Vec<(usize, usize)>>,
}

impl Segment {
    /// A run's own segment, deleted with the last column or history that reads it.
    pub fn temporary(reader: StoreReader, path: PathBuf) -> Self {
        Self {
            reader,
            path,
            temporary: true,
            slot_signals: OnceLock::new(),
        }
    }

    /// A segment a project, a recovery or a session keeps, which no reader deletes.
    pub fn kept(reader: StoreReader, path: PathBuf) -> Self {
        Self {
            reader,
            path,
            temporary: false,
            slot_signals: OnceLock::new(),
        }
    }

    /// The segment shared with every reader that names it by its digest, so a reference
    /// written while it is open reads it from where it is.
    pub fn shared(self) -> Arc<Self> {
        let digest = self.reader.digest();
        let segment = Arc::new(self);
        if let Some(digest) = digest {
            KEPT.lock()
                .unwrap_or_else(|p| p.into_inner())
                .register(digest, &segment);
        }
        segment
    }

    /// The segment file.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Readable rows.
    pub fn rows(&self) -> u64 {
        self.reader.rows()
    }

    /// The reader every column of the segment shares.
    pub fn reader(&self) -> &StoreReader {
        &self.reader
    }

    /// Run `read` with the segment's reader.
    pub fn with_reader<T>(
        &self,
        read: impl FnOnce(&SegmentReader<SegmentBytes>) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        self.reader.with_reader(read)
    }

    /// The name of the signal whose first value slot is `slot`, or `None` when no analog
    /// signal starts there or the schema cannot be read.
    pub fn slot_name(&self, slot: usize) -> Option<String> {
        let index = match self.slot_signals.get() {
            Some(index) => index,
            None => {
                let built = self
                    .with_reader(|reader| {
                        let mut first = 0;
                        let mut index = Vec::new();
                        for (id, signal) in reader.schema().signals.iter().enumerate() {
                            if !signal.kind.is_event() {
                                index.push((first, id));
                            }
                            first += signal.kind.slots();
                        }
                        Ok(index)
                    })
                    .ok()?;
                self.slot_signals.get_or_init(|| built)
            }
        };
        let at = index
            .binary_search_by_key(&slot, |&(first, _)| first)
            .ok()?;
        let id = index[at].1;
        self.with_reader(|reader| Ok(reader.schema().signals.get(id).map(|s| s.name.clone())))
            .ok()
            .flatten()
    }

    /// Rows `first .. first + out.len()` of the x axis (`slot` `None`) or a value slot.
    pub fn read(&self, slot: Option<usize>, first: u64, out: &mut [f64]) -> Result<(), StoreError> {
        self.reader.read(slot, first, out)
    }
}

impl Drop for Segment {
    fn drop(&mut self) {
        if self.temporary {
            // Close the segment before deleting it: a host may refuse to delete an open file.
            self.reader.close();
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

/// Why a read of a segment failed, in a reader's words: rows on their way to this process (a
/// browser page's segment read from its worker) fail as [`crate::waveform::PENDING_READ`], which
/// no reader keeps.
fn read_cause(error: StoreError) -> String {
    match error {
        StoreError::Io {
            kind: std::io::ErrorKind::WouldBlock,
            ..
        } => crate::waveform::PENDING_READ.to_owned(),
        error => error.to_string(),
    }
}

/// The identity a column or event signal of `segment` has: its store's logical digest and
/// `index`.
fn identity(segment: &Segment, index: Option<usize>) -> Option<([u8; 32], u64)> {
    let index = index.map_or(Some(0), |index| u64::try_from(index).ok()?.checked_add(1))?;
    Some((segment.reader.digest()?.0, index))
}

/// One column of a segment, as a waveform reads it.
pub struct SegmentColumn {
    segment: Arc<Segment>,
    /// The value slot, or `None` for the x axis.
    slot: Option<usize>,
    name: String,
}

impl SegmentColumn {
    pub fn new(segment: Arc<Segment>, slot: Option<usize>, name: String) -> Arc<Self> {
        Arc::new(Self {
            segment,
            slot,
            name,
        })
    }
}

impl ColumnSource for SegmentColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.segment.path().display().to_string()
    }

    fn rows(&self) -> u64 {
        self.segment.rows()
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.segment.read(self.slot, first, out).map_err(read_cause)
    }

    /// The time axis: its run checked it ascends strictly before it made its waveforms.
    fn ascending(&self) -> bool {
        self.slot.is_none()
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        identity(&self.segment, self.slot)
    }

    fn column_digest(&self) -> Option<[u8; 32]> {
        let column = self.slot.map_or(Some(0), |slot| slot.checked_add(1))?;
        Some(self.segment.reader().column_digest(column)?.0)
    }

    fn tile(
        &self,
        rows: Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Option<Result<ColumnTile, String>> {
        let slot = self.slot?;
        Some(
            self.segment
                .reader()
                .tile(slot, rows, x0, x1, pixels)
                .map(ColumnTile::from)
                .map_err(read_cause),
        )
    }

    fn rows_around(&self, rows: Range<u64>, x: f64) -> Option<Result<RowsAround, String>> {
        let slot = self.slot?;
        Some(
            self.segment
                .reader()
                .rows_around(slot, rows, x)
                .map_err(read_cause),
        )
    }
}

/// One digital or real event signal of a segment, as its history reads it.
pub struct SegmentEvents {
    segment: Arc<Segment>,
    signal: usize,
    name: String,
}

impl SegmentEvents {
    pub fn new(segment: Arc<Segment>, signal: usize, name: String) -> Arc<Self> {
        Arc::new(Self {
            segment,
            signal,
            name,
        })
    }
}

impl EventHistorySource for SegmentEvents {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.segment.path().display().to_string()
    }

    fn count(&self) -> Result<u64, String> {
        self.segment
            .reader()
            .event_count(self.signal)
            .map_err(read_cause)
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        self.segment
            .reader()
            .events_before(self.signal, t)
            .map_err(read_cause)
    }

    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String> {
        self.segment
            .reader()
            .events_around(self.signal, t0, t1)
            .map_err(read_cause)
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((
            self.segment.reader().digest()?.0,
            u64::try_from(self.signal).ok()?,
        ))
    }
}

/// A column or event signal of a kept segment that no store folder holds. It reads as
/// unavailable, naming the file, and keeps the identity its result was saved with.
pub struct MissingSegment {
    digest: Digest,
    index: u64,
    /// The rows of a column, or the changes of a signal, the saved result stated.
    len: u64,
    /// A column's digest, as the saved result stated it.
    column_digest: Option<[u8; 32]>,
    /// Whether a column's rows ascend strictly, as the saved result stated it.
    ascending: bool,
    name: String,
}

impl MissingSegment {
    pub fn new(digest: Digest, index: u64, len: u64, name: String) -> Arc<Self> {
        Arc::new(Self {
            digest,
            index,
            len,
            column_digest: None,
            ascending: false,
            name,
        })
    }

    /// A column of a missing segment, keeping the column digest and the ascent its result was
    /// saved with, so the result saves again as it was.
    pub fn column(
        digest: Digest,
        column: u64,
        rows: u64,
        ascending: bool,
        column_digest: Option<[u8; 32]>,
        name: String,
    ) -> Arc<Self> {
        Arc::new(Self {
            digest,
            index: column,
            len: rows,
            column_digest,
            ascending,
            name,
        })
    }

    fn failure(&self) -> String {
        missing(self.digest)
    }
}

/// Why no folder holds a segment of `digest`. A browser keeps a project's runs in its own
/// storage, which another browser, or a page that could not keep a run, never wrote: those runs
/// are not damaged, they are not here.
fn missing(digest: Digest) -> String {
    if cfg!(target_arch = "wasm32") {
        return "its samples are not kept in this browser: it was run in another browser, or \
                this browser's storage could not keep it"
            .to_owned();
    }
    format!(
        "its results store {} is missing: it was deleted or moved outside RSpice",
        rspice_results_store::kept_segment_name(&digest)
    )
}

/// Why no segment of `digest` reads, after its check ended: the file a folder holds is not the
/// segment, or no folder holds one.
pub(super) fn unread(digest: Digest) -> String {
    let name = rspice_results_store::kept_segment_name(&digest);
    let folders = KEPT
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .folders
        .clone();
    match folders
        .iter()
        .map(|folder| folder.join(&name))
        .find(|path| path.is_file())
    {
        Some(path) => format!(
            "its results store {} is damaged: its rows are not the ones its name states",
            path.display()
        ),
        None => missing(digest),
    }
}

impl ColumnSource for MissingSegment {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        rspice_results_store::kept_segment_name(&self.digest)
    }

    fn rows(&self) -> u64 {
        self.len
    }

    fn read(&self, _first: u64, _out: &mut [f64]) -> Result<(), String> {
        Err(self.failure())
    }

    fn ascending(&self) -> bool {
        self.ascending
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.digest.0, self.index))
    }

    fn column_digest(&self) -> Option<[u8; 32]> {
        self.column_digest
    }
}

impl EventHistorySource for MissingSegment {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        rspice_results_store::kept_segment_name(&self.digest)
    }

    fn count(&self) -> Result<u64, String> {
        Ok(self.len)
    }

    fn before(&self, _t: f64) -> Result<u64, String> {
        Err(self.failure())
    }

    fn span(&self, _t0: f64, _t1: f64) -> Result<EventSpan, String> {
        Err(self.failure())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.digest.0, self.index))
    }
}

/// The folders kept segments are looked for in, newest first, and the kept segments open now.
struct KeptSegments {
    folders: Vec<PathBuf>,
    /// Every segment open now, by its logical digest. Two open segments can hold one digest (two
    /// runs that solved the same rows), and either serves a reader of it, so each is listed for
    /// as long as it is open: one closing never hides the other.
    open: BTreeMap<[u8; 32], Vec<Weak<Segment>>>,
}

impl KeptSegments {
    /// An open segment whose logical digest is `digest`.
    #[cfg(any(unix, windows))]
    fn open(&self, digest: Digest) -> Option<Arc<Segment>> {
        self.open.get(&digest.0)?.iter().find_map(Weak::upgrade)
    }

    /// List `segment` as open under `digest`, dropping every listing that has closed.
    fn register(&mut self, digest: Digest, segment: &Arc<Segment>) {
        self.open.retain(|_, open| {
            open.retain(|segment| segment.strong_count() > 0);
            !open.is_empty()
        });
        self.open
            .entry(digest.0)
            .or_default()
            .push(Arc::downgrade(segment));
    }
}

static KEPT: Mutex<KeptSegments> = Mutex::new(KeptSegments {
    folders: Vec::new(),
    open: BTreeMap::new(),
});

/// The segments a document archived, held open for as long as this process runs: a run's own
/// segment outlives the run's leaving the history until a save keeps it beside its file.
#[cfg(any(unix, windows))]
static PINNED: Mutex<Vec<Arc<Segment>>> = Mutex::new(Vec::new());

/// Hold the segment whose digest is `digest` open for as long as this process runs, and say
/// where its file is. `None` when no reader has it open and no folder keeps it.
#[cfg(any(unix, windows))]
pub fn pin_segment(digest: Digest) -> Option<PathBuf> {
    let segment = kept_segment(digest)?;
    let path = segment.path.clone();
    let mut pinned = PINNED.lock().unwrap_or_else(|p| p.into_inner());
    if !pinned.iter().any(|known| Arc::ptr_eq(known, &segment)) {
        pinned.push(segment);
    }
    Some(path)
}

/// Look for kept segments in `folder` too, before every folder named earlier.
pub fn keep_segments_in(folder: &Path) {
    let mut kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    kept.folders.retain(|known| known != folder);
    kept.folders.insert(0, folder.to_path_buf());
}

/// The kept segment whose logical digest is `digest`, opened from the first folder that holds
/// it, or shared with the readers that already have it open. `None` when no folder holds a
/// segment of that digest.
///
/// A file under the digest's name is the segment only when its stored bits hash to the digest
/// ([`rspice_results_store::verify_segment`]): the first time a file is opened that reads all of
/// it, on the calling thread, unless the caller is [`super::opening_unchecked`] or the window's
/// thread ([`super::never_wait_for_checks_here`]). There a file not checked yet is left for a
/// check of its own, and `None` until it has had it, as is every file already left for one: its
/// result reads as being checked ([`super::checking`]). Anywhere else, a file left for a check
/// joins it: this waits for the check under way, or runs it when none has started, and then
/// opens the file the check found to be the segment.
#[cfg(any(unix, windows))]
pub fn kept_segment(digest: Digest) -> Option<Arc<Segment>> {
    let lock = || KEPT.lock().unwrap_or_else(|p| p.into_inner());
    let folders = {
        let kept = lock();
        if let Some(open) = kept.open(digest) {
            return Some(open);
        }
        kept.folders.clone()
    };
    let name = rspice_results_store::kept_segment_name(&digest);
    let leaving = super::checks::leaving_checks();
    let opening = super::checks::deferring();
    let mut unchecked = None;
    // Checked without holding the lock, so a long first check does not stall other reads.
    let segment = folders.iter().find_map(|folder| {
        let path = folder.join(&name);
        // A job of its own reads it whole; reading it here too would read it twice, so an open
        // and the window's thread leave it to that job, and anything else waits for the job or
        // takes its place.
        if super::checks::awaiting_check(&path) {
            if leaving {
                return None;
            }
            super::checks::join(&path);
        }
        if leaving {
            // An open reads only the file's metadata. Elsewhere a check stopped before it reads
            // a block answers what an earlier check found, so a file found damaged is not left
            // for another check.
            let found = if opening {
                rspice_results_store::segment_verified(&path, &digest).then_some(true)
            } else {
                rspice_results_store::check_segment(&path, &digest, &mut |_, _| false)
                    .map(|checked| checked.is_ok())
            };
            match found {
                Some(true) => {}
                Some(false) => return None,
                None => {
                    if unchecked.is_none() && path.is_file() {
                        unchecked = Some(path);
                    }
                    return None;
                }
            }
        }
        rspice_results_store::verify_segment(&path, &digest).ok()?;
        let reader = StoreReader::open_file(&path).ok()?;
        Some(Arc::new(Segment::kept(reader, path)))
    });
    let Some(segment) = segment else {
        if let Some(path) = unchecked {
            super::checks::defer_check(super::checks::UncheckedSegment {
                digest,
                path,
                replace_from: None,
            });
        }
        return None;
    };
    let mut kept = lock();
    if let Some(open) = kept.open(digest) {
        return Some(open);
    }
    kept.register(digest, &segment);
    Some(segment)
}

/// A build without files has no folder to look in: the kept segment of that digest is the one a
/// reader has open, which the browser loads from its own storage before reading a result again.
#[cfg(not(any(unix, windows)))]
pub fn kept_segment(digest: Digest) -> Option<Arc<Segment>> {
    let kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    kept.open.get(&digest.0)?.iter().find_map(Weak::upgrade)
}

/// Every kept segment some reader has open now, one for each digest.
#[cfg(not(any(unix, windows)))]
pub fn open_segments() -> Vec<Arc<Segment>> {
    let kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    kept.open
        .values()
        .filter_map(|open| open.iter().find_map(Weak::upgrade))
        .collect()
}
