//! The reads a view makes of a stored series, and the service that makes them off the frame.
//!
//! A host installs one [`ReadService`] (the desktop app's reader service, which reads on its own
//! thread into one cache under one memory budget). A view asks it for what it draws and draws what
//! has arrived; without a service, as in a command-line run or a test, a read is made as it is
//! asked.

use std::ops::Deref;
use std::sync::{Arc, OnceLock};

use super::{ColumnTile, WaveformValues};
use crate::sampling::SampleInterpolation;

/// What a view reads of one series.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Read {
    /// Samples `first .. first + count` of the series' window.
    Range { first: usize, count: usize },
    /// `pixels` pixels over x in `x0 ..= x1`, x being the run's axis.
    Tile { x0: f64, x1: f64, pixels: usize },
    /// `pixels` pixels evenly spaced in log10 x over `x0 ..= x1`, both positive.
    LogTile { x0: f64, x1: f64, pixels: usize },
    /// The value at `x`, interpolated as the series is drawn.
    Point {
        x: f64,
        interpolation: SampleInterpolation,
    },
}

/// What a read gave.
#[derive(Debug, Clone, PartialEq)]
pub enum ReadValue {
    Samples(Arc<[f64]>),
    Tile(Arc<ColumnTile>),
    Point(Option<f64>),
    /// Why it could not be read: the store failed, or the read is larger than the budget.
    Failed(String),
}

impl ReadValue {
    /// The bytes a cache counts for it.
    pub fn bytes(&self) -> u64 {
        let len = match self {
            Self::Samples(samples) => samples.len() * 8,
            Self::Tile(tile) => match tile.as_ref() {
                ColumnTile::Samples { x, y, .. } => (x.len() + y.len()) * 8 + 32,
                ColumnTile::Envelope { min, max, touched } => {
                    (min.len() + max.len()) * 8 + touched.len()
                }
            },
            Self::Point(_) => 16,
            Self::Failed(reason) => reason.len(),
        };
        len as u64 + 64
    }
}

/// Make `read` of `values`, a series whose run's axis is `x`, now: what a service's worker does
/// with a queued read.
pub fn perform(values: &WaveformValues, x: &WaveformValues, read: Read) -> ReadValue {
    let untileable = || {
        ReadValue::Failed(format!(
            "'{}' cannot be drawn in tiles over its run's axis",
            values
                .stored_window()
                .map_or("", |(source, _)| source.name())
        ))
    };
    match read {
        Read::Range { first, count } => {
            let end = first.saturating_add(count).min(values.len());
            match values.read_range(first.min(end)..end) {
                Ok(samples) => ReadValue::Samples(samples.into_owned().into()),
                Err(failure) => ReadValue::Failed(failure.to_string()),
            }
        }
        Read::Tile { x0, x1, pixels } => match values.tile_over(x, x0, x1, pixels) {
            Some(tile) => ReadValue::Tile(Arc::new(tile)),
            None => untileable(),
        },
        Read::LogTile { x0, x1, pixels } => match values.log_tile_over(x, x0, x1, pixels) {
            Some(tile) => ReadValue::Tile(Arc::new(tile)),
            None => untileable(),
        },
        Read::Point {
            x: at,
            interpolation,
        } => ReadValue::Point(values.value_at(x, at, interpolation)),
    }
}

/// A service that reads stored series off the frame.
pub trait ReadService: Send + Sync {
    /// Ask for `read` of `values`, a series of run `run` (0 when the asker names none) whose
    /// axis is `x`. What the service holds, or what memory answers at once, comes back now; a
    /// stored read is queued and comes back from a later ask once it has arrived.
    fn request(
        &self,
        run: u64,
        values: &WaveformValues,
        x: &WaveformValues,
        read: Read,
    ) -> Option<ReadValue>;
}

static SERVICE: OnceLock<&'static dyn ReadService> = OnceLock::new();

/// Make `service` the process's read service. The first one installed stays.
pub fn install_read_service(service: &'static dyn ReadService) {
    let _ = SERVICE.set(service);
}

/// The process's read service, if a host installed one.
pub fn read_service() -> Option<&'static dyn ReadService> {
    SERVICE.get().copied()
}

/// Every sample of a series, held as a slice a view can keep across frames: the series' own
/// samples when they are in memory, or a whole read a service shares from its cache. Cloning
/// shares the samples.
#[derive(Debug, Clone)]
pub struct HeldColumn(Held);

#[derive(Debug, Clone)]
enum Held {
    /// Samples in memory; [`HeldColumn`] makes this only of samples whose slice is held.
    Memory(WaveformValues),
    Shared(Arc<[f64]>),
}

impl Deref for HeldColumn {
    type Target = [f64];

    fn deref(&self) -> &[f64] {
        match &self.0 {
            Held::Memory(values) => values.in_memory().unwrap_or_default(),
            Held::Shared(samples) => samples,
        }
    }
}

impl Default for HeldColumn {
    fn default() -> Self {
        Self(Held::Shared(Arc::from([])))
    }
}

impl From<Vec<f64>> for HeldColumn {
    fn from(samples: Vec<f64>) -> Self {
        Self(Held::Memory(samples.into()))
    }
}

impl From<Arc<[f64]>> for HeldColumn {
    fn from(samples: Arc<[f64]>) -> Self {
        Self(Held::Shared(samples))
    }
}

/// Samples a view reads as one slice: borrowed from evidence that holds them as one, or a held
/// column.
#[derive(Debug, Clone)]
pub enum SampleSlice<'a> {
    Borrowed(&'a [f64]),
    Held(HeldColumn),
}

impl Deref for SampleSlice<'_> {
    type Target = [f64];

    fn deref(&self) -> &[f64] {
        match self {
            Self::Borrowed(samples) => samples,
            Self::Held(samples) => samples,
        }
    }
}

impl<'a> From<&'a [f64]> for SampleSlice<'a> {
    fn from(samples: &'a [f64]) -> Self {
        Self::Borrowed(samples)
    }
}

impl From<HeldColumn> for SampleSlice<'_> {
    fn from(samples: HeldColumn) -> Self {
        Self::Held(samples)
    }
}

/// What a view has of a read it asked for.
#[derive(Debug, Clone)]
pub enum Fetched<T> {
    /// It has arrived.
    Ready(T),
    /// It is being read; a later frame has it.
    Pending,
    /// It cannot be read: the store failed, or it is larger than the loaded results may hold.
    Refused(String),
}

impl<T> Fetched<T> {
    /// The read, if it has arrived.
    pub fn ready(self) -> Option<T> {
        match self {
            Self::Ready(value) => Some(value),
            Self::Pending | Self::Refused(_) => None,
        }
    }
}

impl WaveformValues {
    /// The value at `at` by `interpolation`, as [`crate::sampling::sample_at_with`] reads it
    /// from the whole series, `x` being its run's axis over the same rows: from memory; from the
    /// few rows around `at` of a stored window; and otherwise from both series whole, read by
    /// the installed service off the frame, NaN until they have arrived.
    pub fn sample_at(&self, x: &Self, at: f64, interpolation: SampleInterpolation) -> f64 {
        if let (Some(xs), Some(ys)) = (x.in_memory(), self.in_memory()) {
            return crate::sampling::sample_at_with(xs, ys, at, interpolation);
        }
        if let Some(value) = self.value_at(x, at, interpolation) {
            return value;
        }
        match (x.fetch_whole(), self.fetch_whole()) {
            (Fetched::Ready(xs), Fetched::Ready(ys)) => {
                crate::sampling::sample_at_with(&xs, &ys, at, interpolation)
            }
            _ => f64::NAN,
        }
    }

    /// Every sample of the window, for a view that shows a whole series at once (a polar or
    /// Smith chart, a matrix, a table): borrowed when the samples are in memory; read by the
    /// installed [`ReadService`] off the frame, and held under its budget, when they are
    /// stored; read now, refused past the whole-read budget, when no service is installed.
    /// A stored series larger than the budget is refused, never read.
    pub fn fetch_whole(&self) -> Fetched<HeldColumn> {
        if self.in_memory().is_some() {
            return Fetched::Ready(HeldColumn(Held::Memory(self.clone())));
        }
        let Some(service) = read_service() else {
            return match self.read_whole() {
                Ok(samples) => Fetched::Ready(samples.into_owned().into()),
                Err(failure) => Fetched::Refused(failure.to_string()),
            };
        };
        let read = Read::Range {
            first: 0,
            count: self.len(),
        };
        match service.request(0, self, self, read) {
            None => Fetched::Pending,
            Some(ReadValue::Samples(samples)) => Fetched::Ready(samples.into()),
            Some(ReadValue::Failed(reason)) => Fetched::Refused(reason),
            Some(ReadValue::Tile(_) | ReadValue::Point(_)) => {
                Fetched::Refused("the reader answered a range with another read".to_owned())
            }
        }
    }
}
