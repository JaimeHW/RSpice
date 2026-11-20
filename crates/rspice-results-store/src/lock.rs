//! One writer per store: a lock a process holds while it writes a store or publishes over one.
//!
//! The lock is the file `<store>.lock` beside the store, locked through the operating system, so
//! a process that dies lets go of it and the next one to ask gets it. The file is removed when
//! its last holder lets go: on unix while it is still locked (a process that then opens the path
//! makes a new file, and one already waiting on the removed file sees it is no longer the one at
//! the path and asks again), on Windows only once no other process has it open, which the file's
//! sharing (no delete while open) enforces. A process that holds a store's lock shares it when it
//! asks again with [`StoreLock::acquire`], so a run holding it can publish under it.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, Weak};

/// The lock on one results store, held while this value (or a clone) lives.
#[derive(Clone)]
pub struct StoreLock {
    _held: Arc<Held>,
}

struct Held {
    file: Option<File>,
    path: PathBuf,
}

/// Every store lock this process holds, by lock file.
static HELD: Mutex<BTreeMap<PathBuf, Weak<Held>>> = Mutex::new(BTreeMap::new());

fn holders() -> MutexGuard<'static, BTreeMap<PathBuf, Weak<Held>>> {
    HELD.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl StoreLock {
    /// The lock file of the store at `store`: `<store>.lock`, beside it.
    pub fn file_for(store: &Path) -> PathBuf {
        let mut name = store.as_os_str().to_owned();
        name.push(".lock");
        PathBuf::from(name)
    }

    /// The lock on `store`, waiting while another process holds it, and shared with this
    /// process's own holder when it has one.
    pub fn acquire(store: &Path) -> io::Result<Self> {
        Self::take(store, true)?.ok_or_else(|| io::Error::other("a store lock was not taken"))
    }

    /// The lock on `store`, or `None` while anyone else holds it: another process, or another
    /// holder in this one.
    pub fn try_acquire(store: &Path) -> io::Result<Option<Self>> {
        Self::take(store, false)
    }

    /// This process's lock on `store`, shared, when it holds one.
    pub fn held(store: &Path) -> Option<Self> {
        let own = holders()
            .get(&Self::file_for(store))
            .and_then(Weak::upgrade)?;
        Some(Self { _held: own })
    }

    fn take(store: &Path, wait: bool) -> io::Result<Option<Self>> {
        let path = Self::file_for(store);
        if let Some(own) = holders().get(&path).and_then(Weak::upgrade) {
            return Ok(wait.then_some(Self { _held: own }));
        }
        loop {
            let file = open(&path)?;
            if wait {
                file.lock()?;
            } else {
                match file.try_lock() {
                    Ok(()) => {}
                    Err(std::fs::TryLockError::WouldBlock) => return Ok(None),
                    Err(std::fs::TryLockError::Error(error)) => return Err(error),
                }
            }
            // A holder that let go removed the file it locked; a lock on that file locks nothing.
            if !still_at(&file, &path)? {
                continue;
            }
            let own = Arc::new(Held {
                file: Some(file),
                path: path.clone(),
            });
            let mut held = holders();
            held.retain(|_, holder| holder.strong_count() > 0);
            held.insert(path, Arc::downgrade(&own));
            return Ok(Some(Self { _held: own }));
        }
    }
}

impl Drop for Held {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            let _ = std::fs::remove_file(&self.path);
            self.file.take();
        }
        #[cfg(windows)]
        {
            self.file.take();
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

/// Whether `file` is still the file at `path`. On Windows a file nobody may delete while it is
/// open always is.
fn still_at(file: &File, path: &Path) -> io::Result<bool> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let locked = file.metadata()?;
        Ok(std::fs::metadata(path)
            .is_ok_and(|now| now.dev() == locked.dev() && now.ino() == locked.ino()))
    }
    #[cfg(not(unix))]
    {
        let _ = (file, path);
        Ok(true)
    }
}

pub(crate) fn open(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.read(true).write(true).create(true).truncate(false);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        // Read and write sharing, never delete: the file stays while anyone has it open.
        const FILE_SHARE_READ: u32 = 1;
        const FILE_SHARE_WRITE: u32 = 2;
        options.share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
    }
    options.open(path)
}
