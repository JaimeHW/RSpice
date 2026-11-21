//! A waveform's samples: in memory, or a window of a column a results store holds.
//!
//! An analysis written to a results store keeps its samples on disk. Its waveforms hold a window
//! of each stored column instead of the values, and nothing ever reads a window whole and keeps
//! it: a reader asks for a range, a pass a block at a time, a sample, a tile of a view (per pixel,
//! the least and greatest value of the rows in it), the rows around a cursor, its extent or its
//! ends, which come from the store's summaries and the few rows they name. A run larger than
//! memory is read the same way. A column held in memory is shared by its clones, so a run's axis
//! is one allocation however many waveforms share it.

use std::borrow::Cow;
use std::fmt;
use std::ops::{Bound, Range, RangeBounds};
use std::sync::{Arc, Mutex, OnceLock};

/// A column of samples kept outside memory and read on demand: one signal of a run in a results
/// store.
pub trait ColumnSource: Send + Sync {
    /// The column's name, as a failure to read it names it.
    fn name(&self) -> &str;

    /// The store that holds the column, as its host names it (a segment's path), so a failure to
    /// read it can be traced to the run that wrote it.
    fn store(&self) -> String;

    /// The rows the column holds.
    fn rows(&self) -> u64;

    /// Read rows `first .. first + out.len()` into `out`.
    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String>;

    /// Whether the column's rows ascend strictly: a time axis its run checked as it ended.
    fn ascending(&self) -> bool {
        false
    }

    /// Whether the column stands in for one of a kept store still waiting for its first check.
    /// A read of it waits for that check, or fails at once as [`CHECKING_READ`] where nothing
    /// may wait, and the host reads the window again once the check ends.
    fn stands_in(&self) -> bool {
        false
    }

    /// The logical digest of the store that holds the column and the column's index in it (its
    /// run's x axis being 0): a column with the same identity holds the same rows. `None` when
    /// the store states no digest, as a run recovered without its end does not.
    fn identity(&self) -> Option<([u8; 32], u64)> {
        None
    }

    /// The digest of the whole column, as its store states it: the
    /// `rspice_results_store::ColumnDigest` of its rows, which the same column held in memory
    /// has too. `None` when the store states none, as a run recovered without its end does not.
    fn column_digest(&self) -> Option<[u8; 32]> {
        None
    }

    /// The tile of the rows in `rows` with x in `x0 ..= x1` at `pixels` pixels, x being the axis
    /// of the run the column belongs to; `None` when the source keeps nothing to tile from.
    fn tile(
        &self,
        _rows: Range<u64>,
        _x0: f64,
        _x1: f64,
        _pixels: usize,
    ) -> Option<Result<ColumnTile, String>> {
        None
    }

    /// The rows of `rows` around `x` on its run's axis, each as (x, value), with the index of
    /// the first: up to two at or before `x` and up to two after, consecutive. `None` when the
    /// source cannot find them without reading the column.
    fn rows_around(&self, _rows: Range<u64>, _x: f64) -> Option<Result<RowsAround, String>> {
        None
    }
}

/// Consecutive rows of a column around a cursor, each as (x, value), and the index of the
/// first.
pub type RowsAround = (u64, Vec<(f64, f64)>);

/// What a stored column shows of a view of its rows: N pixels over x in [x0, x1], x being the
/// axis of the run the column belongs to.
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnTile {
    /// At most two rows per pixel: every row with x in the view, and the nearest row either
    /// side of them, each as (x, value).
    Samples {
        x: Vec<f64>,
        y: Vec<f64>,
        before: Option<(f64, f64)>,
        after: Option<(f64, f64)>,
    },
    /// Per pixel, the least and greatest value of the rows in it (NaN in both where they hold
    /// no available value), and whether any row fell in it.
    Envelope {
        min: Vec<f64>,
        max: Vec<f64>,
        touched: Vec<bool>,
    },
}

impl ColumnTile {
    /// A view of no rows: what a column that cannot be read shows.
    fn empty(pixels: usize) -> Self {
        Self::Envelope {
            min: vec![f64::NAN; pixels],
            max: vec![f64::NAN; pixels],
            touched: vec![false; pixels],
        }
    }

    /// This view with the sample `(x, y)` that comes before every row in it.
    fn with_lead(mut self, (x, y): (f64, f64), x0: f64, x1: f64) -> Self {
        let inside = x >= x0 && x <= x1;
        match &mut self {
            Self::Samples {
                x: xs,
                y: ys,
                before,
                after,
            } => {
                if inside {
                    xs.insert(0, x);
                    ys.insert(0, y);
                } else if x < x0 {
                    before.get_or_insert((x, y));
                } else {
                    *after = Some((x, y));
                }
            }
            Self::Envelope { min, max, touched } => {
                if inside {
                    let pixels = min.len();
                    let pixel = if x1 > x0 {
                        (((x - x0) / (x1 - x0) * pixels as f64) as usize).min(pixels - 1)
                    } else {
                        0
                    };
                    if !y.is_nan() {
                        min[pixel] = if min[pixel] <= y { min[pixel] } else { y };
                        max[pixel] = if max[pixel] >= y { max[pixel] } else { y };
                    }
                    touched[pixel] = true;
                }
            }
        }
        self
    }
}

/// The pixels a stored waveform's extent is read at: fine enough that the least positive pixel
/// extreme is close to the least positive value.
const EXTENT_PIXELS: usize = 1024;

/// The least and greatest available values of a set of samples, and the least positive one.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueExtent {
    pub min: f64,
    pub max: f64,
    pub min_positive: f64,
}

/// What a stored window holds before its first row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowLead {
    /// Nothing: the window starts at its first row.
    None,
    /// This value: an axis's own start, between two rows.
    Value(f64),
    /// The value between the row before the first and the first, `fraction` of the way from the
    /// one to the other: a waveform's value at its axis's start.
    Interpolated(f64),
}

/// The digest of a column held in memory, as a results store digests each column it stores
/// (`rspice_results_store::ColumnDigest`, which this build may not link): the SHA-256 of its rows
/// as little-endian binary64, in row order.
#[derive(Default)]
struct ColumnHasher(sha2::Sha256);

impl ColumnHasher {
    fn values(&mut self, values: &[f64]) {
        use sha2::Digest as _;
        let mut staged = [0u8; 4096];
        for block in values.chunks(staged.len() / 8) {
            let bytes = &mut staged[..8 * block.len()];
            for (out, value) in bytes.chunks_exact_mut(8).zip(block) {
                out.copy_from_slice(&value.to_le_bytes());
            }
            self.0.update(bytes);
        }
    }

    fn finish(self) -> [u8; 32] {
        use sha2::Digest as _;
        self.0.finalize().into()
    }
}

/// What identifies a waveform's samples wherever they are held
/// ([`WaveformValues::samples_identity`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SamplesIdentity {
    /// The digest of the column the samples are a window of: the SHA-256 of its rows as
    /// little-endian binary64, as a results store digests each column it stores
    /// (`rspice_results_store::ColumnDigest`).
    pub column: [u8; 32],
    /// The rows that column holds.
    pub column_rows: u64,
    /// The window of it the samples are: the whole column for samples held in memory.
    pub window: StoredWindow,
}

/// The part of a stored column a waveform holds.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StoredWindow {
    /// The first row the window holds.
    pub first: u64,
    /// The rows it holds from `first` on.
    pub rows: u64,
    /// What it holds before `first`.
    pub lead: WindowLead,
}

/// A stored column that could not be read: the waveform it belongs to shows the window as
/// unavailable samples, and the failure is kept for the run's diagnostics to name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnReadFailure {
    /// The column's name.
    pub column: String,
    /// The store that holds it, as [`ColumnSource::store`] names it.
    pub store: String,
    /// Why it could not be read.
    pub cause: String,
}

impl fmt::Display for ColumnReadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "the stored samples of '{}' could not be read from {}: {}",
            self.column, self.store, self.cause
        )
    }
}

impl std::error::Error for ColumnReadFailure {}

/// The cause a source gives a read of rows that are on their way to this process, as a browser
/// page's are from the worker that holds its segments: the read fails now and a later one finds
/// them, so the failure is not kept and the window is not shown as unavailable.
pub const PENDING_READ: &str = "its samples are still being read from this browser's storage";

/// The cause a stand-in for a kept results store gives a read made, while the store waits for its
/// first check, on a thread that never waits for one: the read fails now and a later one, once
/// the check ends, finds the rows.
pub const CHECKING_READ: &str = "its results store is still being checked";

/// Whether a read failed only because its rows are on their way ([`PENDING_READ`]) or their store
/// is still being checked ([`CHECKING_READ`]).
pub fn is_pending_read(cause: &str) -> bool {
    cause == PENDING_READ || cause == CHECKING_READ || cause == super::reads::BEING_READ
}

/// Failures of reads that had no caller to return them to, waiting for diagnostics to collect.
static UNREPORTED_FAILURES: Mutex<Vec<ColumnReadFailure>> = Mutex::new(Vec::new());

/// Keep a failed read that had no caller to return it to, for diagnostics to collect.
#[cfg(feature = "engine-evidence")]
pub(crate) fn report_column_read_failure(failure: ColumnReadFailure) {
    UNREPORTED_FAILURES
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .push(failure);
}

/// Take every stored column read that failed behind a waveform's values since the last call, so
/// a host can show each one. Each column fails at most once: its window keeps the gap it showed.
pub fn take_column_read_failures() -> Vec<ColumnReadFailure> {
    std::mem::take(
        &mut *UNREPORTED_FAILURES
            .lock()
            .unwrap_or_else(|p| p.into_inner()),
    )
}

/// Samples a pass over a stored window reads at a time: 64 Ki samples, half a mebibyte.
pub const BLOCK_SAMPLES: usize = 1 << 16;

/// The most bytes [`WaveformValues::read_whole`] reads of a stored window: the loaded-results
/// budget, 1 GiB until the host sets its own.
static WHOLE_READ_BUDGET: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1 << 30);

/// Refuse a whole read of a stored window larger than `bytes`: the host's loaded-results budget.
pub fn set_whole_read_budget(bytes: u64) {
    WHOLE_READ_BUDGET.store(bytes, std::sync::atomic::Ordering::Relaxed);
}

/// A waveform's samples.
///
/// Samples held in memory are one slice ([`Self::in_memory`]). A stored window is never read
/// whole and kept: a reader asks for what it needs of it (a range with [`Self::read_range`], a
/// pass a block at a time with [`Self::blocks`] or [`Self::samples`], one sample, a tile, the
/// value at an x, its extent or its ends) and keeps what it read itself. A window that cannot be
/// read shows as unavailable samples (NaN) to a reader that takes no failure, and its failure
/// goes to [`take_column_read_failures`] once. [`Self::len`] never reads.
#[derive(Clone)]
pub struct WaveformValues(Repr);

#[derive(Clone)]
enum Repr {
    Memory(Arc<Vec<f64>>),
    Stored(Arc<Stored>),
}

struct Stored {
    source: Arc<dyn ColumnSource>,
    window: StoredWindow,
    failure: OnceLock<ColumnReadFailure>,
    /// The value an interpolated lead holds, read once.
    lead_value: OnceLock<f64>,
    /// The first and last sample, read once.
    ends: OnceLock<Option<(f64, f64)>>,
}

impl Stored {
    fn len(&self) -> usize {
        // The window was checked against the source when it was made, and a source whose rows
        // the host cannot address could not have been written by it.
        usize::try_from(self.window.rows).unwrap_or(usize::MAX)
            + usize::from(self.window.lead != WindowLead::None)
    }

    /// Keep the window's first failure, and hand it to diagnostics once. A read whose rows are
    /// on their way is not a failure the window keeps.
    fn fail(&self, failure: ColumnReadFailure) {
        if is_pending_read(&failure.cause) {
            return;
        }
        if self.failure.set(failure.clone()).is_ok() {
            UNREPORTED_FAILURES
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .push(failure);
        }
    }

    fn failed(&self, cause: String) -> ColumnReadFailure {
        ColumnReadFailure {
            column: self.source.name().to_owned(),
            store: self.source.store(),
            cause,
        }
    }

    /// The rows of the column the window holds.
    fn rows(&self) -> Range<u64> {
        self.window.first..self.window.first + self.window.rows
    }

    /// What the window holds before its first row, read once for an interpolated lead: NaN
    /// when it cannot be read.
    fn lead(&self) -> Option<f64> {
        self.read_lead().unwrap_or(Some(f64::NAN))
    }

    /// [`Self::lead`], or the failure of a read whose rows are on their way, which is not kept.
    fn read_lead(&self) -> Result<Option<f64>, ColumnReadFailure> {
        let fraction = match self.window.lead {
            WindowLead::None => return Ok(None),
            WindowLead::Value(value) => return Ok(Some(value)),
            WindowLead::Interpolated(fraction) => fraction,
        };
        if let Some(value) = self.lead_value.get() {
            return Ok(Some(*value));
        }
        // The row before the first, moved `fraction` of the way to the first, with the
        // arithmetic the in-memory window uses.
        let count = if self.window.rows > 0 { 2 } else { 1 };
        let mut pair = [0.0; 2];
        let value = match self.source.read(self.window.first - 1, &mut pair[..count]) {
            Ok(()) if count == 2 => pair[0] + fraction * (pair[1] - pair[0]),
            Ok(()) => pair[0],
            Err(cause) if is_pending_read(&cause) => return Err(self.failed(cause)),
            Err(cause) => {
                self.fail(self.failed(cause));
                f64::NAN
            }
        };
        Ok(Some(*self.lead_value.get_or_init(|| value)))
    }

    /// Read samples `start .. start + out.len()` of the window into `out`.
    fn read_into(&self, start: usize, out: &mut [f64]) -> Result<(), ColumnReadFailure> {
        if let Some(failure) = self.failure.get() {
            return Err(failure.clone());
        }
        if out.is_empty() {
            return Ok(());
        }
        let lead = usize::from(self.window.lead != WindowLead::None);
        let skip = usize::from(start < lead);
        if skip == 1 {
            out[0] = self.read_lead()?.unwrap_or(f64::NAN);
            if let Some(failure) = self.failure.get() {
                return Err(failure.clone());
            }
            if out.len() == 1 {
                return Ok(());
            }
        }
        let first = self.window.first + (start + skip - lead) as u64;
        if let Err(cause) = self.source.read(first, &mut out[skip..]) {
            let failure = self.failed(cause);
            self.fail(failure.clone());
            return Err(failure);
        }
        Ok(())
    }

    /// The first and last sample, reading only those. `None` while their rows are on their way.
    fn ends(&self) -> Option<(f64, f64)> {
        if let Some(ends) = self.ends.get() {
            return *ends;
        }
        let pending = std::cell::Cell::new(false);
        let row = |row: u64| {
            let mut value = [0.0];
            match self.source.read(row, &mut value) {
                Ok(()) => value[0],
                Err(cause) => {
                    pending.set(pending.get() || is_pending_read(&cause));
                    self.fail(self.failed(cause));
                    f64::NAN
                }
            }
        };
        let lead = self.read_lead().unwrap_or_else(|_| {
            pending.set(true);
            Some(f64::NAN)
        });
        let ends = match (self.window.rows, lead) {
            (0, None) => None,
            (0, Some(lead)) => Some((lead, lead)),
            (rows, lead) => {
                let last = row(self.window.first + rows - 1);
                Some((lead.unwrap_or_else(|| row(self.window.first)), last))
            }
        };
        if pending.get() {
            return None;
        }
        *self.ends.get_or_init(|| ends)
    }

    /// The tile of the window over x in `x0 ..= x1`, the window's lead at `lead_x`.
    fn tile(&self, lead_x: Option<f64>, x0: f64, x1: f64, pixels: usize) -> Option<ColumnTile> {
        if self.failure.get().is_some() {
            return Some(ColumnTile::empty(pixels));
        }
        let tile = match self.source.tile(self.rows(), x0, x1, pixels)? {
            Ok(tile) => tile,
            Err(cause) => {
                self.fail(self.failed(cause));
                return Some(ColumnTile::empty(pixels));
            }
        };
        Some(match lead_x.zip(self.lead()) {
            Some(lead) => tile.with_lead(lead, x0, x1),
            None => tile,
        })
    }

    /// The window's value at `at`, the window's lead at `lead_x`.
    fn value_at(
        &self,
        lead_x: Option<f64>,
        at: f64,
        interpolation: crate::sampling::SampleInterpolation,
    ) -> Option<f64> {
        if self.failure.get().is_some() {
            return Some(f64::NAN);
        }
        let (first, mut rows) = match self.source.rows_around(self.rows(), at)? {
            Ok(around) => around,
            Err(cause) => {
                self.fail(self.failed(cause));
                return Some(f64::NAN);
            }
        };
        // The lead comes before the window's first row: a cursor near it reads it too.
        if first == self.window.first
            && let Some(lead) = lead_x.zip(self.lead())
        {
            rows.insert(0, lead);
        }
        let (x, y): (Vec<f64>, Vec<f64>) = rows.into_iter().unzip();
        Some(crate::sampling::sample_at_with(&x, &y, at, interpolation))
    }
}

impl WaveformValues {
    /// A window of `source`. Refused when it reaches past the rows the source holds, or
    /// interpolates from a row before the source's first.
    pub fn stored(source: Arc<dyn ColumnSource>, window: StoredWindow) -> Result<Self, String> {
        let end = window.first.checked_add(window.rows);
        if end.is_none_or(|end| end > source.rows()) {
            return Err(format!(
                "the window of rows {} .. {} reaches past the {} rows of '{}'",
                window.first,
                window.first.saturating_add(window.rows),
                source.rows(),
                source.name()
            ));
        }
        if matches!(window.lead, WindowLead::Interpolated(_)) && window.first == 0 {
            return Err(format!(
                "the window of '{}' interpolates its start from before its first row",
                source.name()
            ));
        }
        Ok(Self(Repr::Stored(Arc::new(Stored {
            source,
            window,
            failure: OnceLock::new(),
            lead_value: OnceLock::new(),
            ends: OnceLock::new(),
        }))))
    }

    /// How many samples there are, without reading a stored window.
    pub fn len(&self) -> usize {
        match &self.0 {
            Repr::Memory(values) => values.len(),
            Repr::Stored(stored) => stored.len(),
        }
    }

    /// Whether there are no samples, without reading a stored window.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether the samples are a window of a stored column.
    pub fn is_stored(&self) -> bool {
        matches!(self.0, Repr::Stored(_))
    }

    /// The stored column and the window of it these samples are, if they are one.
    pub fn stored_window(&self) -> Option<(&Arc<dyn ColumnSource>, StoredWindow)> {
        match &self.0 {
            Repr::Memory(_) => None,
            Repr::Stored(stored) => Some((&stored.source, stored.window)),
        }
    }

    /// What identifies the samples wherever they are held: the digest of the column they are a
    /// window of, the rows it holds, and the window. Samples held in memory are a whole column of
    /// their own, digested here, so they have the identity the same column stored has. A window of
    /// a stored column whose store states the column's digest reads no row; one whose store
    /// states none is digested as its samples in memory are, reading them a block at a time.
    pub fn samples_identity(&self) -> SamplesIdentity {
        if let Some((source, window)) = self.stored_window()
            && let Some(column) = source.column_digest()
        {
            return SamplesIdentity {
                column,
                column_rows: source.rows(),
                window,
            };
        }
        let mut digest = ColumnHasher::default();
        match self.in_memory() {
            Some(values) => digest.values(values),
            None => {
                let mut staged = Vec::with_capacity(BLOCK_SAMPLES.min(self.len()));
                for value in self.samples(..) {
                    staged.push(value);
                    if staged.len() == BLOCK_SAMPLES {
                        digest.values(&staged);
                        staged.clear();
                    }
                }
                digest.values(&staged);
            }
        }
        let rows = self.len() as u64;
        SamplesIdentity {
            column: digest.finish(),
            column_rows: rows,
            window: StoredWindow {
                first: 0,
                rows,
                lead: WindowLead::None,
            },
        }
    }

    /// Whether the samples are a window of a column whose rows ascend strictly: a stored run's
    /// time axis.
    pub fn is_ascending_axis(&self) -> bool {
        matches!(&self.0, Repr::Stored(stored) if stored.source.ascending())
    }

    /// The first and last sample, reading at most those two of a stored window.
    pub fn ends(&self) -> Option<(f64, f64)> {
        match &self.0 {
            Repr::Memory(values) => Some((*values.first()?, *values.last()?)),
            Repr::Stored(stored) => stored.ends(),
        }
    }

    /// The samples, when they are held in memory: one slice. A stored window has none; it is
    /// read by range, block, sample, tile or point.
    pub fn in_memory(&self) -> Option<&[f64]> {
        match &self.0 {
            Repr::Memory(values) => {
                column_work::view(self);
                Some(values)
            }
            Repr::Stored(_) => None,
        }
    }

    /// Sample `index`: from memory, or that one row of a stored window, which reads NaN when the
    /// window cannot be read. `None` past the last sample.
    pub fn sample(&self, index: usize) -> Option<f64> {
        if index >= self.len() {
            return None;
        }
        Some(match &self.0 {
            Repr::Memory(values) => values[index],
            Repr::Stored(stored) => {
                let mut value = [0.0];
                match stored.read_into(index, &mut value) {
                    Ok(()) => value[0],
                    Err(_) => f64::NAN,
                }
            }
        })
    }

    /// Samples `range`: borrowed from memory, or read from a stored window for this caller
    /// alone and not kept. A range larger than memory can hold is refused, not aborted on; a
    /// reader of a long window reads it a block at a time instead ([`Self::blocks`]).
    pub fn read_range(&self, range: Range<usize>) -> Result<Cow<'_, [f64]>, ColumnReadFailure> {
        assert!(
            range.start <= range.end && range.end <= self.len(),
            "samples {range:?} of a window of {}",
            self.len()
        );
        match &self.0 {
            Repr::Memory(values) => Ok(Cow::Borrowed(&values[range])),
            Repr::Stored(stored) => {
                if range.len() == self.len() && range.len() > 1 {
                    column_work::whole_read();
                }
                let mut out = Vec::new();
                out.try_reserve_exact(range.len()).map_err(|_| {
                    stored.failed(format!(
                        "{} samples do not fit in this process's memory",
                        range.len()
                    ))
                })?;
                out.resize(range.len(), 0.0);
                stored.read_into(range.start, &mut out)?;
                Ok(Cow::Owned(out))
            }
        }
    }

    /// Every sample at once, for a computation over a whole sweep that needs them together (a
    /// network matrix, a margin search, a spectrum of a small sweep): borrowed from memory, or a
    /// stored window read for this caller alone and kept by no one. A stored window larger than
    /// the loaded-results budget ([`set_whole_read_budget`]) is refused with a typed failure; a
    /// reader of a long window reads it by range, block, tile or point instead.
    pub fn read_whole(&self) -> Result<Cow<'_, [f64]>, ColumnReadFailure> {
        if let Repr::Stored(stored) = &self.0 {
            let bytes = (self.len() as u64).saturating_mul(8);
            let budget = WHOLE_READ_BUDGET.load(std::sync::atomic::Ordering::Relaxed);
            if bytes > budget {
                return Err(stored.failed(format!(
                    "its {} samples ({bytes} bytes) are more than the {budget} bytes the loaded                      results may hold: read it by window",
                    self.len()
                )));
            }
        }
        self.read_range(0..self.len())
    }

    /// Samples `range` a block at a time: one slice of memory, or blocks of at most
    /// [`BLOCK_SAMPLES`] read from a stored window and kept by no one, so a pass over a window
    /// of any length holds one block. A block that cannot be read ends the pass with its
    /// failure.
    pub fn blocks(&self, range: impl RangeBounds<usize>) -> Blocks<'_> {
        let range = self.bounds(range);
        assert!(
            range.start <= range.end && range.end <= self.len(),
            "samples {range:?} of a window of {}",
            self.len()
        );
        if range.len() == self.len() && range.len() > 1 {
            match &self.0 {
                Repr::Memory(_) => column_work::view(self),
                Repr::Stored(_) => column_work::pass(),
            }
        }
        Blocks {
            values: self,
            next: range.start,
            end: range.end,
            failed: false,
        }
    }

    /// Each sample of `range` in order, read as [`Self::blocks`] reads them. The samples of a
    /// stored window that cannot be read are unavailable (NaN), and its failure goes to
    /// [`take_column_read_failures`] once.
    pub fn samples(&self, range: impl RangeBounds<usize>) -> Samples<'_> {
        let range = self.bounds(range);
        let remaining = range.len();
        Samples {
            blocks: self.blocks(range),
            block: Cow::Borrowed(&[]),
            at: 0,
            remaining,
        }
    }

    /// Whether both hold the same samples, an unavailable sample being the same absence as
    /// another: compared without reading when both are the same samples, otherwise a block at a
    /// time.
    pub fn same_samples(&self, other: &Self) -> bool {
        Self::ptr_eq(self, other)
            || (self.len() == other.len()
                && self
                    .samples(..)
                    .zip(other.samples(..))
                    .all(|(left, right)| left == right || (left.is_nan() && right.is_nan())))
    }

    /// The index of the first sample for which `before` is false, the samples being partitioned
    /// by it (every sample it holds for comes first), as [`slice::partition_point`] finds it: by
    /// bisection, which reads one row of a stored window per step.
    pub fn partition_point(&self, mut before: impl FnMut(f64) -> bool) -> usize {
        if let Repr::Memory(values) = &self.0 {
            return values.partition_point(|value| before(*value));
        }
        let (mut low, mut high) = (0, self.len());
        while low < high {
            let middle = low + (high - low) / 2;
            if self.sample(middle).is_some_and(&mut before) {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        low
    }

    /// The samples at `indices`, in their order: from memory, or each run of consecutive
    /// indices read as one range of a stored window, whose unreadable samples are NaN.
    ///
    /// # Panics
    ///
    /// When an index is past the last sample.
    pub fn gather(&self, indices: &[usize]) -> Vec<f64> {
        if let Repr::Memory(values) = &self.0 {
            return indices.iter().map(|&index| values[index]).collect();
        }
        let mut out = Vec::with_capacity(indices.len());
        let mut rest = indices;
        while let Some(&start) = rest.first() {
            let run = rest
                .iter()
                .enumerate()
                .take_while(|&(offset, &index)| index == start + offset)
                .count()
                .min(BLOCK_SAMPLES);
            match self.read_range(start..start + run) {
                Ok(samples) => out.extend_from_slice(&samples),
                Err(_) => out.extend(std::iter::repeat_n(f64::NAN, run)),
            }
            rest = &rest[run..];
        }
        out
    }

    /// Whether each sample is greater than the one before: a stored axis its run checked says
    /// so without being read; anything else is passed over a block at a time.
    pub fn ascends_strictly(&self) -> bool {
        self.is_ascending_axis() || pairs_all(self.samples(..), |before, after| before < after)
    }

    /// Whether no sample is less than the one before, passed over a block at a time.
    pub fn ascends(&self) -> bool {
        self.is_ascending_axis() || pairs_all(self.samples(..), |before, after| before <= after)
    }

    /// `range` as the indices it names of these samples.
    fn bounds(&self, range: impl RangeBounds<usize>) -> Range<usize> {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.len(),
        };
        start..end
    }

    /// The stored window these samples are, and the abscissa of its lead, when `x` is the window
    /// of its run's axis over the same rows.
    fn stored_against(&self, x: &Self) -> Option<(&Stored, Option<f64>)> {
        let (Repr::Stored(values), Repr::Stored(axis)) = (&self.0, &x.0) else {
            return None;
        };
        let lead_x = match (axis.window.lead, values.window.lead) {
            (WindowLead::None, WindowLead::None) => None,
            (WindowLead::Value(x), WindowLead::Value(_) | WindowLead::Interpolated(_)) => Some(x),
            _ => return None,
        };
        let same_rows =
            axis.window.first == values.window.first && axis.window.rows == values.window.rows;
        (same_rows && axis.source.ascending()).then_some((values.as_ref(), lead_x))
    }

    /// The tile of these samples over x in `x0 ..= x1` at `pixels` pixels, `x` being their run's
    /// axis over the same rows: read from the store's summaries, never the whole window. `None`
    /// when they are not such a window of a column that can tile. A window that cannot be read
    /// tiles as no rows, and its failure goes to [`take_column_read_failures`] once.
    pub fn tile_over(&self, x: &Self, x0: f64, x1: f64, pixels: usize) -> Option<ColumnTile> {
        if pixels == 0 || !(x0.is_finite() && x1.is_finite() && x0 <= x1) {
            return None;
        }
        let (stored, lead_x) = self.stored_against(x)?;
        stored.tile(lead_x, x0, x1, pixels)
    }

    /// [`Self::tile_over`] on a decade axis: `pixels` pixels evenly spaced in log10 x over
    /// `x0 ..= x1`, both positive. An envelope's pixel `i` spans log10 x from
    /// `log10(x0) + i * w` to `log10(x0) + (i + 1) * w`, `w` being the view's decades over
    /// `pixels`. Each decade is read as one linear tile fine enough that no log pixel in it is
    /// narrower than its pixels, and folded into the log pixels, so a view reads a few tiles of
    /// the store's summaries however long the run. A view whose rows all come back as rows is
    /// those rows. `None` when these samples cannot tile.
    pub fn log_tile_over(&self, x: &Self, x0: f64, x1: f64, pixels: usize) -> Option<ColumnTile> {
        if pixels == 0 || !(x0 > 0.0 && x1.is_finite() && x0 < x1) {
            return None;
        }
        let (stored, lead_x) = self.stored_against(x)?;
        let (l0, l1) = (x0.log10(), x1.log10());
        let width = (l1 - l0) / pixels as f64;
        let mut min = vec![f64::NAN; pixels];
        let mut max = vec![f64::NAN; pixels];
        let mut touched = vec![false; pixels];
        let take = |at: f64, value: f64, touched: &mut [bool], min: &mut [f64], max: &mut [f64]| {
            if !(at >= x0 && at <= x1) {
                return;
            }
            let pixel = (((at.log10() - l0) / width) as usize).min(pixels - 1);
            touched[pixel] = true;
            if !value.is_nan() {
                min[pixel] = if min[pixel] <= value {
                    min[pixel]
                } else {
                    value
                };
                max[pixel] = if max[pixel] >= value {
                    max[pixel]
                } else {
                    value
                };
            }
        };
        let mut rows: Option<(Vec<f64>, Vec<f64>)> = Some((Vec::new(), Vec::new()));
        let (mut before, mut after) = (None, None);
        let mut start = l0;
        let mut first_span = true;
        while start < l1 {
            let end = (start.floor() + 1.0).min(l1);
            let (a, b) = (10f64.powf(start), 10f64.powf(end));
            // The narrowest log pixel of the span is its first; a linear tile with pixels no
            // wider than it resolves every log pixel the span holds.
            let narrowest = a * (10f64.powf(width) - 1.0);
            let linear = (((b - a) / narrowest).ceil() as usize).clamp(1, 64 * pixels);
            match stored.tile(lead_x, a, b, linear)? {
                ColumnTile::Samples {
                    x: xs,
                    y: ys,
                    before: span_before,
                    after: span_after,
                } => {
                    if first_span {
                        before = span_before;
                    }
                    after = span_after;
                    for (&at, &value) in xs.iter().zip(&ys) {
                        take(at, value, &mut touched, &mut min, &mut max);
                    }
                    if let Some((all_x, all_y)) = &mut rows {
                        // Neighbouring decades share their edge row; it is kept once.
                        let shared = xs.first().zip(ys.first()).is_some_and(|(at, value)| {
                            all_x.last() == Some(at) && all_y.last() == Some(value)
                        });
                        let skip = usize::from(shared);
                        all_x.extend_from_slice(&xs[skip..]);
                        all_y.extend_from_slice(&ys[skip..]);
                    }
                }
                ColumnTile::Envelope {
                    min: span_min,
                    max: span_max,
                    touched: span_touched,
                } => {
                    rows = None;
                    let linear_width = (b - a) / span_min.len() as f64;
                    for pixel in (0..span_min.len()).filter(|&pixel| span_touched[pixel]) {
                        let at = a + (pixel as f64 + 0.5) * linear_width;
                        take(at, span_min[pixel], &mut touched, &mut min, &mut max);
                        take(at, span_max[pixel], &mut touched, &mut min, &mut max);
                    }
                }
            }
            start = end;
            first_span = false;
        }
        Some(match rows {
            Some((x, y)) => ColumnTile::Samples {
                x,
                y,
                before,
                after,
            },
            None => ColumnTile::Envelope { min, max, touched },
        })
    }

    /// The value at `at` by `interpolation`, as [`crate::sampling::sample_at_with`] reads it
    /// from the whole series, from the few rows around `at` alone, `x` being their run's axis
    /// over the same rows. `None` when these samples are not such a window. A window that cannot
    /// be read reads NaN.
    pub fn value_at(
        &self,
        x: &Self,
        at: f64,
        interpolation: crate::sampling::SampleInterpolation,
    ) -> Option<f64> {
        let (stored, lead_x) = self.stored_against(x)?;
        stored.value_at(lead_x, at, interpolation)
    }

    /// The least and greatest available value, and the least positive one, over x in `window`
    /// (all of the samples when `None`), `x` being their run's axis over the same rows, from a
    /// fine tile. The least positive value is a pixel's extreme, so where one pixel's rows cross
    /// zero it may stand above the least positive sample. `None` when these samples cannot tile.
    pub fn extent_over(&self, x: &Self, window: Option<(f64, f64)>) -> Option<ValueExtent> {
        let (x0, x1) = match window {
            Some(window) => window,
            None => x.ends()?,
        };
        let tile = self.tile_over(x, x0.min(x1), x0.max(x1), EXTENT_PIXELS)?;
        let mut extent = ValueExtent {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            min_positive: f64::INFINITY,
        };
        let mut take = |value: f64| {
            if value.is_finite() {
                extent.min = extent.min.min(value);
                extent.max = extent.max.max(value);
                if value > 0.0 {
                    extent.min_positive = extent.min_positive.min(value);
                }
            }
        };
        match tile {
            ColumnTile::Samples { y, .. } => y.into_iter().for_each(&mut take),
            ColumnTile::Envelope { min, max, .. } => {
                min.into_iter().chain(max).for_each(&mut take);
            }
        }
        Some(extent)
    }

    /// The values to change, held in memory from now on. A stored window is copied into memory
    /// a block at a time, as an edit of its samples needs; its unreadable samples are NaN.
    pub fn make_mut(&mut self) -> &mut Vec<f64> {
        if self.is_stored() {
            let copied: Vec<f64> = self.samples(..).collect();
            self.0 = Repr::Memory(Arc::new(copied));
        }
        match &mut self.0 {
            Repr::Memory(values) => Arc::make_mut(values),
            Repr::Stored(_) => unreachable!("a stored window was just copied into memory"),
        }
    }

    /// The samples, moved out when they are held in memory and nothing else shares them, copied
    /// otherwise; a stored window is copied a block at a time, its unreadable samples NaN.
    pub fn into_vec(self) -> Vec<f64> {
        match self.0 {
            Repr::Memory(values) => Arc::unwrap_or_clone(values),
            Repr::Stored(_) => self.samples(..).collect(),
        }
    }

    /// Whether both are the same samples, without comparing them: one allocation, or one window
    /// of one stored column.
    pub fn ptr_eq(left: &Self, right: &Self) -> bool {
        match (&left.0, &right.0) {
            (Repr::Memory(left), Repr::Memory(right)) => Arc::ptr_eq(left, right),
            (Repr::Stored(left), Repr::Stored(right)) => {
                Arc::ptr_eq(left, right)
                    || (std::ptr::addr_eq(Arc::as_ptr(&left.source), Arc::as_ptr(&right.source))
                        && left.window == right.window)
                    || (left.window == right.window
                        && left
                            .source
                            .identity()
                            .is_some_and(|identity| right.source.identity() == Some(identity)))
            }
            _ => false,
        }
    }
}

/// Whether `holds` holds for each consecutive pair of `samples`.
fn pairs_all(mut samples: impl Iterator<Item = f64>, holds: impl Fn(f64, f64) -> bool) -> bool {
    let Some(mut before) = samples.next() else {
        return true;
    };
    samples.all(|after| {
        let held = holds(before, after);
        before = after;
        held
    })
}

/// A pass over samples a block at a time: [`WaveformValues::blocks`].
pub struct Blocks<'a> {
    values: &'a WaveformValues,
    next: usize,
    end: usize,
    failed: bool,
}

impl Blocks<'_> {
    /// The index of the first sample the next block holds.
    pub fn position(&self) -> usize {
        self.next
    }
}

impl<'a> Iterator for Blocks<'a> {
    type Item = Result<Cow<'a, [f64]>, ColumnReadFailure>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.failed || self.next >= self.end {
            return None;
        }
        match &self.values.0 {
            Repr::Memory(values) => {
                let block = &values[self.next..self.end];
                self.next = self.end;
                Some(Ok(Cow::Borrowed(block)))
            }
            Repr::Stored(stored) => {
                let start = self.next;
                let mut block = vec![0.0; (self.end - start).min(BLOCK_SAMPLES)];
                self.next += block.len();
                match stored.read_into(start, &mut block) {
                    Ok(()) => Some(Ok(Cow::Owned(block))),
                    Err(failure) => {
                        self.failed = true;
                        Some(Err(failure))
                    }
                }
            }
        }
    }
}

/// Each sample of a range in order: [`WaveformValues::samples`].
pub struct Samples<'a> {
    blocks: Blocks<'a>,
    block: Cow<'a, [f64]>,
    at: usize,
    remaining: usize,
}

impl Iterator for Samples<'_> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        if self.at == self.block.len() {
            self.at = 0;
            self.block = match self.blocks.next() {
                Some(Ok(block)) => block,
                // An unreadable window's failure was kept when it failed; the rest of the
                // range is a gap.
                _ => Cow::Borrowed(&[]),
            };
            if self.block.is_empty() {
                return Some(f64::NAN);
            }
        }
        self.at += 1;
        Some(self.block[self.at - 1])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for Samples<'_> {}

/// Whole-column work, counted per thread in test builds so a frame gate can tell a reader that
/// took a column's whole values, passed over or read a stored window whole, from one that read a
/// tile, a few rows or the ends. In a shipped build the counting calls are empty.
pub mod column_work {
    #[cfg(feature = "test-support")]
    use super::Repr;
    use super::WaveformValues;

    /// What one thread did with whole columns.
    #[cfg(feature = "test-support")]
    #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
    pub struct ColumnWork {
        /// Readers given the whole values of an in-memory column.
        pub memory_views: u64,
        /// Passes over the whole of a stored window, a block at a time.
        pub stored_views: u64,
        /// Stored windows read whole into memory from their store.
        pub stored_reads: u64,
    }

    #[cfg(feature = "test-support")]
    thread_local! {
        static WORK: std::cell::Cell<ColumnWork> = const {
            std::cell::Cell::new(ColumnWork {
                memory_views: 0,
                stored_views: 0,
                stored_reads: 0,
            })
        };
    }

    #[cfg(feature = "test-support")]
    impl ColumnWork {
        /// The work counted so far on this thread.
        pub fn read() -> Self {
            WORK.with(std::cell::Cell::get)
        }

        /// The work counted since `self` was read.
        pub fn since(self) -> Self {
            let now = Self::read();
            Self {
                memory_views: now.memory_views - self.memory_views,
                stored_views: now.stored_views - self.stored_views,
                stored_reads: now.stored_reads - self.stored_reads,
            }
        }
    }

    #[inline]
    pub(super) fn view(_values: &WaveformValues) {
        #[cfg(feature = "test-support")]
        if matches!(_values.0, Repr::Memory(_)) {
            WORK.with(|work| {
                let mut counted = work.get();
                counted.memory_views += 1;
                work.set(counted);
            });
        }
    }

    #[inline]
    pub(super) fn pass() {
        #[cfg(feature = "test-support")]
        WORK.with(|work| {
            let mut counted = work.get();
            counted.stored_views += 1;
            work.set(counted);
        });
    }

    #[inline]
    pub(super) fn whole_read() {
        #[cfg(feature = "test-support")]
        WORK.with(|work| {
            let mut counted = work.get();
            counted.stored_reads += 1;
            work.set(counted);
        });
    }
}

impl Default for WaveformValues {
    fn default() -> Self {
        Self(Repr::Memory(Arc::default()))
    }
}

impl From<Vec<f64>> for WaveformValues {
    fn from(values: Vec<f64>) -> Self {
        Self(Repr::Memory(Arc::new(values)))
    }
}

impl From<Arc<Vec<f64>>> for WaveformValues {
    fn from(values: Arc<Vec<f64>>) -> Self {
        Self(Repr::Memory(values))
    }
}

impl FromIterator<f64> for WaveformValues {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Vec::from_iter(iter).into()
    }
}

/// Equal samples, compared value by value as `Vec<f64>` compares them (a stored window a block
/// at a time), unless both are the same samples.
impl PartialEq for WaveformValues {
    fn eq(&self, other: &Self) -> bool {
        // Lengths first: a stored window of another length is unequal without being read.
        Self::ptr_eq(self, other)
            || (self.len() == other.len() && self.samples(..).eq(other.samples(..)))
    }
}

impl PartialEq<Vec<f64>> for WaveformValues {
    fn eq(&self, other: &Vec<f64>) -> bool {
        *self == other[..]
    }
}

impl PartialEq<[f64]> for WaveformValues {
    fn eq(&self, other: &[f64]) -> bool {
        self.len() == other.len() && self.samples(..).eq(other.iter().copied())
    }
}

impl fmt::Debug for WaveformValues {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Repr::Memory(values) => fmt::Debug::fmt(&**values, formatter),
            Repr::Stored(stored) => formatter
                .debug_struct("StoredColumn")
                .field("name", &stored.source.name())
                .field("window", &stored.window)
                .finish(),
        }
    }
}

impl serde::Serialize for WaveformValues {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::{Error, SerializeSeq};
        // A stored window is written a block at a time, never held whole.
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for block in self.blocks(..) {
            for value in block.map_err(S::Error::custom)?.iter() {
                seq.serialize_element(value)?;
            }
        }
        seq.end()
    }
}

impl<'de> serde::Deserialize<'de> for WaveformValues {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<f64>::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests;
