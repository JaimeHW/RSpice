//! Libraries read from folders of their own.
//!
//! The project names each such library by a [`LibraryReference`]; its cells,
//! once read, sit in the catalog beside the embedded libraries', and their
//! drawings, layouts and sources in the project's stores, so every page and
//! editor works on them as on any other. What this session read from each
//! file of the folder, what it found there since, and which locks it holds
//! are kept here, at runtime only: opening a project reads the folders again.
//!
//! One status function, [`Model::reference_status`], says where a library
//! stands, and [`Model::library_file_status`] where one of its files does,
//! for the rail, the table, the inspector and the Overview alike. Nothing
//! here reads the disk on a frame: a check runs on open, when the window
//! regains focus and on Save.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub use rspice_design::library::folder::{
    CellViewStatus, FileRevision, LibraryFile, LibraryFileLock, LibraryMode, LibraryPath,
    LibraryPathChoice, Missing, ReferenceStatus, StatusMark,
};
pub use rspice_project::LibraryReference;
pub use rspice_project::library_folder::FileStamp;
use rspice_vocabulary::product::ContentDigest;

mod commands;
mod folders;
mod saving;

#[cfg(test)]
mod tests;

use super::Model;

/// What this session knows of every library kept in a folder of its own.
#[derive(Debug, Clone, Default)]
pub struct LibraryFileSession {
    libraries: BTreeMap<String, ReferenceSession>,
    stamp: u64,
}

impl LibraryFileSession {
    pub fn library(&self, name: &str) -> Option<&ReferenceSession> {
        self.libraries.get(name)
    }

    /// Changes whenever anything here does, for caches over the statuses.
    pub const fn stamp(&self) -> u64 {
        self.stamp
    }

    pub(crate) fn library_mut(&mut self, name: &str) -> Option<&mut ReferenceSession> {
        self.stamp = rspice_vocabulary::stamped::issue();
        self.libraries.get_mut(name)
    }

    pub(crate) fn insert(&mut self, name: impl Into<String>, session: ReferenceSession) {
        self.stamp = rspice_vocabulary::stamped::issue();
        self.libraries.insert(name.into(), session);
    }

    pub(crate) fn remove(&mut self, name: &str) -> Option<ReferenceSession> {
        self.stamp = rspice_vocabulary::stamped::issue();
        self.libraries.remove(name)
    }

    pub fn libraries(&self) -> impl Iterator<Item = (&str, &ReferenceSession)> {
        self.libraries
            .iter()
            .map(|(name, session)| (name.as_str(), session))
    }
}

/// What this session read from one library's folder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSession {
    /// The folder the reference resolves to here, or why it names none.
    pub folder: Result<PathBuf, Missing>,
    pub load: ReferenceLoad,
    /// The technology pins, library's then project's, when the library's
    /// layouts were drawn for another and so were not loaded.
    pub technology_differs: Option<(String, String)>,
    pub files: BTreeMap<LibraryFile, FileSession>,
}

/// How reading the folder went, as a whole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceLoad {
    Loaded,
    /// A browser read a picked folder for this session only, read-only.
    ReadForSession(String),
    /// A browser cannot read files by path.
    NotLoaded,
    NotFound(Missing),
    Unreadable(String),
    /// The folder's manifest names this other library.
    DifferentLibrary(String),
}

/// What this session read from one file and found there since.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FileSession {
    /// The revision and digest this session loaded; none when it could not,
    /// or for a file this session made that no save has written yet.
    pub loaded: Option<FileRevision>,
    /// The digest of what this session loaded as this build writes it: the
    /// file has edits here exactly when the project's parts encode to other
    /// bytes. None for a file not loaded.
    pub content: Option<ContentDigest>,
    /// The file's size and modification time when last read or checked: a
    /// check hashes the file only when these moved.
    pub stamp: Option<FileStamp>,
    pub on_disk: OnDisk,
    /// Whether the file has edits here not yet saved.
    pub edited: bool,
    /// The lock this session holds on the file, while it holds one.
    pub lock: Option<LibraryFileLock>,
    /// The user chose "Keep my version" for a file that changed on disk
    /// since they started editing: the next save writes over it.
    pub keep_mine: bool,
}

/// What the last read or check found at a file's place.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum OnDisk {
    /// As this session loaded it.
    #[default]
    AsLoaded,
    /// Another writer saved it since: its revision and digest now.
    Changed(FileRevision),
    Missing(Missing),
    Unreadable(String),
    /// A layout drawn for another technology pin.
    TechnologyDiffers,
}

/// One row of the Refresh list: a file that changed on disk since this
/// session loaded it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshRow {
    pub library: String,
    pub file: LibraryFile,
    pub loaded: Option<u64>,
    pub on_disk: Option<u64>,
    /// It has edits here too: it is never overwritten, and reads "Changed
    /// on disk since you started editing" until the user chooses.
    pub edited: bool,
}

/// What Relink did, or what it needs to be asked first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelinkOutcome {
    Relinked,
    /// The picked folder holds the same library under another identity:
    /// ask before relinking to it.
    AsksFirst {
        recorded: uuid::Uuid,
        found: uuid::Uuid,
    },
}

impl Model {
    /// Where the design library `library` stands: In project when the
    /// project file holds it, its folder's status when it is referenced,
    /// and none when the project has no such library.
    pub fn reference_status(&self, library: &str) -> Option<ReferenceStatus> {
        let Some(reference) = self.library_manager.reference(library) else {
            return self
                .library_manager
                .get_library(library)
                .map(|_| ReferenceStatus::InProject);
        };
        let Some(session) = self.library_files.library(library) else {
            return Some(ReferenceStatus::NotLoaded);
        };
        let read_for_session = match &session.load {
            ReferenceLoad::Loaded => None,
            ReferenceLoad::ReadForSession(folder) => Some(folder.clone()),
            ReferenceLoad::NotLoaded => return Some(ReferenceStatus::NotLoaded),
            ReferenceLoad::NotFound(missing) => {
                return Some(ReferenceStatus::NotFound(missing.clone()));
            }
            ReferenceLoad::Unreadable(reason) => {
                return Some(ReferenceStatus::Unreadable(reason.clone()));
            }
            ReferenceLoad::DifferentLibrary(holds) => {
                return Some(ReferenceStatus::DifferentLibrary {
                    holds: holds.clone(),
                });
            }
        };
        let changed = |edited: bool| {
            session
                .files
                .values()
                .filter(|file| file.edited == edited && matches!(file.on_disk, OnDisk::Changed(_)))
                .count()
        };
        let editing = changed(true);
        if editing > 0 {
            return Some(ReferenceStatus::ChangedSinceEditing { cellviews: editing });
        }
        for file in session.files.values() {
            match &file.on_disk {
                OnDisk::Missing(missing) => {
                    return Some(ReferenceStatus::NotFound(missing.clone()));
                }
                OnDisk::Unreadable(reason) => {
                    return Some(ReferenceStatus::Unreadable(reason.clone()));
                }
                _ => {}
            }
        }
        let changed = changed(false);
        if changed > 0 {
            return Some(ReferenceStatus::Changed { cellviews: changed });
        }
        if let Some((library, project)) = &session.technology_differs {
            return Some(ReferenceStatus::TechnologyDiffers {
                library: library.clone(),
                project: project.clone(),
            });
        }
        let moved_on = session
            .files
            .iter()
            .filter(|(file, session)| saved_with(reference, file, session).is_some())
            .count();
        Some(ReferenceStatus::Found {
            mode: reference.mode,
            moved_on,
            read_for_session,
        })
    }

    /// Where one file of a referenced library stands, for its row on the
    /// Library page and the inspector; none when the library is embedded or
    /// the file is not one this session knows.
    pub fn library_file_status(&self, library: &str, file: &LibraryFile) -> Option<CellViewStatus> {
        let reference = self.library_manager.reference(library)?;
        let session = self.library_files.library(library)?.files.get(file)?;
        let loaded = session.loaded.map_or(0, |loaded| loaded.revision);
        Some(match &session.on_disk {
            OnDisk::AsLoaded => CellViewStatus::Found {
                revision: loaded,
                saved_with: saved_with(reference, file, session),
            },
            OnDisk::Changed(on_disk) if session.edited => CellViewStatus::ChangedSinceEditing {
                loaded,
                on_disk: on_disk.revision,
            },
            OnDisk::Changed(on_disk) => CellViewStatus::Changed {
                loaded,
                on_disk: on_disk.revision,
            },
            OnDisk::Missing(missing) => CellViewStatus::NotFound(missing.clone()),
            OnDisk::Unreadable(reason) => CellViewStatus::Unreadable(reason.clone()),
            OnDisk::TechnologyDiffers => CellViewStatus::TechnologyDiffers,
        })
    }

    /// The folder a referenced library resolves to here.
    pub fn library_folder(&self, library: &str) -> Option<&Path> {
        self.library_files
            .library(library)?
            .folder
            .as_ref()
            .ok()
            .map(PathBuf::as_path)
    }

    /// The lock this session holds on one file of a referenced library.
    pub fn held_library_lock(&self, library: &str, file: &LibraryFile) -> Option<&LibraryFileLock> {
        self.library_files
            .library(library)?
            .files
            .get(file)?
            .lock
            .as_ref()
    }

    /// Every file of a referenced library with edits not yet saved, by
    /// library: each counts as one unsaved document, and the close review
    /// lists them.
    pub fn unsaved_library_files(&self) -> Vec<(String, LibraryFile)> {
        self.library_files
            .libraries()
            .flat_map(|(library, session)| {
                session
                    .files
                    .iter()
                    .filter(|(_, file)| file.edited)
                    .map(move |(file, _)| (library.to_owned(), file.clone()))
            })
            .collect()
    }

    /// The files that changed on disk since this session loaded them, for
    /// the Refresh list.
    pub fn refresh_rows(&self) -> Vec<RefreshRow> {
        self.library_files
            .libraries()
            .flat_map(|(library, session)| {
                session.files.iter().filter_map(move |(file, state)| {
                    let OnDisk::Changed(on_disk) = state.on_disk else {
                        return None;
                    };
                    Some(RefreshRow {
                        library: library.to_owned(),
                        file: file.clone(),
                        loaded: state.loaded.map(|loaded| loaded.revision),
                        on_disk: Some(on_disk.revision),
                        edited: state.edited,
                    })
                })
            })
            .collect()
    }
}

impl Model {
    /// The path Attach would store for `folder`, and the other forms that
    /// name the same folder, for the dialog to show before it attaches.
    pub fn library_path_choice(&self, folder: &Path) -> Result<LibraryPathChoice, String> {
        let roots = self.library_roots();
        rspice_design::library::folder::choose_library_path(
            folder,
            self.project_folder().as_deref(),
            &roots.defined(),
            rspice_design::library::folder::HostPlatform::current(),
        )
        .map_err(|error| error.to_string())
    }

    /// The path-variable table: Preferences values, then the environment.
    pub(crate) fn library_roots(&self) -> LibraryRoots {
        LibraryRoots(
            self.pdk_config
                .environment_variables
                .iter()
                .map(|(name, value)| (name.clone(), value.clone()))
                .collect(),
        )
    }

    /// The folder the project file is in, which relative paths start from.
    pub(crate) fn project_folder(&self) -> Option<PathBuf> {
        self.workspace
            .content
            .project
            .path
            .as_deref()
            .and_then(Path::parent)
            .map(Path::to_path_buf)
    }
}

/// The path variables as Preferences states them; the environment answers
/// a name Preferences leaves unset.
pub(crate) struct LibraryRoots(BTreeMap<String, String>);

impl LibraryRoots {
    pub(crate) fn table(&self) -> rspice_design::library::folder::RootTable<'_> {
        rspice_design::library::folder::RootTable {
            preferences: &self.0,
            environment: &environment_variable,
        }
    }

    fn defined(&self) -> Vec<(String, String)> {
        self.table().defined()
    }
}

fn environment_variable(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// The revision the project last saved `file` with, when the file loaded
/// here has moved on from it.
fn saved_with(
    reference: &LibraryReference,
    file: &LibraryFile,
    session: &FileSession,
) -> Option<u64> {
    let saved = reference.saved_revision(file)?;
    let loaded = session.loaded?;
    (saved.digest != loaded.digest).then_some(saved.revision)
}
