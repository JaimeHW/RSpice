//! Unsaved edits to libraries kept in folders of their own, as a recovery
//! checkpoint holds them and as a restore gives them back.
//!
//! A checkpoint names each such library by path, like the project file, and
//! holds each file edited here and not yet saved, with the revision the edit
//! started from. A restore reads each folder as it is now and puts every
//! edit over it, marked edited, so only a Save writes it to the folder. A
//! file another writer saved since the edit began reads Changed on disk
//! since you started editing, as it would have in the session that crashed.

use std::collections::BTreeMap;

use rspice_design::library::folder::{LibraryFile, Missing};
use rspice_project::{ProjectFile, UnsavedLibraryFile};

use super::{LibraryFileSession, Model, OnDisk, ReferenceLoad};

impl Model {
    /// Every file of each library kept in a folder of its own that `file`
    /// holds otherwise than this session loaded it: what a Save would write.
    /// The edits held for libraries that could not be read come with them.
    pub fn unsaved_library_edits(&self, file: &ProjectFile) -> Vec<UnsavedLibraryFile> {
        let mut unsaved = Vec::new();
        for reference in file.libraries.references() {
            let Some(session) = self.library_files.library(&reference.name) else {
                continue;
            };
            if !matches!(
                session.load,
                ReferenceLoad::Loaded | ReferenceLoad::ReadForSession(_)
            ) {
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
                let content = state.and_then(|state| state.content);
                let base = loaded.map_or(0, |loaded| loaded.revision);
                let encoded = image.encode(&library_file, base).and_then(Result::ok);
                if encoded.map(|encoded| encoded.digest) == content {
                    continue;
                }
                unsaved.push(UnsavedLibraryFile {
                    library: reference.name.clone(),
                    edited: image.record(&library_file),
                    file: library_file,
                    loaded,
                    content,
                });
            }
        }
        unsaved.extend(self.library_files.pending().iter().cloned());
        unsaved
    }

    /// Make `file`, the open project read off the model, what a recovery
    /// checkpoint holds: each library kept in a folder of its own by its
    /// reference alone, and every edit to its files not yet saved there.
    pub fn hold_unsaved_library_files(&self, file: &mut ProjectFile) {
        if file.libraries.references().next().is_none() && self.library_files.pending().is_empty() {
            return;
        }
        let unsaved = self.unsaved_library_edits(file);
        *file = file.without_referenced_libraries();
        file.unsaved_library_files = unsaved;
    }

    /// Put `unsaved`, the edits a checkpoint or a project file held, into
    /// `file` over what `session` read of each folder, and mark each file
    /// edited: none is written to its folder until a Save. A file whose
    /// folder holds another revision than the edit started from reads
    /// Changed on disk since you started editing. The edits to a library
    /// that could not be read are held in `session` until one is.
    pub fn restore_unsaved_library_files(
        &self,
        file: &mut ProjectFile,
        session: &mut LibraryFileSession,
        unsaved: Vec<UnsavedLibraryFile>,
    ) {
        let mut by_library = BTreeMap::<String, Vec<UnsavedLibraryFile>>::new();
        for edit in unsaved {
            by_library
                .entry(edit.library.clone())
                .or_default()
                .push(edit);
        }
        for (library, edits) in by_library {
            let readable = session.library(&library).is_some_and(|read| {
                matches!(
                    read.load,
                    ReferenceLoad::Loaded | ReferenceLoad::ReadForSession(_)
                )
            });
            let read_only = file
                .libraries
                .reference(&library)
                .map(|reference| !reference.mode.is_editable());
            let put = match (readable, read_only) {
                (true, Some(read_only)) => file
                    .put_unsaved_library_files(&library, &edits, read_only)
                    .is_ok(),
                _ => false,
            };
            if !put {
                session.pending.extend(edits);
                continue;
            }
            let Some(read) = session.library_mut(&library) else {
                continue;
            };
            for edit in edits {
                let state = read.files.entry(edit.file.clone()).or_default();
                state.on_disk = on_disk_since(&edit.file, &state.on_disk, state.loaded, &edit);
                state.loaded = edit.loaded;
                state.content = edit.content;
                state.edited = true;
                state.keep_mine = false;
            }
        }
    }
}

/// How the folder stands against an edit that started from `edit.loaded`,
/// given what reading it found: `on_disk`, holding `found`.
fn on_disk_since(
    file: &LibraryFile,
    on_disk: &OnDisk,
    found: Option<rspice_design::library::folder::FileRevision>,
    edit: &UnsavedLibraryFile,
) -> OnDisk {
    match (on_disk, found, edit.loaded) {
        (OnDisk::AsLoaded, Some(found), Some(loaded)) if found.digest == loaded.digest => {
            OnDisk::AsLoaded
        }
        (OnDisk::AsLoaded, Some(found), _) => OnDisk::Changed(found),
        (OnDisk::AsLoaded, None, Some(_)) => OnDisk::Missing(Missing::NoFile(file.relative_path())),
        (on_disk, _, _) => on_disk.clone(),
    }
}
