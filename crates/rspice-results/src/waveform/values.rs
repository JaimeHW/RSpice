//! A waveform's samples: in memory, or a window of a column a results store holds.
//!
//! A transient written to a results store keeps its samples on disk. Its waveforms hold a window
//! of each stored column instead of the values, and read the window the first time something asks
//! for the values: every clone shares that one read, and a reader that asks only how many samples
//! there are reads nothing. A column stored in memory is shared the same way, so a run's axis is
//! one allocation however many waveforms share it.
//!
//! What a plot needs of a stored window never reads it whole: its tile of a view (per pixel, the
//! least and greatest value of the rows in it), the rows around a cursor, its extent, and the ends
//! of a time axis come from the store's summaries and the few rows they name.

use std::fmt;
use std::ops::{Deref, Range};
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

/// Failures of reads that had no caller to return them to, waiting for diagnostics to collect.
static UNREPORTED_FAILURES: Mutex<Vec<ColumnReadFailure>> = Mutex::new(Vec::new());

/// Take every stored column read that failed behind a waveform's values since the last call, so
/// a host can show each one. Each column fails at most once: its window keeps the gap it showed.
pub fn take_column_read_failures() -> Vec<ColumnReadFailure> {
    std::mem::take(
        &mut *UNREPORTED_FAILURES
            .lock()
            .unwrap_or_else(|p| p.into_inner()),
    )
}

/// A waveform's samples.
///
/// It dereferences to the values, reading a stored window the first time; a window that cannot
/// be read shows as unavailable samples (NaN) and its failure goes to
/// [`take_column_read_failures`]. [`Self::try_values`] returns the failure instead, and
/// [`Self::len`] never reads.
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
    values: OnceLock<Vec<f64>>,
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

    fn read(&self) -> Result<Vec<f64>, ColumnReadFailure> {
        let StoredWindow { first, rows, lead } = self.window;
        let failed = |cause: String| ColumnReadFailure {
            column: self.source.name().to_owned(),
            store: self.source.store(),
            cause,
        };
        let rows = usize::try_from(rows)
            .map_err(|_| failed("the window holds more rows than this host can address".into()))?;
        let mut values = vec![0.0; self.len()];
        match lead {
            WindowLead::None => self.source.read(first, &mut values).map_err(failed)?,
            WindowLead::Value(value) => {
                values[0] = value;
                self.source.read(first, &mut values[1..]).map_err(failed)?;
            }
            WindowLead::Interpolated(fraction) => {
                // The row before the first sits where the lead goes; it is replaced by the value
                // between it and the first, with the arithmetic the in-memory window uses.
                self.source.read(first - 1, &mut values).map_err(failed)?;
                if rows > 0 {
                    values[0] += fraction * (values[1] - values[0]);
                }
            }
        }
        Ok(values)
    }

    fn values(&self) -> &Vec<f64> {
        self.values.get_or_init(|| {
            self.read().unwrap_or_else(|failure| {
                self.fail(failure);
                vec![f64::NAN; self.len()]
            })
        })
    }

    /// Keep the window's first failure, and hand it to diagnostics once.
    fn fail(&self, failure: ColumnReadFailure) {
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

    /// What the window holds before its first row, read once for an interpolated lead.
    fn lead(&self) -> Option<f64> {
        match self.window.lead {
            WindowLead::None => None,
            WindowLead::Value(value) => Some(value),
            WindowLead::Interpolated(fraction) => Some(*self.lead_value.get_or_init(|| {
                // As [`Self::read`] makes it: the row before the first, moved `fraction` of the
                // way to the first.
                let count = if self.window.rows > 0 { 2 } else { 1 };
                let mut pair = [0.0; 2];
                match self.source.read(self.window.first - 1, &mut pair[..count]) {
                    Ok(()) if count == 2 => pair[0] + fraction * (pair[1] - pair[0]),
                    Ok(()) => pair[0],
                    Err(cause) => {
                        self.fail(self.failed(cause));
                        f64::NAN
                    }
                }
            })),
        }
    }

    /// The first and last sample, reading only those.
    fn ends(&self) -> Option<(f64, f64)> {
        *self.ends.get_or_init(|| {
            if let Some(values) = self.values.get() {
                return Some((*values.first()?, *values.last()?));
            }
            let row = |row: u64| {
                let mut value = [0.0];
                match self.source.read(row, &mut value) {
                    Ok(()) => value[0],
                    Err(cause) => {
                        self.fail(self.failed(cause));
                        f64::NAN
                    }
                }
            };
            let last = match self.window.rows {
                0 => self.lead()?,
                rows => row(self.window.first + rows - 1),
            };
            let first = match self.lead() {
                Some(lead) => lead,
                None => row(self.window.first),
            };
            Some((first, last))
        })
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

    fn try_values(&self) -> Result<&Vec<f64>, ColumnReadFailure> {
        if let Some(failure) = self.failure.get() {
            return Err(failure.clone());
        }
        if let Some(values) = self.values.get() {
            return Ok(values);
        }
        let read = self.read()?;
        // Another reader may have read it first; both read the same rows.
        Ok(self.values.get_or_init(|| read))
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
            values: OnceLock::new(),
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
    /// when they are not such a window of a column that can tile; read the values instead. A
    /// window that cannot be read tiles as no rows, and its failure goes to
    /// [`take_column_read_failures`] once.
    pub fn tile_over(&self, x: &Self, x0: f64, x1: f64, pixels: usize) -> Option<ColumnTile> {
        if pixels == 0 || !(x0.is_finite() && x1.is_finite() && x0 <= x1) {
            return None;
        }
        let (stored, lead_x) = self.stored_against(x)?;
        stored.tile(lead_x, x0, x1, pixels)
    }

    /// The value at `at` by `interpolation`, as [`crate::sampling::sample_at_with`] reads it
    /// from the whole series, from the few rows around `at` alone, `x` being their run's axis
    /// over the same rows. `None` when these samples are not such a window; read the values
    /// instead. A window that cannot be read reads NaN.
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

    /// The values, read from a stored window the first time, or why it could not be read.
    pub fn try_values(&self) -> Result<&Vec<f64>, ColumnReadFailure> {
        match &self.0 {
            Repr::Memory(values) => Ok(values),
            Repr::Stored(stored) => stored.try_values(),
        }
    }

    /// The values to change, held in memory from now on.
    pub fn make_mut(&mut self) -> &mut Vec<f64> {
        if let Repr::Stored(stored) = &self.0 {
            self.0 = Repr::Memory(Arc::new(stored.values().clone()));
        }
        match &mut self.0 {
            Repr::Memory(values) => Arc::make_mut(values),
            Repr::Stored(_) => unreachable!("a stored window was just read into memory"),
        }
    }

    /// The values, moved out when nothing else shares them.
    pub fn into_vec(self) -> Vec<f64> {
        match self.0 {
            Repr::Memory(values) => Arc::unwrap_or_clone(values),
            Repr::Stored(stored) => match Arc::try_unwrap(stored) {
                Ok(stored) => {
                    stored.values();
                    stored.values.into_inner().unwrap_or_default()
                }
                Err(shared) => shared.values().clone(),
            },
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
            }
            _ => false,
        }
    }
}

impl Deref for WaveformValues {
    type Target = Vec<f64>;

    fn deref(&self) -> &Vec<f64> {
        match &self.0 {
            Repr::Memory(values) => values,
            Repr::Stored(stored) => stored.values(),
        }
    }
}

impl AsRef<Vec<f64>> for WaveformValues {
    fn as_ref(&self) -> &Vec<f64> {
        self
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

/// Equal samples, compared value by value as `Vec<f64>` compares them, unless both are the same
/// samples.
impl PartialEq for WaveformValues {
    fn eq(&self, other: &Self) -> bool {
        Self::ptr_eq(self, other) || **self == **other
    }
}

impl PartialEq<Vec<f64>> for WaveformValues {
    fn eq(&self, other: &Vec<f64>) -> bool {
        **self == *other
    }
}

impl PartialEq<[f64]> for WaveformValues {
    fn eq(&self, other: &[f64]) -> bool {
        self.as_slice() == other
    }
}

impl<'a> IntoIterator for &'a WaveformValues {
    type Item = &'a f64;
    type IntoIter = std::slice::Iter<'a, f64>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
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
        let values = self.try_values().map_err(serde::ser::Error::custom)?;
        serde::Serialize::serialize(values, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for WaveformValues {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Vec::<f64>::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests;
