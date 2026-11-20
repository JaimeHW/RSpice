//! The reader service: stored columns are read off the UI thread into one cache that every loaded
//! run shares under one memory budget.
//!
//! A view asks for what it draws with [`RunReader::request`]: a range of samples, a pixel tile
//! over an x window, or the value at an x. It draws what [`RunReader::get`] holds now; a read
//! that has not arrived is drawn when it does, as [`RunReader::revision`] advances and the waker
//! asks for a frame. The cache never holds more than its budget: when a read would pass it, the
//! reads of runs not on screen ([`RunReader::show`]) go first, the least recently drawn first,
//! then the least recently drawn of the rest. A read larger than the whole budget is refused
//! with a reason, never kept.
//!
//! Samples held in memory are not the service's: a read of them is answered as it is asked.
//! The browser build has no thread to read on, so it reads as each request is made.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use rspice_results::sampling::SampleInterpolation;
use rspice_results::waveform::{ColumnTile, WaveformValues};

/// The memory budget of loaded results a process starts with: 1 GiB.
pub const DEFAULT_BUDGET: u64 = 1 << 30;

/// What a view reads of one series.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Read {
    /// Samples `first .. first + count` of the series' window.
    Range { first: usize, count: usize },
    /// `pixels` pixels over x in `x0 ..= x1`, x being the run's axis.
    Tile { x0: f64, x1: f64, pixels: usize },
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
    /// The bytes the cache counts for it.
    fn bytes(&self) -> u64 {
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

/// One read the cache keeps: the run, the series' column and window, and the read.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReadKey {
    pub run: u64,
    /// The store's logical digest and the column's index in it, and the window's first row and
    /// rows: a series with the same column holds the same samples.
    column: ([u8; 32], u64, u64, u64),
    /// The read's shape, its numbers bit for bit.
    read: [u64; 4],
}

impl ReadKey {
    /// The key of `read` of `values` in run `run`; `None` for samples held in memory or in a
    /// store that states no digest, which the cache does not keep.
    pub fn of(run: u64, values: &WaveformValues, read: Read) -> Option<Self> {
        let (source, window) = values.stored_window()?;
        let (digest, column) = source.identity()?;
        let read = match read {
            Read::Range { first, count } => [0, first as u64, count as u64, 0],
            Read::Tile { x0, x1, pixels } => [1, x0.to_bits(), x1.to_bits(), pixels as u64],
            Read::Point { x, interpolation } => [2, x.to_bits(), interpolation as u64, 0],
        };
        Some(Self {
            run,
            column: (digest, column, window.first, window.rows),
            read,
        })
    }
}

fn perform(values: &WaveformValues, x: &WaveformValues, read: Read) -> ReadValue {
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
            None => ReadValue::Failed(format!(
                "'{}' cannot be drawn in tiles over its run's axis",
                values
                    .stored_window()
                    .map_or("", |(source, _)| source.name())
            )),
        },
        Read::Point {
            x: at,
            interpolation,
        } => ReadValue::Point(values.value_at(x, at, interpolation)),
    }
}

struct Entry {
    value: ReadValue,
    bytes: u64,
    /// When it was last drawn, in the cache's ticks.
    used: u64,
}

struct Job {
    key: ReadKey,
    values: WaveformValues,
    x: WaveformValues,
    read: Read,
}

#[derive(Default)]
struct Cache {
    budget: u64,
    held: u64,
    tick: u64,
    revision: u64,
    entries: HashMap<ReadKey, Entry>,
    queued: HashSet<ReadKey>,
    jobs: VecDeque<Job>,
    shown: HashSet<u64>,
    /// Whether the worker is reading a job it took.
    reading: bool,
}

impl Cache {
    /// Keep `value` under `key`, letting go of the reads the budget gives way to first.
    fn keep(&mut self, key: ReadKey, mut value: ReadValue) {
        let mut bytes = value.bytes();
        if bytes > self.budget {
            value = ReadValue::Failed(format!(
                "the read needs {bytes} bytes, more than the {} the loaded results may hold: \
                 read a tile or a narrower range",
                self.budget
            ));
            bytes = value.bytes();
        }
        if let Some(old) = self.entries.remove(&key) {
            self.held -= old.bytes;
        }
        self.make_room(bytes);
        self.tick += 1;
        self.held += bytes;
        self.entries.insert(
            key,
            Entry {
                value,
                bytes,
                used: self.tick,
            },
        );
        self.revision += 1;
    }

    /// Let go of reads until `bytes` more fit the budget: runs not on screen first, the least
    /// recently drawn first.
    fn make_room(&mut self, bytes: u64) {
        while self.held + bytes > self.budget {
            let Some(victim) = self
                .entries
                .iter()
                .min_by_key(|(key, entry)| (self.shown.contains(&key.run), entry.used))
                .map(|(key, _)| key.clone())
            else {
                return;
            };
            if let Some(entry) = self.entries.remove(&victim) {
                self.held -= entry.bytes;
            }
        }
    }
}

struct Shared {
    cache: Mutex<Cache>,
    work: Condvar,
    idle: Condvar,
    waker: Mutex<Option<Box<dyn Fn() + Send + Sync>>>,
}

impl Shared {
    fn lock(&self) -> MutexGuard<'_, Cache> {
        self.cache.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn wake(&self) {
        if let Some(waker) = self
            .waker
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .as_ref()
        {
            waker();
        }
    }
}

/// The reader service. Clones share one service.
#[derive(Clone)]
pub struct RunReader {
    shared: Arc<Shared>,
}

impl RunReader {
    /// A service that holds at most `budget` bytes of reads.
    pub fn new(budget: u64) -> Self {
        let shared = Arc::new(Shared {
            cache: Mutex::new(Cache {
                budget,
                ..Cache::default()
            }),
            work: Condvar::new(),
            idle: Condvar::new(),
            waker: Mutex::new(None),
        });
        #[cfg(not(target_arch = "wasm32"))]
        {
            let worker = Arc::downgrade(&shared);
            let spawned = std::thread::Builder::new()
                .name("results reader".to_owned())
                .spawn(move || serve(worker));
            if let Err(error) = spawned {
                log::warn!("the results reader could not start: {error}");
            }
        }
        Self { shared }
    }

    /// Ask for a frame when a read arrives.
    pub fn set_waker(&self, waker: impl Fn() + Send + Sync + 'static) {
        *self.shared.waker.lock().unwrap_or_else(|p| p.into_inner()) = Some(Box::new(waker));
    }

    /// The runs on screen now, whose reads go last when the budget needs room.
    pub fn show(&self, runs: impl IntoIterator<Item = u64>) {
        self.shared.lock().shown = runs.into_iter().collect();
    }

    /// Ask for `read` of `values`, a series of run `run` whose axis is `x`. What is held already,
    /// or what memory answers at once, comes back now; a stored read is queued, once, and comes
    /// back from [`Self::get`] when it arrives.
    pub fn request(
        &self,
        run: u64,
        values: &WaveformValues,
        x: &WaveformValues,
        read: Read,
    ) -> Option<ReadValue> {
        let Some(key) = ReadKey::of(run, values, read) else {
            return Some(perform(values, x, read));
        };
        if let Some(value) = self.get(&key) {
            return Some(value);
        }
        let mut cache = self.shared.lock();
        if cache.queued.insert(key.clone()) {
            cache.jobs.push_back(Job {
                key,
                values: values.clone(),
                x: x.clone(),
                read,
            });
            self.shared.work.notify_one();
        }
        drop(cache);
        #[cfg(target_arch = "wasm32")]
        {
            serve_queued(&self.shared);
            return self.get(&ReadKey::of(run, values, read)?);
        }
        #[cfg(not(target_arch = "wasm32"))]
        None
    }

    /// What the cache holds for `key`, counted as drawn now.
    pub fn get(&self, key: &ReadKey) -> Option<ReadValue> {
        let mut cache = self.shared.lock();
        cache.tick += 1;
        let tick = cache.tick;
        let entry = cache.entries.get_mut(key)?;
        entry.used = tick;
        Some(entry.value.clone())
    }

    /// Advances each time a read arrives.
    pub fn revision(&self) -> u64 {
        self.shared.lock().revision
    }

    /// The bytes of reads held now.
    pub fn loaded_bytes(&self) -> u64 {
        self.shared.lock().held
    }

    pub fn budget(&self) -> u64 {
        self.shared.lock().budget
    }

    /// Hold at most `budget` bytes from now on, letting go of reads at once to fit.
    pub fn set_budget(&self, budget: u64) {
        let mut cache = self.shared.lock();
        cache.budget = budget;
        cache.make_room(0);
    }

    /// Let go of every read of run `run`: the history no longer keeps it.
    pub fn release_run(&self, run: u64) {
        let mut cache = self.shared.lock();
        let held: u64 = cache
            .entries
            .iter()
            .filter(|(key, _)| key.run == run)
            .map(|(_, entry)| entry.bytes)
            .sum();
        cache.entries.retain(|key, _| key.run != run);
        cache.held -= held;
    }

    /// Wait until every queued read has arrived.
    pub fn settle(&self) {
        let mut cache = self.shared.lock();
        while !cache.jobs.is_empty() || cache.reading {
            cache = self
                .shared
                .idle
                .wait(cache)
                .unwrap_or_else(|p| p.into_inner());
        }
    }
}

/// What a history's results take, as the Save page states it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ResultsStorage {
    /// The bytes of the distinct results store segments its runs read.
    pub disk_bytes: u64,
    /// The bytes loaded in memory: samples its runs hold there, and the reads the reader
    /// service holds.
    pub loaded_bytes: u64,
    /// The reader service's budget.
    pub budget: u64,
}

/// What `runs` take on disk and in memory, `reader` holding their reads.
pub fn results_storage<'a>(
    runs: impl IntoIterator<Item = &'a super::SimulationRun>,
    reader: &RunReader,
) -> ResultsStorage {
    let mut segments: HashSet<[u8; 32]> = HashSet::new();
    let mut storage = ResultsStorage {
        loaded_bytes: reader.loaded_bytes(),
        budget: reader.budget(),
        ..ResultsStorage::default()
    };
    for run in runs {
        for analysis in &run.analyses {
            for waveform in &analysis.waveforms {
                let complex = waveform
                    .complex
                    .iter()
                    .flat_map(|complex| [&complex.real, &complex.imag]);
                for values in [&waveform.x, &waveform.y].into_iter().chain(complex) {
                    match values.stored_window() {
                        None => storage.loaded_bytes += values.len() as u64 * 8,
                        Some((source, _)) => {
                            if let Some((digest, _)) = source.identity()
                                && segments.insert(digest)
                            {
                                storage.disk_bytes +=
                                    std::fs::metadata(source.store()).map_or(0, |meta| meta.len());
                            }
                        }
                    }
                }
            }
        }
    }
    storage
}

/// The process's reader service, holding [`DEFAULT_BUDGET`] until the reader sets another.
pub fn run_reader() -> &'static RunReader {
    static READER: std::sync::OnceLock<RunReader> = std::sync::OnceLock::new();
    READER.get_or_init(|| RunReader::new(DEFAULT_BUDGET))
}

/// The worker: read each queued job and keep what it gave, until the service is gone.
#[cfg(not(target_arch = "wasm32"))]
fn serve(shared: std::sync::Weak<Shared>) {
    loop {
        let Some(strong) = shared.upgrade() else {
            return;
        };
        let job = {
            let mut cache = strong.lock();
            loop {
                if let Some(job) = cache.jobs.pop_front() {
                    cache.reading = true;
                    break Some(job);
                }
                strong.idle.notify_all();
                let (next, timeout) = strong
                    .work
                    .wait_timeout(cache, std::time::Duration::from_secs(1))
                    .unwrap_or_else(|p| p.into_inner());
                cache = next;
                if timeout.timed_out() && cache.jobs.is_empty() {
                    break None;
                }
            }
        };
        let Some(job) = job else {
            // Let go of the service between waits, so it can be dropped.
            drop(strong);
            continue;
        };
        let value = perform(&job.values, &job.x, job.read);
        {
            let mut cache = strong.lock();
            cache.queued.remove(&job.key);
            cache.keep(job.key, value);
            cache.reading = false;
            if cache.jobs.is_empty() {
                strong.idle.notify_all();
            }
        }
        strong.wake();
    }
}

/// The browser reads every queued job as it is asked.
#[cfg(target_arch = "wasm32")]
fn serve_queued(shared: &Shared) {
    loop {
        let Some(job) = shared.lock().jobs.pop_front() else {
            return;
        };
        let value = perform(&job.values, &job.x, job.read);
        let mut cache = shared.lock();
        cache.queued.remove(&job.key);
        cache.keep(job.key, value);
    }
}

#[cfg(test)]
mod tests;
