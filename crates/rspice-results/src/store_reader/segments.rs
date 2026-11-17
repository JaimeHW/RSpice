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
use std::sync::{Arc, Mutex, Weak};

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
}

impl Segment {
    /// A run's own segment, deleted with the last column or history that reads it.
    pub fn temporary(reader: StoreReader, path: PathBuf) -> Self {
        Self {
            reader,
            path,
            temporary: true,
        }
    }

    /// A segment a project, a recovery or a session keeps, which no reader deletes.
    pub fn kept(reader: StoreReader, path: PathBuf) -> Self {
        Self {
            reader,
            path,
            temporary: false,
        }
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
        read: impl FnOnce(&mut SegmentReader<SegmentBytes>) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        self.reader.with_reader(read)
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
        self.segment
            .read(self.slot, first, out)
            .map_err(|error| error.to_string())
    }

    /// The time axis: its run checked it ascends strictly before it made its waveforms.
    fn ascending(&self) -> bool {
        self.slot.is_none()
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        identity(&self.segment, self.slot)
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
                .map_err(|error| error.to_string()),
        )
    }

    fn rows_around(&self, rows: Range<u64>, x: f64) -> Option<Result<RowsAround, String>> {
        let slot = self.slot?;
        Some(
            self.segment
                .reader()
                .rows_around(slot, rows, x)
                .map_err(|error| error.to_string()),
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
            .map_err(|error| error.to_string())
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        self.segment
            .reader()
            .events_before(self.signal, t)
            .map_err(|error| error.to_string())
    }

    fn span(&self, t0: f64, t1: f64) -> Result<EventSpan, String> {
        self.segment
            .reader()
            .events_around(self.signal, t0, t1)
            .map_err(|error| error.to_string())
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
    name: String,
}

impl MissingSegment {
    pub fn new(digest: Digest, index: u64, len: u64, name: String) -> Arc<Self> {
        Arc::new(Self {
            digest,
            index,
            len,
            name,
        })
    }

    fn failure(&self) -> String {
        format!(
            "its results store {} is missing: it was deleted or moved outside RSpice",
            rspice_results_store::kept_segment_name(&self.digest)
        )
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

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((self.digest.0, self.index))
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
    open: BTreeMap<[u8; 32], Weak<Segment>>,
}

static KEPT: Mutex<KeptSegments> = Mutex::new(KeptSegments {
    folders: Vec::new(),
    open: BTreeMap::new(),
});

/// Look for kept segments in `folder` too, before every folder named earlier.
pub fn keep_segments_in(folder: &Path) {
    let mut kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    kept.folders.retain(|known| known != folder);
    kept.folders.insert(0, folder.to_path_buf());
}

/// The kept segment whose logical digest is `digest`, opened from the first folder that holds
/// it, or shared with the readers that already have it open. `None` when no folder holds a
/// segment of that digest.
#[cfg(any(unix, windows))]
pub fn kept_segment(digest: Digest) -> Option<Arc<Segment>> {
    let mut kept = KEPT.lock().unwrap_or_else(|p| p.into_inner());
    if let Some(open) = kept.open.get(&digest.0).and_then(Weak::upgrade) {
        return Some(open);
    }
    let name = rspice_results_store::kept_segment_name(&digest);
    let segment = kept.folders.iter().find_map(|folder| {
        let path = folder.join(&name);
        let reader = StoreReader::open_file(&path).ok()?;
        // A file under the name whose rows are not the digest's is not the segment.
        (reader.digest() == Some(digest)).then(|| Arc::new(Segment::kept(reader, path)))
    })?;
    kept.open.retain(|_, open| open.strong_count() > 0);
    kept.open.insert(digest.0, Arc::downgrade(&segment));
    Some(segment)
}

/// No folder holds a segment in a build without files.
#[cfg(not(any(unix, windows)))]
pub fn kept_segment(_digest: Digest) -> Option<Arc<Segment>> {
    None
}
