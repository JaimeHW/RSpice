//! Where this process keeps the results stores its transients write, and the stored columns their
//! waveforms read.
//!
//! Each process keeps its segments in a folder of its own under the system's temporary folder,
//! beside a lock file it holds for as long as it runs. A segment lives as long as a waveform reads
//! it: the last column dropped deletes it. A process that ended without deleting its folder (a
//! crash, a kill) leaves its lock free, and [`sweep_abandoned_stores`] deletes the folder.
//! Liveness is the lock, never a process id, since process ids are reused.

use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime};

use rspice_results::store_reader::{SegmentBytes, StoreReader};
use rspice_results::waveform::{ColumnSource, ColumnTile, RowsAround};
use rspice_results_store::{FileIo, SegmentIo, SegmentReader, StoreError};

/// The folder under the system's temporary folder that holds every process's stores.
const STORES_FOLDER: &str = "rspice-results";

/// A folder or lock younger than this may belong to a process still creating it.
const CREATION_GRACE: Duration = Duration::from_secs(60);

/// This process's store folder and the lock that says it is alive.
struct ProcessStores {
    dir: PathBuf,
    _lock: File,
}

static PROCESS_STORES: OnceLock<Result<ProcessStores, String>> = OnceLock::new();
static NEXT_SEGMENT: AtomicU64 = AtomicU64::new(0);

fn stores_root() -> PathBuf {
    std::env::temp_dir().join(STORES_FOLDER)
}

/// The lock file beside a process's store folder.
fn lock_path(dir: &Path) -> PathBuf {
    let mut name = dir.as_os_str().to_owned();
    name.push(".lock");
    PathBuf::from(name)
}

fn process_stores() -> Result<&'static ProcessStores, String> {
    PROCESS_STORES
        .get_or_init(|| {
            let root = stores_root();
            let created = create_process_stores(&root)
                .map_err(|error| format!("{}: {error}", root.display()))?;
            sweep_in_background(root);
            Ok(created)
        })
        .as_ref()
        .map_err(Clone::clone)
}

/// Lock a new lock file, then make the folder it guards, so a sweeper that finds the folder finds
/// its lock held.
fn create_process_stores(root: &Path) -> std::io::Result<ProcessStores> {
    fs::create_dir_all(root)?;
    let stamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos());
    let mut attempt = 0u32;
    loop {
        let dir = root.join(format!("{}-{stamp:x}-{attempt}", std::process::id()));
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(lock_path(&dir))
        {
            Ok(lock) => {
                lock.try_lock().map_err(std::io::Error::other)?;
                fs::create_dir(&dir)?;
                return Ok(ProcessStores { dir, _lock: lock });
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => attempt += 1,
            Err(error) => return Err(error),
        }
    }
}

/// Delete, on a thread of its own, the store folders of processes that ended without deleting
/// them. Never blocks its caller; a folder it cannot delete stays for a later sweep.
pub fn sweep_abandoned_stores() {
    sweep_in_background(stores_root());
}

fn sweep_in_background(root: PathBuf) {
    let _ = std::thread::Builder::new()
        .name("rspice-store-sweep".to_owned())
        .spawn(move || sweep(&root, SystemTime::now()));
}

/// Delete each store folder under `root` whose lock no process holds, and each folder whose lock
/// file is gone, unless it is younger than [`CREATION_GRACE`] at `now`. Returns the folders
/// deleted.
fn sweep(root: &Path, now: SystemTime) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(root) else {
        return Vec::new();
    };
    let young = |path: &Path| {
        fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .map_or(true, |modified| {
                now.duration_since(modified)
                    .is_ok_and(|age| age < CREATION_GRACE)
                    || modified > now
            })
    };
    let mut deleted = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        let is_lock = path
            .extension()
            .is_some_and(|extension| extension == "lock");
        let dir = if is_lock {
            path.with_extension("")
        } else if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            path.clone()
        } else {
            continue;
        };
        let lock = lock_path(&dir);
        if !is_lock && lock.exists() {
            // Its lock file is visited on its own.
            continue;
        }
        if young(&path) {
            continue;
        }
        let held = if is_lock {
            match File::open(&lock) {
                Ok(file) => match file.try_lock() {
                    Ok(()) => {
                        // The owner is gone: delete what it guarded while holding its lock.
                        let removed = remove_dir(&dir);
                        drop(file);
                        removed && fs::remove_file(&lock).is_ok()
                    }
                    Err(_) => false,
                },
                Err(_) => false,
            }
        } else {
            remove_dir(&dir)
        };
        if held {
            deleted.push(dir);
        }
    }
    deleted
}

fn remove_dir(dir: &Path) -> bool {
    match fs::remove_dir_all(dir) {
        Ok(()) => true,
        Err(error) => error.kind() == std::io::ErrorKind::NotFound,
    }
}

/// The free space of the volume holding `segment`, which its run's disk watch reads.
pub(crate) fn free_space(segment: &Path) -> rspice_results_store::FreeSpace {
    #[cfg(test)]
    if let Some(free) =
        tests::FREE_SPACE.with(|free| free.borrow().as_ref().map(|free| free(segment)))
    {
        return free;
    }
    rspice_results_store::volume_free_space(segment.parent().unwrap_or(segment))
}

/// A new segment file in this process's store folder.
pub(crate) fn create_segment() -> Result<(PathBuf, TemporaryIo), String> {
    let stores = process_stores()?;
    let path = stores.dir.join(format!(
        "transient-{}.rsr",
        NEXT_SEGMENT.fetch_add(1, Ordering::Relaxed)
    ));
    FileIo::create(&path)
        .map(|io| (path.clone(), TemporaryIo(io)))
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// A segment of this process's store folder, as its run writes it.
///
/// Nothing reads the segment once this process ends: the last waveform that reads it deletes
/// it, and the sweep deletes it when the process dies first. So it is never made durable, where a
/// segment in general is synced when its run ends (on macOS a full flush of the drive's cache,
/// which waits for every other process's writes too). A run kept with a project is made durable
/// when the project is saved.
pub(crate) struct TemporaryIo(FileIo);

impl TemporaryIo {
    pub(crate) fn into_file(self) -> FileIo {
        self.0
    }
}

impl SegmentIo for TemporaryIo {
    fn read_at(&self, offset: u64, buf: &mut [u8]) -> std::io::Result<()> {
        self.0.read_at(offset, buf)
    }

    fn write_at(&mut self, offset: u64, data: &[u8]) -> std::io::Result<()> {
        self.0.write_at(offset, data)
    }

    fn truncate(&mut self, len: u64) -> std::io::Result<()> {
        self.0.truncate(len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<u64> {
        self.0.size()
    }
}

/// A finished segment its waveforms read from, deleted with the last of them.
pub(crate) struct StoredRun {
    reader: StoreReader,
    path: PathBuf,
}

impl StoredRun {
    pub(crate) fn new(reader: StoreReader, path: PathBuf) -> Self {
        Self { reader, path }
    }

    /// The segment file.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn rows(&self) -> u64 {
        self.reader.rows()
    }

    /// The reader every column of the run shares.
    pub(crate) fn reader(&self) -> &StoreReader {
        &self.reader
    }

    /// Run `read` with the segment's reader.
    pub(crate) fn with_reader<T>(
        &self,
        read: impl FnOnce(&mut SegmentReader<SegmentBytes>) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        self.reader.with_reader(read)
    }

    /// Rows `first .. first + out.len()` of the x axis (`slot` `None`) or a value slot.
    pub(crate) fn read(
        &self,
        slot: Option<usize>,
        first: u64,
        out: &mut [f64],
    ) -> Result<(), StoreError> {
        self.reader.read(slot, first, out)
    }
}

impl Drop for StoredRun {
    fn drop(&mut self) {
        // Close the segment before deleting it: a host may refuse to delete an open file.
        self.reader.close();
        let _ = fs::remove_file(&self.path);
    }
}

/// One digital or real event signal of a stored run, as its history reads it.
pub(crate) struct StoredEventSignal {
    run: Arc<StoredRun>,
    signal: usize,
    name: String,
}

impl StoredEventSignal {
    pub(crate) fn new(run: Arc<StoredRun>, signal: usize, name: String) -> Arc<Self> {
        Arc::new(Self { run, signal, name })
    }
}

impl rspice_results::events::points::EventHistorySource for StoredEventSignal {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.run.path().display().to_string()
    }

    fn count(&self) -> Result<u64, String> {
        self.run
            .reader()
            .event_count(self.signal)
            .map_err(|error| error.to_string())
    }

    fn before(&self, t: f64) -> Result<u64, String> {
        self.run
            .reader()
            .events_before(self.signal, t)
            .map_err(|error| error.to_string())
    }

    fn span(&self, t0: f64, t1: f64) -> Result<rspice_results_store::EventSpan, String> {
        self.run
            .reader()
            .events_around(self.signal, t0, t1)
            .map_err(|error| error.to_string())
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        Some((
            self.run.reader().digest()?.0,
            u64::try_from(self.signal).ok()?,
        ))
    }
}

/// One column of a stored run, as a waveform reads it.
pub(crate) struct StoredColumn {
    run: Arc<StoredRun>,
    /// The value slot, or `None` for the x axis.
    slot: Option<usize>,
    name: String,
}

impl StoredColumn {
    pub(crate) fn new(run: Arc<StoredRun>, slot: Option<usize>, name: String) -> Arc<Self> {
        Arc::new(Self { run, slot, name })
    }
}

impl ColumnSource for StoredColumn {
    fn name(&self) -> &str {
        &self.name
    }

    fn store(&self) -> String {
        self.run.path().display().to_string()
    }

    fn rows(&self) -> u64 {
        self.run.rows()
    }

    fn read(&self, first: u64, out: &mut [f64]) -> Result<(), String> {
        self.run
            .read(self.slot, first, out)
            .map_err(|error| error.to_string())
    }

    /// The time axis: the run checked it ascends strictly before it made its waveforms.
    fn ascending(&self) -> bool {
        self.slot.is_none()
    }

    fn identity(&self) -> Option<([u8; 32], u64)> {
        let column = self
            .slot
            .map_or(Some(0), |slot| u64::try_from(slot).ok()?.checked_add(1))?;
        Some((self.run.reader().digest()?.0, column))
    }

    fn tile(
        &self,
        rows: std::ops::Range<u64>,
        x0: f64,
        x1: f64,
        pixels: usize,
    ) -> Option<Result<ColumnTile, String>> {
        let slot = self.slot?;
        Some(
            self.run
                .reader()
                .tile(slot, rows, x0, x1, pixels)
                .map(ColumnTile::from)
                .map_err(|error| error.to_string()),
        )
    }

    fn rows_around(
        &self,
        rows: std::ops::Range<u64>,
        x: f64,
    ) -> Option<Result<RowsAround, String>> {
        let slot = self.slot?;
        Some(
            self.run
                .reader()
                .rows_around(slot, rows, x)
                .map_err(|error| error.to_string()),
        )
    }
}

#[cfg(test)]
pub(crate) mod tests;
