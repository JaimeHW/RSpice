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

use rspice_results_store::{FileIo, SegmentIo};

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

pub use rspice_results_store::StorageBudget;

thread_local! {
    /// The storage budget of the run solving on this thread, which every segment it writes is
    /// counted against. The bridge solves a task on the thread the runner started it on.
    static STORAGE_BUDGET: std::cell::RefCell<Option<Arc<StorageBudget>>> =
        const { std::cell::RefCell::new(None) };
}

/// Count every segment the run on this thread writes against `budget` until the guard drops.
#[must_use]
pub(crate) fn install_storage_budget(budget: Arc<StorageBudget>) -> StorageBudgetGuard {
    let previous = STORAGE_BUDGET.with(|slot| slot.replace(Some(budget)));
    StorageBudgetGuard { previous }
}

/// What removes a run's storage budget from its thread, on every exit path.
pub(crate) struct StorageBudgetGuard {
    previous: Option<Arc<StorageBudget>>,
}

impl Drop for StorageBudgetGuard {
    fn drop(&mut self) {
        let previous = self.previous.take();
        STORAGE_BUDGET.with(|slot| {
            slot.replace(previous);
        });
    }
}

/// The storage budget of the run on this thread, when its plan set one.
pub(crate) fn storage_budget() -> Option<Arc<StorageBudget>> {
    STORAGE_BUDGET.with(|slot| slot.borrow().clone())
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

/// A finished segment its waveforms read from, deleted with the last of them
/// ([`StoredRun::temporary`]), and its columns and event signals.
pub(crate) use rspice_results::store_reader::{
    Segment as StoredRun, SegmentColumn as StoredColumn, SegmentEvents as StoredEventSignal,
};

#[cfg(test)]
pub(crate) mod tests;
