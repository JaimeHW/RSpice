//! A library folder on this computer: reading it, writing its files one at
//! a time and atomically, and the lock beside each file.
//!
//! The reader takes the folder as it finds it: every cell folder with its
//! `cell.json` and every view folder with its `<type>.json`. A file that
//! cannot be read is named with its reason; it never costs the rest.

use std::collections::BTreeMap;
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use rspice_design::library::folder::{
    CELL_FILE, CellFile, CellViewFile, FileRevision, LIBRARY_MANIFEST_FILE, LOCK_SUFFIX,
    LibraryFile, LibraryFileError, LibraryFileLock, LibraryManifest, digest_file_bytes,
};

use crate::workspace::LibraryImage;

/// A file's size and modification time: a check hashes a file only when
/// these moved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStamp {
    pub len: u64,
    pub modified_unix_ns: Option<u128>,
}

/// One file as read: its record, its revision and digest, and its stamp.
#[derive(Debug, Clone)]
pub struct ReadFile<T> {
    pub record: T,
    pub revision: FileRevision,
    pub stamp: Option<FileStamp>,
}

/// Why one file of a readable folder could not be taken.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileProblem {
    Missing,
    Unreadable(String),
}

/// What reading one file gave: its revision and stamp, or why not.
pub type FileOutcome = Result<(FileRevision, Option<FileStamp>), FileProblem>;

/// Why a folder could not be taken as a library at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FolderProblem {
    NoFolder,
    Unreadable(String),
}

/// Everything read from one library folder.
#[derive(Debug, Clone)]
pub struct FolderRead {
    pub manifest: ReadFile<LibraryManifest>,
    pub cells: BTreeMap<String, Result<ReadFile<CellFile>, FileProblem>>,
    pub cellviews: BTreeMap<LibraryFile, Result<ReadFile<CellViewFile>, FileProblem>>,
}

impl FolderRead {
    /// What could be read, as the folder's image. A cell whose own file is
    /// missing still holds the views found under it.
    pub fn image(&self) -> LibraryImage {
        let mut cells = BTreeMap::new();
        for (name, read) in &self.cells {
            if let Ok(read) = read {
                cells.insert(name.clone(), read.record.clone());
            }
        }
        let cellviews = self
            .cellviews
            .iter()
            .filter_map(|(file, read)| Some((file.clone(), read.as_ref().ok()?.record.clone())))
            .collect();
        LibraryImage {
            manifest: self.manifest.record.clone(),
            cells,
            cellviews,
        }
    }

    /// Every file found, each with what reading it gave.
    pub fn files(&self) -> Vec<(LibraryFile, FileOutcome)> {
        let mut files = vec![(
            LibraryFile::Manifest,
            Ok((self.manifest.revision, self.manifest.stamp)),
        )];
        files.extend(self.cells.iter().map(|(cell, read)| {
            (
                LibraryFile::cell(cell.as_str()),
                read.as_ref()
                    .map(|read| (read.revision, read.stamp))
                    .map_err(Clone::clone),
            )
        }));
        files.extend(self.cellviews.iter().map(|(file, read)| {
            (
                file.clone(),
                read.as_ref()
                    .map(|read| (read.revision, read.stamp))
                    .map_err(Clone::clone),
            )
        }));
        files
    }
}

/// Read every file of the library folder at `folder`.
pub fn read_library_folder(folder: &Path) -> Result<FolderRead, FolderProblem> {
    if !folder.is_dir() {
        return Err(FolderProblem::NoFolder);
    }
    let mut files = Vec::new();
    let manifest = match std::fs::read(folder.join(LIBRARY_MANIFEST_FILE)) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(FolderProblem::Unreadable(format!(
                "The folder holds no {LIBRARY_MANIFEST_FILE}, so it is not a library"
            )));
        }
        Err(error) => return Err(FolderProblem::Unreadable(error.to_string())),
    };
    files.push((LIBRARY_MANIFEST_FILE.to_owned(), manifest));
    let cell_folders =
        sorted_entries(folder).map_err(|error| FolderProblem::Unreadable(error.to_string()))?;
    for (cell, cell_path) in cell_folders.into_iter().filter(|(_, path)| path.is_dir()) {
        if let Ok(bytes) = std::fs::read(cell_path.join(CELL_FILE)) {
            files.push((format!("{cell}/{CELL_FILE}"), bytes));
        }
        let Ok(views) = sorted_entries(&cell_path) else {
            continue;
        };
        for (view, view_path) in views.into_iter().filter(|(_, path)| path.is_dir()) {
            let Ok(entries) = sorted_entries(&view_path) else {
                continue;
            };
            for (name, path) in entries {
                if name.ends_with(".json") && path.is_file() {
                    let bytes = std::fs::read(&path).unwrap_or_default();
                    files.push((format!("{cell}/{view}/{name}"), bytes));
                }
            }
        }
    }
    let stamps = |relative: &str| file_stamp_at(&folder.join(relative));
    read_library_files_with(files, &stamps)
}

/// Read a library from its files' bytes, each by its path inside the
/// folder: a browser's folder upload, or a folder read file by file.
pub fn read_library_files(files: Vec<(String, Vec<u8>)>) -> Result<FolderRead, FolderProblem> {
    read_library_files_with(files, &|_| None)
}

fn read_library_files_with(
    files: Vec<(String, Vec<u8>)>,
    stamps: &dyn Fn(&str) -> Option<FileStamp>,
) -> Result<FolderRead, FolderProblem> {
    let mut manifest = None;
    let mut cells = BTreeMap::new();
    let mut cellviews = BTreeMap::new();
    for (relative, bytes) in files {
        let relative = relative.replace('\\', "/");
        let relative = relative.trim_start_matches('/');
        if relative.ends_with(LOCK_SUFFIX) {
            continue;
        }
        let Ok(file) = relative.parse::<LibraryFile>() else {
            continue;
        };
        let revision = |record_revision: u64| FileRevision {
            revision: record_revision,
            digest: digest_file_bytes(&bytes),
        };
        let stamp = stamps(relative);
        match &file {
            LibraryFile::Manifest => {
                let record = LibraryManifest::decode(&bytes)
                    .map_err(|error| FolderProblem::Unreadable(error.to_string()))?;
                manifest = Some(ReadFile {
                    revision: revision(record.revision),
                    record,
                    stamp,
                });
            }
            LibraryFile::Cell { cell } => {
                cells.insert(
                    cell.clone(),
                    read_record(&bytes, CellFile::decode).map(|record| ReadFile {
                        revision: revision(record.revision),
                        record,
                        stamp,
                    }),
                );
            }
            LibraryFile::CellView { cell, .. } => {
                cells
                    .entry(cell.clone())
                    .or_insert(Err(FileProblem::Missing));
                let read = read_record(&bytes, CellViewFile::decode).and_then(|record| {
                    if record.view.name != file.view_name().unwrap_or_default()
                        || Some(&record.view.view_type)
                            != match &file {
                                LibraryFile::CellView { view_type, .. } => Some(view_type),
                                _ => None,
                            }
                    {
                        return Err(FileProblem::Unreadable(
                            "The file holds another view than its place names".to_owned(),
                        ));
                    }
                    Ok(ReadFile {
                        revision: revision(record.revision),
                        record,
                        stamp,
                    })
                });
                cellviews.insert(file, read);
            }
        }
    }
    let manifest = manifest.ok_or_else(|| {
        FolderProblem::Unreadable(format!(
            "The folder holds no {LIBRARY_MANIFEST_FILE}, so it is not a library"
        ))
    })?;
    Ok(FolderRead {
        manifest,
        cells,
        cellviews,
    })
}

fn read_record<T>(
    bytes: &[u8],
    decode: fn(&[u8]) -> Result<T, LibraryFileError>,
) -> Result<T, FileProblem> {
    if bytes.is_empty() {
        return Err(FileProblem::Unreadable("The file is empty".to_owned()));
    }
    decode(bytes).map_err(|error| FileProblem::Unreadable(error.to_string()))
}

/// The bytes of one file of the folder, with its stamp.
pub fn read_library_file(
    folder: &Path,
    file: &LibraryFile,
) -> Result<(Vec<u8>, Option<FileStamp>), FileProblem> {
    let path = folder.join(file.relative_path());
    match std::fs::read(&path) {
        Ok(bytes) => Ok((bytes, file_stamp_at(&path))),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Err(FileProblem::Missing),
        Err(error) => Err(FileProblem::Unreadable(error.to_string())),
    }
}

/// One file's size and modification time, when it is there.
pub fn library_file_stamp(folder: &Path, file: &LibraryFile) -> Option<FileStamp> {
    file_stamp_at(&folder.join(file.relative_path()))
}

fn file_stamp_at(path: &Path) -> Option<FileStamp> {
    let metadata = std::fs::metadata(path).ok()?;
    Some(FileStamp {
        len: metadata.len(),
        modified_unix_ns: metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|elapsed| elapsed.as_nanos()),
    })
}

/// Write one file of the folder atomically: the bytes go to a file beside
/// it, which then takes its place, so a reader sees the old file or the new
/// one and never part of either.
pub fn write_library_file(
    folder: &Path,
    file: &LibraryFile,
    bytes: &[u8],
) -> io::Result<Option<FileStamp>> {
    let path = folder.join(file.relative_path());
    let parent = path.parent().unwrap_or(folder);
    std::fs::create_dir_all(parent)?;
    let staging = parent.join(format!(".{}.{}.tmp", file.file_name(), std::process::id()));
    let written = (|| {
        let mut staged = std::fs::File::create(&staging)?;
        staged.write_all(bytes)?;
        staged.sync_all()?;
        drop(staged);
        std::fs::rename(&staging, &path)
    })();
    if written.is_err() {
        let _ = std::fs::remove_file(&staging);
    }
    written?;
    Ok(file_stamp_at(&path))
}

/// Remove one cellview's file, and its view folder once empty, for a view
/// deleted from an editable library.
pub fn remove_library_file(folder: &Path, file: &LibraryFile) -> io::Result<()> {
    let path = folder.join(file.relative_path());
    match std::fs::remove_file(&path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    if let Some(parent) = path.parent()
        && parent != folder
    {
        let _ = std::fs::remove_dir(parent);
        if let Some(cell) = parent.parent()
            && cell != folder
        {
            let _ = std::fs::remove_dir(cell);
        }
    }
    Ok(())
}

/// What taking a lock came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LockOutcome {
    /// No lock was there; this one is.
    Taken,
    /// This project in this process already holds it.
    AlreadyHeld,
    /// A lock left by a process that is gone from this computer was
    /// removed, and this one taken.
    TakenOver(LibraryFileLock),
    /// Someone else holds it.
    HeldBy(LibraryFileLock),
}

/// Take the lock on one file for `lock`'s holder: written only when no lock
/// is there. A lock left on this computer by a process that is provably
/// gone is taken over; any other is reported with its holder.
pub fn take_library_lock(
    folder: &Path,
    file: &LibraryFile,
    lock: &LibraryFileLock,
    process_liveness: &dyn Fn(u32) -> Option<bool>,
) -> io::Result<LockOutcome> {
    let path = folder.join(file.lock_path());
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let bytes = lock
        .encode()
        .map_err(|error| io::Error::other(error.to_string()))?
        .bytes;
    let create = |bytes: &[u8]| -> io::Result<bool> {
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut created) => {
                created.write_all(bytes)?;
                created.sync_all()?;
                Ok(true)
            }
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => Ok(false),
            Err(error) => Err(error),
        }
    };
    if create(&bytes)? {
        return Ok(LockOutcome::Taken);
    }
    let Some(held) = read_library_lock(folder, file) else {
        // A lock that cannot be read is still someone's: it is broken only
        // on purpose.
        return Ok(LockOutcome::HeldBy(LibraryFileLock {
            user: "someone".to_owned(),
            computer: "another computer".to_owned(),
            process: 0,
            project: String::new(),
            since_unix_ms: 0,
        }));
    };
    if held.is_on(&lock.computer) && held.process == lock.process && held.project == lock.project {
        return Ok(LockOutcome::AlreadyHeld);
    }
    if held.is_on(&lock.computer)
        && held.process != std::process::id()
        && process_liveness(held.process) == Some(false)
    {
        std::fs::remove_file(&path)?;
        if create(&bytes)? {
            return Ok(LockOutcome::TakenOver(held));
        }
        return Ok(read_library_lock(folder, file).map_or(LockOutcome::Taken, LockOutcome::HeldBy));
    }
    Ok(LockOutcome::HeldBy(held))
}

/// The lock on one file, when there is one to read.
pub fn read_library_lock(folder: &Path, file: &LibraryFile) -> Option<LibraryFileLock> {
    let bytes = std::fs::read(folder.join(file.lock_path())).ok()?;
    LibraryFileLock::decode(&bytes).ok()
}

/// Release `lock`: the lock file goes only while it is still this holder's.
pub fn release_library_lock(
    folder: &Path,
    file: &LibraryFile,
    lock: &LibraryFileLock,
) -> io::Result<()> {
    if read_library_lock(folder, file).as_ref() == Some(lock) {
        match std::fs::remove_file(folder.join(file.lock_path())) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

/// Remove whatever lock is on one file: "Break lock…". Returns the holder
/// it removed.
pub fn break_library_lock(
    folder: &Path,
    file: &LibraryFile,
) -> io::Result<Option<LibraryFileLock>> {
    let held = read_library_lock(folder, file);
    match std::fs::remove_file(folder.join(file.lock_path())) {
        Ok(()) => Ok(held),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn sorted_entries(folder: &Path) -> io::Result<Vec<(String, PathBuf)>> {
    let mut entries = std::fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            (!name.starts_with('.')).then(|| (name, entry.path()))
        })
        .collect::<Vec<_>>();
    entries.sort();
    Ok(entries)
}

#[cfg(test)]
mod tests;
