//! Frozen copies of referenced libraries for publications.
//!
//! A publication is self-contained. The project file it records names each
//! library kept in a folder of its own by path alone, so every file of each
//! such library the design holds is frozen in the project's publication
//! store, under its content digest: one copy per digest, however many
//! publications hold it. Restoring a publication rebuilds those libraries
//! from the store, as they were published, and the next check names each
//! file whose folder has moved on since.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_design::library::folder::{
    CellFile, CellViewFile, FileRevision, LibraryFile, LibraryManifest, digest_file_bytes,
};
use rspice_project::{FrozenCellView, LibraryImage, ProjectFile};

use super::{
    FileSession, LibraryFileSession, LibraryRoots, Model, OnDisk, ReferenceLoad, ReferenceSession,
    folders,
};

/// The folder beside the project file that holds frozen copies.
pub const PUBLICATION_STORE_FOLDER: &str = "publications";

/// The project's publication store: beside its file.
pub fn publication_store(project_file: &Path) -> Option<PathBuf> {
    Some(project_file.parent()?.join(PUBLICATION_STORE_FOLDER))
}

fn stored_copy(store: &Path, digest: &rspice_vocabulary::product::ContentDigest) -> PathBuf {
    store.join(format!("{digest}.json"))
}

impl Model {
    /// Freeze every file of every library the project keeps in a folder of
    /// its own into `store`, as the project holds it now, and name each for
    /// the publication. Refused for a library this session could not read:
    /// the publication could not hold it.
    pub fn freeze_referenced_libraries(&self, store: &Path) -> Result<Vec<FrozenCellView>, String> {
        let mut frozen = Vec::new();
        for reference in self.library_manager.references() {
            let name = &reference.name;
            let readable = self.library_files.library(name).is_some_and(|session| {
                matches!(
                    session.load,
                    ReferenceLoad::Loaded | ReferenceLoad::ReadForSession(_)
                )
            });
            if !readable {
                return Err(format!(
                    "Library '{name}' has not been read, so a publication cannot hold it."
                ));
            }
            let image = self
                .workspace
                .content
                .library_image(&self.library_manager, name, reference.id)
                .ok_or_else(|| format!("Library '{name}' has not been read."))?;
            let session = self.library_files.library(name);
            for file in image.files() {
                let revision = session
                    .and_then(|session| session.files.get(&file))
                    .and_then(|state| state.loaded)
                    .map_or(0, |loaded| loaded.revision);
                let encoded = image
                    .encode(&file, revision)
                    .and_then(Result::ok)
                    .ok_or_else(|| format!("{name}/{file} cannot be frozen."))?;
                let copy = stored_copy(store, &encoded.digest);
                if !copy.is_file() {
                    write_once(&copy, &encoded.bytes)
                        .map_err(|error| format!("{name}/{file} could not be frozen: {error}"))?;
                }
                frozen.push(FrozenCellView {
                    library: name.clone(),
                    file,
                    revision,
                    digest: encoded.digest,
                });
            }
        }
        Ok(frozen)
    }

    /// Rebuild in `file`, from `store`, every library a publication froze,
    /// as it was published, and the session's record of each. A copy that
    /// is missing or altered refuses the restore, naming it.
    pub fn thaw_frozen_libraries(
        &self,
        file: &mut ProjectFile,
        store: &Path,
        frozen: &[FrozenCellView],
    ) -> Result<LibraryFileSession, String> {
        let mut by_library = BTreeMap::<&str, Vec<&FrozenCellView>>::new();
        for copy in frozen {
            by_library.entry(&copy.library).or_default().push(copy);
        }
        let roots: LibraryRoots = self.library_roots();
        let project_folder = file
            .workspace
            .project
            .path
            .as_deref()
            .and_then(Path::parent)
            .map(Path::to_path_buf);
        let mut session = LibraryFileSession::default();
        for (library, copies) in by_library {
            let reference = file.libraries.reference(library).cloned().ok_or_else(|| {
                format!("The publication names library '{library}' but not its folder.")
            })?;
            let mut manifest = None;
            let mut cells = BTreeMap::new();
            let mut cellviews = BTreeMap::new();
            let mut files = BTreeMap::new();
            for copy in copies {
                let bytes = std::fs::read(stored_copy(store, &copy.digest)).map_err(|error| {
                    format!(
                        "The frozen copy of {library}/{} cannot be read: {error}",
                        copy.file
                    )
                })?;
                if digest_file_bytes(&bytes) != copy.digest {
                    return Err(format!(
                        "The frozen copy of {library}/{} was altered.",
                        copy.file
                    ));
                }
                let damaged = |error: rspice_design::library::folder::LibraryFileError| {
                    format!(
                        "The frozen copy of {library}/{} cannot be read: {error}",
                        copy.file
                    )
                };
                match &copy.file {
                    LibraryFile::Manifest => {
                        manifest = Some(LibraryManifest::decode(&bytes).map_err(damaged)?);
                    }
                    LibraryFile::Cell { cell } => {
                        cells.insert(cell.clone(), CellFile::decode(&bytes).map_err(damaged)?);
                    }
                    LibraryFile::CellView { .. } => {
                        cellviews.insert(
                            copy.file.clone(),
                            CellViewFile::decode(&bytes).map_err(damaged)?,
                        );
                    }
                }
                files.insert(
                    copy.file.clone(),
                    FileSession {
                        loaded: Some(FileRevision {
                            revision: copy.revision,
                            digest: copy.digest,
                        }),
                        content: Some(copy.digest),
                        stamp: None,
                        on_disk: OnDisk::AsLoaded,
                        ..FileSession::default()
                    },
                );
            }
            let manifest = manifest.ok_or_else(|| {
                format!("The publication holds no manifest of library '{library}'.")
            })?;
            let image = LibraryImage {
                manifest,
                cells,
                cellviews,
            };
            file.workspace.insert_library_image(
                &mut file.libraries,
                &image,
                !reference.mode.is_editable(),
                &|_| true,
            )?;
            session.insert(
                library,
                ReferenceSession {
                    folder: folders::resolve_folder(&reference, project_folder.as_deref(), &roots),
                    load: ReferenceLoad::Loaded,
                    technology_differs: None,
                    files,
                },
            );
        }
        Ok(session)
    }
}

/// Write a frozen copy once: to a file beside it, then into its place.
fn write_once(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    let folder = path.parent().unwrap_or(Path::new("."));
    std::fs::create_dir_all(folder)?;
    let staging = folder.join(format!(".frozen.{}.tmp", std::process::id()));
    let written = (|| {
        let mut staged = std::fs::File::create(&staging)?;
        staged.write_all(bytes)?;
        staged.sync_all()?;
        drop(staged);
        std::fs::rename(&staging, path)
    })();
    if written.is_err() {
        let _ = std::fs::remove_file(&staging);
    }
    written
}
