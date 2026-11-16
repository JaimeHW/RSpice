//! A waveform's samples: in memory, or a window of a column a results store holds.
//!
//! A transient written to a results store keeps its samples on disk. Its waveforms hold a window
//! of each stored column instead of the values, and read the window the first time something asks
//! for the values: every clone shares that one read, and a reader that asks only how many samples
//! there are reads nothing. A column stored in memory is shared the same way, so a run's axis is
//! one allocation however many waveforms share it.

use std::fmt;
use std::ops::Deref;
use std::sync::{Arc, Mutex, OnceLock};

/// A column of samples kept outside memory and read on demand: one signal of a run in a results
/// store.
pub trait ColumnSource: Send + Sync {
    /// The column's name, as a failure to read it names it.
    fn name(&self) -> &str;

    /// The rows the column holds.
    fn rows(&self) -> u64;

    /// Read rows `first .. first + out.len()` into `out`.
    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String>;
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
    /// Why it could not be read.
    pub cause: String,
}

impl fmt::Display for ColumnReadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "the stored samples of '{}' could not be read: {}",
            self.column, self.cause
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
                UNREPORTED_FAILURES
                    .lock()
                    .unwrap_or_else(|p| p.into_inner())
                    .push(failure.clone());
                let _ = self.failure.set(failure);
                vec![f64::NAN; self.len()]
            })
        })
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
