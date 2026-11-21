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

    /// What the service holds for `read` of `values` in run `run` now, without asking for it:
    /// how a view finds a coarser tile to draw while the one it asked for is read.
    fn held(&self, run: u64, values: &WaveformValues, read: Read) -> Option<ReadValue> {
        let _ = (run, values, read);
        None
    }

    /// Wait until every read asked so far has arrived: how work off the frame that needs a read
    /// waits for it. A service that answers every read as it is asked has nothing to wait for.
    fn settle(&self) {}
}

static SERVICE: OnceLock<&'static dyn ReadService> = OnceLock::new();

#[cfg(feature = "test-support")]
thread_local! {
    /// The service a test reads through on this thread, in place of the process's.
    static THREAD_SERVICE: std::cell::Cell<Option<&'static dyn ReadService>> =
        const { std::cell::Cell::new(None) };
}

/// Make `service` the process's read service. The first one installed stays.
pub fn install_read_service(service: &'static dyn ReadService) {
    let _ = SERVICE.set(service);
}

/// The process's read service, if a host installed one.
pub fn read_service() -> Option<&'static dyn ReadService> {
    #[cfg(feature = "test-support")]
    if let Some(service) = THREAD_SERVICE.get() {
        return Some(service);
    }
    SERVICE.get().copied()
}

/// Run `body` reading every stored series through `service` on this thread, as a test stands a
/// reader service of its own in for the app's without installing one for the whole process.
#[cfg(feature = "test-support")]
pub fn read_through<R>(service: &'static dyn ReadService, body: impl FnOnce() -> R) -> R {
    struct Restore(Option<&'static dyn ReadService>);
    impl Drop for Restore {
        fn drop(&mut self) {
            THREAD_SERVICE.set(self.0);
        }
    }
    let _restore = Restore(THREAD_SERVICE.replace(Some(service)));
    body()
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

    /// The read, or why there is none: [`BEING_READ`] while it is being read, or why it cannot
    /// be read.
    pub fn into_result(self) -> Result<T, String> {
        match self {
            Self::Ready(value) => Ok(value),
            Self::Pending => Err(BEING_READ.to_owned()),
            Self::Refused(reason) => Err(reason),
        }
    }
}

/// The cause work that needs a whole series gives, on the thread that draws the window, while
/// the reader service is still reading the series: a later frame finds it read, so the answer is
/// not kept. [`super::is_pending_read`] knows it.
pub const BEING_READ: &str = "its samples are still being read";

thread_local! {
    /// Whether this thread draws the window, and so never waits for a read.
    static DRAWS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Make this thread one that never waits for the reader service: the thread that draws the
/// window. [`WaveformValues::fetch_whole_or_wait`] answers `Pending` here, and waits anywhere
/// else.
pub fn never_wait_for_reads_here() {
    DRAWS.with(|draws| draws.set(true));
}

/// A tile a view draws: the store's pixels over an x window that holds the view, and whether it
/// is the tile of the view's own level or a coarser one drawn while that one is read.
#[derive(Debug, Clone)]
pub struct ViewTile {
    pub tile: Arc<ColumnTile>,
    /// The x window the tile's pixels span, evenly in log10 x when `log`.
    pub x0: f64,
    pub x1: f64,
    pub log: bool,
    /// Whether this is the tile the view asked for; a coarser one stands in until it arrives.
    pub exact: bool,
}

impl ViewTile {
    /// The x at `fraction` of the window, in its scale.
    pub fn x_at(&self, fraction: f64) -> f64 {
        if self.log {
            let (l0, l1) = (self.x0.log10(), self.x1.log10());
            10f64.powf(l0 + fraction * (l1 - l0))
        } else {
            self.x0 + fraction * (self.x1 - self.x0)
        }
    }

    /// The middle of pixel `pixel` of `pixels` on the run's axis.
    pub fn middle(&self, pixel: usize, pixels: usize) -> f64 {
        self.x_at((pixel as f64 + 0.5) / pixels as f64)
    }

    /// The x where pixel `pixel` of `pixels` begins.
    pub fn edge(&self, pixel: usize, pixels: usize) -> f64 {
        self.x_at(pixel as f64 / pixels as f64)
    }
}

/// How many times a view's columns a tile of the pyramid reads: a view at least half as wide as
/// its level's tile, and at most a quarter of the pair it reads, then has at least one pixel of
/// the tile in every column.
const TILE_OVERSAMPLE: usize = 4;

/// The window of level `level` of the tile pyramid that holds the view `u0 ..= u1`, in the
/// view's scale (log10 x on a decade axis): the pair of tiles `2^level` wide, starting at a
/// multiple of that width, that begins at or before `u0`. `None` when the level's width is not
/// a finite non-zero number or the pair does not hold the view, as at the end of the floats.
fn pyramid_window(u0: f64, u1: f64, level: i32) -> Option<(f64, f64)> {
    let width = 2f64.powi(level);
    if !(width.is_finite() && width > 0.0) {
        return None;
    }
    let first = (u0 / width).floor() * width;
    let last = 2f64.mul_add(width, first);
    (first <= u0 && u1 <= last && last.is_finite()).then_some((first, last))
}

/// The read of a tile of `pixels` pixels over `x0 ..= x1`, evenly in log10 x when `log`.
fn tile_read(log: bool, x0: f64, x1: f64, pixels: usize) -> Read {
    if log {
        Read::LogTile { x0, x1, pixels }
    } else {
        Read::Tile { x0, x1, pixels }
    }
}

/// What a view has of a tile read over `x0 ..= x1`.
fn view_tile(
    value: Option<ReadValue>,
    log: bool,
    x0: f64,
    x1: f64,
    exact: bool,
) -> Fetched<ViewTile> {
    match value {
        None => Fetched::Pending,
        Some(ReadValue::Tile(tile)) => Fetched::Ready(ViewTile {
            tile,
            x0,
            x1,
            log,
            exact,
        }),
        Some(ReadValue::Failed(reason)) => Fetched::Refused(reason),
        Some(ReadValue::Samples(_) | ReadValue::Point(_)) => {
            Fetched::Refused("the reader answered a tile with another read".to_owned())
        }
    }
}

/// The narrowest level of the tile pyramid at least as wide as the view `u0 ..= u1`.
fn view_level(u0: f64, u1: f64) -> Option<i32> {
    let span = u1 - u0;
    if !(span > 0.0 && span.is_finite()) {
        return None;
    }
    let mut level = span.log2().ceil() as i32;
    if 2f64.powi(level) < span {
        level += 1;
    }
    Some(level)
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

    /// The tile a view of `x0 ..= x1` at `columns` columns draws (evenly in log10 x when `log`),
    /// `x` being these samples' run axis over the same rows. Without a service, or for samples
    /// in memory, the view's own tile is read now. The installed service reads views in tiles
    /// of a pyramid over the axis, so a pan or zoom within a tile pair draws the pair already
    /// held: the view's tile comes back once it has arrived, and until then the nearest coarser
    /// level the service holds stands in, the tile of the whole axis being asked for so one
    /// always will. `Refused` when these samples cannot be tiled, so the view must read them
    /// another way.
    pub fn fetch_tile(
        &self,
        x: &Self,
        log: bool,
        x0: f64,
        x1: f64,
        columns: usize,
    ) -> Fetched<ViewTile> {
        self.fetch_tile_from(read_service(), x, log, x0, x1, columns)
    }

    /// [`Self::fetch_tile`] through `service`.
    fn fetch_tile_from(
        &self,
        service: Option<&dyn ReadService>,
        x: &Self,
        log: bool,
        x0: f64,
        x1: f64,
        columns: usize,
    ) -> Fetched<ViewTile> {
        let service = service.filter(|_| self.in_memory().is_none());
        let Some(service) = service else {
            let read = tile_read(log, x0, x1, columns);
            return view_tile(Some(perform(self, x, read)), log, x0, x1, true);
        };
        let to_u = |at: f64| if log { at.log10() } else { at };
        let to_x = |u: f64| if log { 10f64.powf(u) } else { u };
        let (u0, u1) = (to_u(x0), to_u(x1));
        let pixels = columns.saturating_mul(TILE_OVERSAMPLE);
        let level = view_level(u0, u1);
        let Some((level, (w0, w1))) =
            level.and_then(|level| Some((level, pyramid_window(u0, u1, level)?)))
        else {
            // A view the pyramid cannot hold is read as it is.
            let read = tile_read(log, x0, x1, columns);
            return view_tile(service.request(0, self, x, read), log, x0, x1, true);
        };
        let (w0, w1) = (to_x(w0), to_x(w1));
        let read = tile_read(log, w0, w1, pixels);
        if let Some(value) = service.request(0, self, x, read) {
            return view_tile(Some(value), log, w0, w1, true);
        }
        // The axis, as far as the view's scale can show it: a decade axis from zero shows from
        // the view's own first decade.
        let (first, last) = x.ends().map_or((u0, u1), |(first, last)| {
            let (first, last) = (to_u(first), to_u(last));
            (
                if first.is_finite() { first.min(u0) } else { u0 },
                if last.is_finite() { last.max(u1) } else { u1 },
            )
        });
        // Coarser levels up to the first as wide as the axis, whose tile the axis' own stands for.
        let mut parent = level;
        while let Some(next) = parent.checked_add(1) {
            parent = next;
            if 2f64.powi(parent - 1) >= last - first {
                break;
            }
            let Some((p0, p1)) = pyramid_window(u0, u1, parent) else {
                break;
            };
            let (p0, p1) = (to_x(p0), to_x(p1));
            if let Some(value) = service.held(0, self, tile_read(log, p0, p1, pixels)) {
                return view_tile(Some(value), log, p0, p1, false);
            }
        }
        match self.fetch_axis_tile_from(Some(service), x, log, pixels) {
            Fetched::Ready(tile) => Fetched::Ready(ViewTile {
                exact: false,
                ..tile
            }),
            Fetched::Pending | Fetched::Refused(_) => Fetched::Pending,
        }
    }

    /// [`Self::fetch_axis_tile`] through `service`.
    fn fetch_axis_tile_from(
        &self,
        service: Option<&dyn ReadService>,
        x: &Self,
        log: bool,
        pixels: usize,
    ) -> Fetched<ViewTile> {
        let Some((x0, x1)) = x.ends().filter(|(x0, x1)| x1 > x0) else {
            return Fetched::Refused("the axis spans no range".to_owned());
        };
        if log && x0 <= 0.0 {
            return Fetched::Refused("a decade axis from zero has no tile".to_owned());
        }
        let read = tile_read(log, x0, x1, pixels);
        let value = match service.filter(|_| self.in_memory().is_none()) {
            Some(service) => service.request(0, self, x, read),
            None => Some(perform(self, x, read)),
        };
        view_tile(value, log, x0, x1, true)
    }

    /// The tile of these samples across their run's whole axis `x` at `pixels` pixels (evenly in
    /// log10 x when `log`), as an overview draws it: read now without a service or for samples
    /// in memory, and by the installed service off the frame otherwise. `Refused` when they
    /// cannot be tiled, as on a decade axis from zero.
    pub fn fetch_axis_tile(&self, x: &Self, log: bool, pixels: usize) -> Fetched<ViewTile> {
        self.fetch_axis_tile_from(read_service(), x, log, pixels)
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

    /// Every sample of the window, for work off the frame that needs the whole series at once
    /// (an eye fold): asked of the installed [`ReadService`], as [`Self::fetch_whole`] asks, and
    /// waited for, so the read is held under the service's one budget and shared with the views
    /// that draw the same series. A read the service let go of again before it was taken is
    /// made here, under the same budget. Never call it on the frame.
    pub fn fetch_whole_waiting(&self) -> Result<HeldColumn, String> {
        match self.fetch_whole() {
            Fetched::Ready(samples) => return Ok(samples),
            Fetched::Refused(reason) => return Err(reason),
            Fetched::Pending => {}
        }
        if let Some(service) = read_service() {
            service.settle();
        }
        match self.fetch_whole() {
            Fetched::Ready(samples) => Ok(samples),
            Fetched::Refused(reason) => Err(reason),
            Fetched::Pending => match self.read_whole() {
                Ok(samples) => Ok(samples.into_owned().into()),
                Err(failure) => Err(failure.to_string()),
            },
        }
    }

    /// Every sample of the window, for work that needs the whole series at once and runs both on
    /// the frame and off it (a Bode summary, a two-port, a validation): on the thread that draws
    /// the window ([`never_wait_for_reads_here`]) as [`Self::fetch_whole`] asks, `Pending` until
    /// the reader service has read it; anywhere else waited for, as
    /// [`Self::fetch_whole_waiting`] waits.
    pub fn fetch_whole_or_wait(&self) -> Fetched<HeldColumn> {
        if DRAWS.with(std::cell::Cell::get) {
            return self.fetch_whole();
        }
        match self.fetch_whole_waiting() {
            Ok(samples) => Fetched::Ready(samples),
            Err(reason) => Fetched::Refused(reason),
        }
    }
}
