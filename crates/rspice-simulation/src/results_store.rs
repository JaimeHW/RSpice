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
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

use rspice_results::waveform::ColumnSource;
use rspice_results_store::{FileIo, SegmentReader, StoreError};

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
pub(crate) fn create_segment() -> Result<(PathBuf, FileIo), String> {
    let stores = process_stores()?;
    let path = stores.dir.join(format!(
        "transient-{}.rsr",
        NEXT_SEGMENT.fetch_add(1, Ordering::Relaxed)
    ));
    FileIo::create(&path)
        .map(|io| (path.clone(), io))
        .map_err(|error| format!("{}: {error}", path.display()))
}

/// A finished segment its waveforms read from, deleted with the last of them.
pub(crate) struct StoredRun {
    reader: Mutex<Option<SegmentReader<FileIo>>>,
    /// Each chunk's first row and row count, in row order.
    spans: Vec<(u64, usize)>,
    rows: u64,
    path: PathBuf,
}

impl StoredRun {
    pub(crate) fn new(reader: SegmentReader<FileIo>, path: PathBuf) -> Self {
        Self {
            spans: reader.chunk_spans().collect(),
            rows: reader.rows(),
            reader: Mutex::new(Some(reader)),
            path,
        }
    }

    /// The segment file.
    pub(crate) fn path(&self) -> &Path {
        &self.path
    }

    pub(crate) fn rows(&self) -> u64 {
        self.rows
    }

    /// Run `read` with the segment's reader.
    pub(crate) fn with_reader<T>(
        &self,
        read: impl FnOnce(&mut SegmentReader<FileIo>) -> Result<T, StoreError>,
    ) -> Result<T, StoreError> {
        let mut reader = self.reader.lock().unwrap_or_else(|p| p.into_inner());
        let reader = reader.as_mut().ok_or_else(|| StoreError::Io {
            kind: std::io::ErrorKind::Other,
            message: "the results segment was closed".to_owned(),
        })?;
        read(reader)
    }

    /// Rows `first .. first + out.len()` of the x axis (`slot` `None`) or a value slot, decoded
    /// chunk by chunk into `out`.
    pub(crate) fn read(
        &self,
        slot: Option<usize>,
        first: u64,
        out: &mut [f64],
    ) -> Result<(), StoreError> {
        let end = first + out.len() as u64;
        if end > self.rows {
            return Err(StoreError::OutOfRange(format!(
                "rows {first} .. {end} of a segment with {} rows",
                self.rows
            )));
        }
        self.with_reader(|reader| {
            let mut chunk = self
                .spans
                .partition_point(|&(start, rows)| start + rows as u64 <= first);
            let mut buffer = Vec::new();
            let mut row = first;
            while row < end {
                let (start, rows) = self.spans[chunk];
                match slot {
                    None => reader.read_chunk_x(chunk, &mut buffer)?,
                    Some(slot) => reader.read_chunk_slot(chunk, slot, &mut buffer)?,
                }
                let from = (row - start) as usize;
                let to = rows.min((end - start) as usize);
                let at = (row - first) as usize;
                out[at..at + to - from].copy_from_slice(&buffer[from..to]);
                row = start + to as u64;
                chunk += 1;
            }
            Ok(())
        })
    }
}

impl Drop for StoredRun {
    fn drop(&mut self) {
        // Close the segment before deleting it: a host may refuse to delete an open file.
        drop(
            self.reader
                .get_mut()
                .unwrap_or_else(|p| p.into_inner())
                .take(),
        );
        let _ = fs::remove_file(&self.path);
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
}

#[cfg(test)]
pub(crate) mod tests;
