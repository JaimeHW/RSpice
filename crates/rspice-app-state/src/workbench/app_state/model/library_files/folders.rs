//! Reading a referenced library's folder into the project, and the lock
//! this session writes beside a file it edits.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_design::library::folder::{HostPlatform, LibraryFile, LibraryFileLock, Missing};
use rspice_project::library_folder::{FileProblem, FolderProblem, FolderRead};
use rspice_project::{LibraryImage, ProjectFile};

use super::{
    FileSession, LibraryFileSession, LibraryReference, LibraryRoots, Model, OnDisk, ReferenceLoad,
    ReferenceSession,
};

/// What reading one folder came to: the session's record of it and, when it
/// could be read, the parts to put into the project.
pub(super) struct FolderReading {
    pub session: ReferenceSession,
    pub image: Option<LibraryImage>,
}

impl FolderReading {
    fn not_read(folder: Result<PathBuf, Missing>, load: ReferenceLoad) -> Self {
        Self {
            session: ReferenceSession {
                folder,
                load,
                technology_differs: None,
                files: BTreeMap::new(),
            },
            image: None,
        }
    }

    /// Each file's revision as this session loaded it, for the reference's
    /// record of what the project saved with.
    pub(super) fn loaded_revisions(
        &self,
    ) -> BTreeMap<LibraryFile, rspice_design::library::folder::FileRevision> {
        self.session
            .files
            .iter()
            .filter_map(|(file, session)| Some((file.clone(), session.loaded?)))
            .collect()
    }
}

/// The folder `reference` names on this computer, or why it names none.
pub(super) fn resolve_folder(
    reference: &LibraryReference,
    project_folder: Option<&Path>,
    roots: &LibraryRoots,
) -> Result<PathBuf, Missing> {
    reference
        .path
        .resolve(project_folder, &roots.table(), HostPlatform::current())
        .map_err(Missing::Path)
}

/// Read the folder `folder` as the library `reference` names.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn read_folder(
    reference: &LibraryReference,
    folder: Result<PathBuf, Missing>,
    project_technology: &str,
) -> FolderReading {
    let path = match &folder {
        Ok(path) => path.clone(),
        Err(missing) => {
            let load = ReferenceLoad::NotFound(missing.clone());
            return FolderReading::not_read(folder, load);
        }
    };
    match rspice_project::library_folder::read_library_folder(&path) {
        Err(FolderProblem::NoFolder) => FolderReading::not_read(
            folder,
            ReferenceLoad::NotFound(Missing::NoFolder(path.display().to_string())),
        ),
        Err(FolderProblem::Unreadable(reason)) => {
            FolderReading::not_read(folder, ReferenceLoad::Unreadable(reason))
        }
        Ok(read) => from_read(
            reference,
            folder,
            &read,
            project_technology,
            ReferenceLoad::Loaded,
        ),
    }
}

/// The session and parts of a folder read whole. A folder whose manifest
/// names another library, or this one under another identity, loads
/// nothing. A layout drawn for another technology than the project's is
/// left out, and its file says so.
pub(super) fn from_read(
    reference: &LibraryReference,
    folder: Result<PathBuf, Missing>,
    read: &FolderRead,
    project_technology: &str,
    load: ReferenceLoad,
) -> FolderReading {
    let manifest = &read.manifest.record;
    if manifest.name != reference.name || manifest.id != reference.id {
        let holds = manifest.name.clone();
        return FolderReading::not_read(folder, ReferenceLoad::DifferentLibrary(holds));
    }
    let mut image = read.image();
    let mut files = BTreeMap::new();
    for (file, outcome) in read.files() {
        let session = match outcome {
            Ok((revision, stamp)) => FileSession {
                loaded: Some(revision),
                content: image
                    .encode(&file, revision.revision)
                    .and_then(Result::ok)
                    .map(|encoded| encoded.digest),
                stamp,
                ..FileSession::default()
            },
            Err(FileProblem::Missing) => FileSession {
                on_disk: OnDisk::Missing(Missing::NoFile(file.relative_path())),
                ..FileSession::default()
            },
            Err(FileProblem::Unreadable(reason)) => FileSession {
                on_disk: OnDisk::Unreadable(reason),
                ..FileSession::default()
            },
        };
        files.insert(file, session);
    }
    let technology_differs = technology_differs(&manifest.technology, project_technology);
    if technology_differs.is_some() {
        for (file, record) in &mut image.cellviews {
            if record.layout.take().is_some()
                && let Some(session) = files.get_mut(file)
            {
                session.on_disk = OnDisk::TechnologyDiffers;
            }
        }
    }
    FolderReading {
        session: ReferenceSession {
            folder,
            load,
            technology_differs,
            files,
        },
        image: Some(image),
    }
}

/// The two technology pins, library's then project's, when both are set
/// and differ.
pub(super) fn technology_differs(library: &str, project: &str) -> Option<(String, String)> {
    let (library, project) = (library.trim(), project.trim());
    (!library.is_empty() && !project.is_empty() && library != project)
        .then(|| (library.to_owned(), project.to_owned()))
}

impl Model {
    /// Read every library `file` references from its folder into `file`
    /// itself, before the project is installed: its cells into the catalog,
    /// its drawings, layouts and sources into the stores, so the open
    /// project and its saved baseline both hold them as read. Returns what
    /// this session read of each folder. A browser reads no folder by path:
    /// each reference reads Not loaded.
    pub fn read_referenced_libraries(&self, file: &mut ProjectFile) -> LibraryFileSession {
        let mut session = LibraryFileSession::default();
        let references = file.libraries.references().cloned().collect::<Vec<_>>();
        #[cfg(target_arch = "wasm32")]
        for reference in references {
            let folder = Err(Missing::NoFolder(
                reference.path.shown(HostPlatform::current()),
            ));
            session.insert(
                reference.name,
                FolderReading::not_read(folder, ReferenceLoad::NotLoaded).session,
            );
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let roots = self.library_roots();
            let project_folder = file
                .workspace
                .project
                .path
                .as_deref()
                .and_then(Path::parent)
                .map(Path::to_path_buf);
            let technology = file
                .workspace
                .project
                .technology
                .clone()
                .unwrap_or_default();
            for reference in references {
                let folder = resolve_folder(&reference, project_folder.as_deref(), &roots);
                let reading = read_folder(&reference, folder, &technology);
                let mut reading = reading;
                if let Some(image) = &reading.image
                    && let Err(reason) = file.workspace.insert_library_image(
                        &mut file.libraries,
                        image,
                        !reference.mode.is_editable(),
                        &|_| true,
                    )
                {
                    reading.session.load = ReferenceLoad::Unreadable(reason);
                    reading.session.files.clear();
                }
                session.insert(reference.name, reading.session);
            }
        }
        session
    }
}

/// The lock this session writes on a file it edits, for the project at
/// `project`.
pub(super) fn own_lock(project: String) -> LibraryFileLock {
    let first = |names: &[&str], otherwise: &str| {
        names
            .iter()
            .find_map(|name| {
                std::env::var(name)
                    .ok()
                    .filter(|value| !value.trim().is_empty())
            })
            .unwrap_or_else(|| otherwise.to_owned())
    };
    LibraryFileLock {
        user: first(&["USERNAME", "USER", "LOGNAME"], "someone"),
        computer: first(&["COMPUTERNAME", "HOSTNAME"], "this computer"),
        process: std::process::id(),
        project,
        since_unix_ms: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |elapsed| {
                u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
            }),
    }
}

/// Whether two paths name one folder, as far as this computer can tell.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn same_folder(left: &Path, right: &Path) -> bool {
    match (std::fs::canonicalize(left), std::fs::canonicalize(right)) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}
