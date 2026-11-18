//! Checking referenced files against disk, and writing them on Save.
//!
//! Save writes each changed file of each editable referenced library first,
//! each atomically on its own, and the project file only once every one of
//! them is written: a project file never names a revision its folder does
//! not hold. A file that changed on disk since editing began is written
//! only once the user chose Keep mine; until then Save refuses, naming it.

use std::path::Path;

use rspice_design::library::folder::{FileRevision, LibraryFile, Missing};
use rspice_project::ProjectFile;

use super::{Model, OnDisk, ReferenceLoad};

/// One file Save writes or removes.
#[cfg(not(target_arch = "wasm32"))]
struct PlannedWrite {
    library: String,
    file: LibraryFile,
    /// The bytes and the revision they carry; none removes the file.
    write: Option<(rspice_design::library::folder::EncodedFile, u64)>,
}

impl Model {
    /// Check every referenced file by size and time, hashing only those that
    /// moved, and mark each that differs from what was loaded. Returns how
    /// many differ. Runs on open, on focus and on Save; never on a frame.
    pub fn check_library_files(&mut self) -> usize {
        #[cfg(not(target_arch = "wasm32"))]
        self.check_library_files_on_disk();
        self.refresh_library_edits();
        self.library_files
            .libraries()
            .flat_map(|(_, session)| session.files.values())
            .filter(|file| matches!(file.on_disk, OnDisk::Changed(_)))
            .count()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn check_library_files_on_disk(&mut self) {
        use rspice_project::library_folder::{library_file_stamp, read_library_file};
        let names = self
            .library_files
            .libraries()
            .filter(|(_, session)| matches!(session.load, ReferenceLoad::Loaded))
            .map(|(name, _)| name.to_owned())
            .collect::<Vec<_>>();
        for name in names {
            let Some(folder) = self.library_folder(&name).map(Path::to_path_buf) else {
                continue;
            };
            let Some(session) = self.library_files.library_mut(&name) else {
                continue;
            };
            for (file, state) in &mut session.files {
                if matches!(state.on_disk, OnDisk::TechnologyDiffers) || state.loaded.is_none() {
                    continue;
                }
                let stamp = library_file_stamp(&folder, file);
                if stamp.is_some() && stamp == state.stamp {
                    continue;
                }
                state.on_disk = match read_library_file(&folder, file) {
                    Err(rspice_project::library_folder::FileProblem::Missing) => {
                        OnDisk::Missing(Missing::NoFile(file.relative_path()))
                    }
                    Err(rspice_project::library_folder::FileProblem::Unreadable(reason)) => {
                        OnDisk::Unreadable(reason)
                    }
                    Ok((bytes, stamp)) => {
                        let digest = rspice_design::library::folder::digest_file_bytes(&bytes);
                        if state.loaded.is_some_and(|loaded| loaded.digest == digest) {
                            state.stamp = stamp;
                            OnDisk::AsLoaded
                        } else {
                            match file_revision(file, &bytes) {
                                Ok(revision) => OnDisk::Changed(FileRevision { revision, digest }),
                                Err(reason) => OnDisk::Unreadable(reason),
                            }
                        }
                    }
                };
            }
        }
    }

    /// Write every changed file of every editable referenced library that
    /// `file` holds, before the project file itself is written, and record
    /// in `file` and in the open project the revision each file now has.
    /// Refused, writing nothing, while a changed file cannot be written: a
    /// read-only library's, or one changed on disk since editing began with
    /// no Keep mine. A write that fails names its file, and the project file
    /// must then stay as it was.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn write_library_files(&mut self, file: &mut ProjectFile) -> Result<(), String> {
        // What another writer saved since the last check is found now, so
        // it is never written over unasked.
        self.check_library_files_on_disk();
        self.refresh_library_edits();
        let planned = self.plan_library_writes(file)?;
        let mut failures = Vec::new();
        for planned in planned {
            let Some(folder) = self.library_folder(&planned.library).map(Path::to_path_buf) else {
                continue;
            };
            let outcome = match &planned.write {
                Some((encoded, _)) => rspice_project::library_folder::write_library_file(
                    &folder,
                    &planned.file,
                    &encoded.bytes,
                ),
                None => rspice_project::library_folder::remove_library_file(&folder, &planned.file)
                    .map(|()| None),
            };
            match outcome {
                Ok(stamp) => self.record_library_write(&planned, stamp),
                Err(error) => {
                    failures.push(format!("{}/{} ({error})", planned.library, planned.file))
                }
            }
        }
        self.record_saved_library_revisions(file);
        if failures.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "The project file was left as it was, because these library files could not be written: {}.",
                failures.join("; ")
            ))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn plan_library_writes(&self, file: &ProjectFile) -> Result<Vec<PlannedWrite>, String> {
        let mut planned = Vec::new();
        let mut refused = Vec::new();
        for reference in file.libraries.references() {
            let Some(session) = self.library_files.library(&reference.name) else {
                continue;
            };
            if !matches!(session.load, ReferenceLoad::Loaded) {
                continue;
            }
            let Some(image) =
                file.workspace
                    .library_image(&file.libraries, &reference.name, reference.id)
            else {
                continue;
            };
            let mut files = image.files();
            files.extend(
                session
                    .files
                    .iter()
                    .filter(|(file, state)| {
                        state.loaded.is_some()
                            && matches!(state.on_disk, OnDisk::AsLoaded | OnDisk::Changed(_))
                            && image.revision(file).is_none()
                    })
                    .map(|(file, _)| file.clone()),
            );
            for library_file in files {
                let state = session.files.get(&library_file);
                if state.is_some_and(|state| matches!(state.on_disk, OnDisk::TechnologyDiffers)) {
                    continue;
                }
                let loaded = state.and_then(|state| state.loaded);
                let base = loaded.map_or(0, |loaded| loaded.revision);
                let encoded = image.encode(&library_file, base).and_then(Result::ok);
                let content = state.and_then(|state| state.content);
                if encoded.as_ref().map(|encoded| encoded.digest) == content {
                    continue;
                }
                let shown = format!("{}/{library_file}", reference.name);
                if !reference.mode.is_editable() {
                    refused.push(format!("{shown}: the library is read-only"));
                    continue;
                }
                let on_disk = state.and_then(|state| match state.on_disk {
                    OnDisk::Changed(on_disk) => Some(on_disk.revision),
                    _ => None,
                });
                if on_disk.is_some() && !state.is_some_and(|state| state.keep_mine) {
                    refused.push(format!(
                        "{shown}: {}; choose Keep mine or Take the file's",
                        rspice_design::library::folder::CHANGED_SINCE_EDITING
                    ));
                    continue;
                }
                let revision = base.max(on_disk.unwrap_or(0)) + 1;
                let write = match image.encode(&library_file, revision) {
                    Some(Ok(encoded)) => Some((encoded, revision)),
                    Some(Err(error)) => {
                        refused.push(format!("{shown}: {error}"));
                        continue;
                    }
                    None => None,
                };
                planned.push(PlannedWrite {
                    library: reference.name.clone(),
                    file: library_file,
                    write,
                });
            }
        }
        if refused.is_empty() {
            Ok(planned)
        } else {
            Err(format!("Nothing was saved. {}.", refused.join("; ")))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn record_library_write(
        &mut self,
        planned: &PlannedWrite,
        stamp: Option<rspice_project::library_folder::FileStamp>,
    ) {
        let folder = self.library_folder(&planned.library).map(Path::to_path_buf);
        let Some(session) = self.library_files.library_mut(&planned.library) else {
            return;
        };
        match &planned.write {
            Some((encoded, revision)) => {
                let state = session.files.entry(planned.file.clone()).or_default();
                state.loaded = Some(FileRevision {
                    revision: *revision,
                    digest: encoded.digest,
                });
                state.content = Some(encoded.digest);
                state.stamp = stamp;
                state.on_disk = OnDisk::AsLoaded;
                state.edited = false;
                state.keep_mine = false;
                if let (Some(lock), Some(folder)) = (state.lock.take(), folder) {
                    let _ = rspice_project::library_folder::release_library_lock(
                        &folder,
                        &planned.file,
                        &lock,
                    );
                }
            }
            None => {
                if let Some(state) = session.files.remove(&planned.file)
                    && let (Some(lock), Some(folder)) = (state.lock, folder)
                {
                    let _ = rspice_project::library_folder::release_library_lock(
                        &folder,
                        &planned.file,
                        &lock,
                    );
                }
            }
        }
    }

    /// Record in each reference, in `file` and in the open project alike, the
    /// revision of each file this session holds: what the project saves with.
    pub fn record_saved_library_revisions(&mut self, file: &mut ProjectFile) {
        let saved = self
            .library_files
            .libraries()
            .filter(|(_, session)| matches!(session.load, ReferenceLoad::Loaded))
            .map(|(name, session)| {
                let files = session
                    .files
                    .iter()
                    .filter_map(|(file, state)| Some((file.clone(), state.loaded?)))
                    .collect::<std::collections::BTreeMap<_, _>>();
                (name.to_owned(), files)
            })
            .collect::<Vec<_>>();
        for (name, files) in saved {
            for libraries in [&mut file.libraries, &mut self.library_manager] {
                if let Some(reference) = libraries.reference_mut(&name)
                    && reference.files != files
                {
                    reference.files = files.clone();
                }
            }
        }
    }

    /// Rewrite each relative library path, in `file` and in the open
    /// project, for a project file saved from the folder `from` into the
    /// folder `to`, so each still names the folder it named.
    pub fn rebase_library_references(
        &mut self,
        file: &mut ProjectFile,
        from: &Path,
        to: &Path,
    ) -> Result<(), String> {
        if from == to {
            return Ok(());
        }
        file.rebase_library_references(from, to)?;
        let rebased = file
            .libraries
            .references()
            .map(|reference| (reference.name.clone(), reference.path.clone()))
            .collect::<Vec<_>>();
        for (name, path) in rebased {
            if let Some(reference) = self.library_manager.reference_mut(&name)
                && reference.path != path
            {
                reference.path = path;
            }
        }
        Ok(())
    }
}

/// The revision a file's own record states.
#[cfg(not(target_arch = "wasm32"))]
fn file_revision(file: &LibraryFile, bytes: &[u8]) -> Result<u64, String> {
    use rspice_design::library::folder::{CellFile, CellViewFile, LibraryManifest};
    match file {
        LibraryFile::Manifest => LibraryManifest::decode(bytes).map(|record| record.revision),
        LibraryFile::Cell { .. } => CellFile::decode(bytes).map(|record| record.revision),
        LibraryFile::CellView { .. } => CellViewFile::decode(bytes).map(|record| record.revision),
    }
    .map_err(|error| error.to_string())
}
