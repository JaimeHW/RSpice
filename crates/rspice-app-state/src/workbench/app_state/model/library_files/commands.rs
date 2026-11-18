//! The commands a page calls on libraries kept in folders of their own.
//!
//! Attach, Detach, Move to its own folder, Embed and a change of access are
//! each one step of the one Undo. Relink, Refresh and Read from folder are
//! not: they change what this session read, not the design. Nothing here
//! writes the project file; Save does, after the library files.

use std::path::Path;

use rspice_design::library::folder::{LibraryFile, LibraryMode, LibraryPath, Missing};
use rspice_project::library_folder::{FolderProblem, FolderRead, read_library_files};
#[cfg(not(target_arch = "wasm32"))]
use rspice_project::library_folder::{LockOutcome, read_library_folder};

use super::super::library_catalog::WorkspacePruned;
use super::folders::{self, FolderReading};
use super::{LibraryReference, Model, OnDisk, ReferenceLoad, RelinkOutcome};

impl Model {
    /// Attach the library in `folder`, read-only, written as `path`.
    /// Refused for a name already in the project, or a folder already
    /// attached. Returns the library's name.
    pub fn attach_library(&mut self, folder: &Path, path: LibraryPath) -> Result<String, String> {
        self.require_library_change("Attach library")?;
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (folder, path);
            Err(
                "A browser cannot keep a library by path: use Embed library from folder."
                    .to_owned(),
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some((other, _)) = self.library_files.libraries().find(|(_, session)| {
                session
                    .folder
                    .as_ref()
                    .is_ok_and(|attached| folders::same_folder(attached, folder))
            }) {
                return Err(format!(
                    "That folder is already attached as library '{other}'."
                ));
            }
            let read =
                read_library_folder(folder).map_err(|problem| folder_refusal(folder, problem))?;
            let name = read.manifest.record.name.clone();
            self.require_new_library_name(&name)?;
            let mut reference = LibraryReference::new(&name, path, read.manifest.record.id);
            let reading = folders::from_read(
                &reference,
                Ok(folder.to_path_buf()),
                &read,
                &self.project_technology(),
                ReferenceLoad::Loaded,
            );
            reference.files = reading.loaded_revisions();
            let before = self.capture_library_place(&name);
            self.put_reading(&name, reading, true)?;
            self.library_manager.insert_reference(reference);
            self.library_lifecycle_changed();
            self.record_library_reference_edit(format!("attach library {name}"), &name, before);
            Ok(name)
        }
    }

    /// Drop the reference to `library`. Its placements stay, unresolved;
    /// returns the drawings that hold them. Refused for the top cell's
    /// library, a library a configuration runs from, one with an open
    /// cellview, with unsaved edits, or placed in another library's layout.
    pub fn detach_library(&mut self, library: &str) -> Result<Vec<String>, String> {
        self.require_library_change("Detach library")?;
        if !self.library_manager.is_referenced(library) {
            return Err(format!(
                "Library '{library}' is held in the project; Detach is for a library kept in a folder of its own."
            ));
        }
        if self.workspace.content.project.root_library == library {
            return Err(format!(
                "Library '{library}' holds the project top cell '{}'. Set another top cell before detaching it.",
                self.workspace.content.project.top_cell
            ));
        }
        self.require_library_closed(library, "detaching it")?;
        super::super::library_catalog::require_no_configuration_roots(
            self,
            &format!("library '{library}'"),
            |root| root.library == library,
        )?;
        let foreign_layouts = self
            .workspace
            .content
            .physical_layout_documents()
            .values()
            .filter(|document| document.owner().library != library)
            .filter(|document| {
                document
                    .instances()
                    .values()
                    .any(|instance| instance.master.library == library)
            })
            .count();
        if foreign_layouts > 0 {
            return Err(format!(
                "{foreign_layouts} layout{} outside library '{library}' place a master from it. Remove those placements first.",
                if foreign_layouts == 1 { "" } else { "s" }
            ));
        }
        self.require_no_library_edits(library, "detaching it")?;
        self.workspace.save_active_schematic(&self.schematic);
        let prefix = format!("{library}/");
        let drawings = self
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.starts_with(&prefix))
            .filter(|(_, schematic)| {
                schematic.document().components.iter().any(|component| {
                    component
                        .library_cell
                        .as_ref()
                        .is_some_and(|binding| binding.library == library)
                })
            })
            .map(|(key, _)| key.clone())
            .collect::<Vec<_>>();
        let before = self.capture_library_place(library);
        self.take_library_out(library);
        self.library_lifecycle_changed();
        self.record_library_reference_edit(format!("detach library {library}"), library, before);
        Ok(drawings)
    }

    /// Write the embedded `library` to a folder of its own at `folder`, take
    /// its parts out of the project file and make it Editable. The folder
    /// must be new or empty.
    pub fn move_library_to_folder(&mut self, library: &str, folder: &Path) -> Result<(), String> {
        self.require_library_change("Move to its own folder")?;
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (library, folder);
            Err("A browser cannot keep a library in a folder of its own.".to_owned())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.library_manager.is_referenced(library) {
                return Err(format!(
                    "Library '{library}' is already kept in a folder of its own."
                ));
            }
            let Some(read_only) = self
                .library_manager
                .get_library(library)
                .map(|record| record.read_only)
            else {
                return Err(format!("Library '{library}' not found."));
            };
            if std::fs::read_dir(folder).is_ok_and(|mut entries| entries.next().is_some()) {
                return Err(format!(
                    "{} is not empty. Choose a new or empty folder.",
                    folder.display()
                ));
            }
            let path = self.library_path_choice(folder)?.stored;
            self.workspace.save_active_schematic(&self.schematic);
            let id = uuid::Uuid::new_v4();
            let image = self
                .workspace
                .content
                .library_image(&self.library_manager, library, id)
                .ok_or_else(|| format!("Library '{library}' not found."))?;
            let created = !folder.exists();
            let mut session = super::ReferenceSession {
                folder: Ok(folder.to_path_buf()),
                load: ReferenceLoad::Loaded,
                technology_differs: None,
                files: std::collections::BTreeMap::new(),
            };
            for file in image.files() {
                let written = image
                    .encode(&file, 1)
                    .and_then(Result::ok)
                    .ok_or_else(|| format!("{file} of library '{library}' cannot be written"))
                    .and_then(|encoded| {
                        rspice_project::library_folder::write_library_file(
                            folder,
                            &file,
                            &encoded.bytes,
                        )
                        .map(|stamp| (encoded, stamp))
                        .map_err(|error| format!("{file} could not be written: {error}"))
                    });
                let (encoded, stamp) = match written {
                    Ok(written) => written,
                    Err(error) => {
                        if created {
                            let _ = std::fs::remove_dir_all(folder);
                        }
                        return Err(format!(
                            "Library '{library}' was not moved, and stays in the project: {error}"
                        ));
                    }
                };
                session.files.insert(
                    file,
                    super::FileSession {
                        loaded: Some(rspice_design::library::folder::FileRevision {
                            revision: 1,
                            digest: encoded.digest,
                        }),
                        content: Some(encoded.digest),
                        stamp,
                        ..super::FileSession::default()
                    },
                );
            }
            let before = self.capture_library_place(library);
            let mut reference = LibraryReference::new(library, path, id);
            reference.mode = if read_only {
                LibraryMode::ReadOnly
            } else {
                LibraryMode::Editable
            };
            reference.files = session
                .files
                .iter()
                .filter_map(|(file, session)| Some((file.clone(), session.loaded?)))
                .collect();
            self.library_manager.insert_reference(reference);
            self.library_files.insert(library, session);
            self.library_lifecycle_changed();
            self.record_library_reference_edit(
                format!("move library {library} to its own folder"),
                library,
                before,
            );
            Ok(())
        }
    }

    /// Copy every file of the referenced `library` into the project and drop
    /// the reference. Refused while a file of it could not be read, or a
    /// layout of it was left out for another technology: the copy would
    /// leave them behind.
    pub fn embed_library(&mut self, library: &str) -> Result<(), String> {
        self.require_library_change("Embed in project")?;
        let Some(session) = self.library_files.library(library) else {
            return Err(format!(
                "Library '{library}' is held in the project already."
            ));
        };
        if !matches!(
            session.load,
            ReferenceLoad::Loaded | ReferenceLoad::ReadForSession(_)
        ) {
            return Err(format!(
                "Library '{library}' has not been read, so there is nothing to copy in. Relink it first."
            ));
        }
        let left_behind = session
            .files
            .iter()
            .filter(|(_, file)| {
                matches!(
                    file.on_disk,
                    OnDisk::Missing(_) | OnDisk::Unreadable(_) | OnDisk::TechnologyDiffers
                )
            })
            .map(|(file, _)| file.to_string())
            .collect::<Vec<_>>();
        if !left_behind.is_empty() {
            return Err(format!(
                "These files of library '{library}' were not read, and a copy would leave them behind: {}.",
                left_behind.join(", ")
            ));
        }
        self.workspace.save_active_schematic(&self.schematic);
        let before = self.capture_library_place(library);
        self.release_library_locks_of(library);
        self.library_files.remove(library);
        self.library_manager.remove_reference(library);
        if let Some(mut edit) = self.library_manager.edit_library(library) {
            edit.set_read_only(false);
        }
        self.library_lifecycle_changed();
        self.record_library_reference_edit(
            format!("embed library {library} in the project"),
            library,
            before,
        );
        Ok(())
    }

    /// Embed a copy of the library in `folder` straight away. Returns its
    /// name.
    pub fn embed_library_from_folder(&mut self, folder: &Path) -> Result<String, String> {
        self.require_library_change("Embed library from folder")?;
        #[cfg(target_arch = "wasm32")]
        {
            let _ = folder;
            Err("A browser reads a folder by upload: pick its files.".to_owned())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let read =
                read_library_folder(folder).map_err(|problem| folder_refusal(folder, problem))?;
            self.embed_read_library(&read)
        }
    }

    /// Embed a copy of a library from a browser's folder upload: each file
    /// by its path inside the folder. Returns its name.
    pub fn embed_library_from_files(
        &mut self,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<String, String> {
        self.require_library_change("Embed library from folder")?;
        let read = read_library_files(files)
            .map_err(|problem| folder_refusal(Path::new("the picked folder"), problem))?;
        self.embed_read_library(&read)
    }

    fn embed_read_library(&mut self, read: &FolderRead) -> Result<String, String> {
        let name = read.manifest.record.name.clone();
        self.require_new_library_name(&name)?;
        let unread = read
            .cellviews
            .iter()
            .filter(|(_, outcome)| outcome.is_err())
            .map(|(file, _)| file.to_string())
            .collect::<Vec<_>>();
        if !unread.is_empty() {
            return Err(format!(
                "These files of library '{name}' cannot be read, and a copy would leave them behind: {}.",
                unread.join(", ")
            ));
        }
        let image = read.image();
        if let Some((library, project)) =
            folders::technology_differs(&image.manifest.technology, &self.project_technology())
            && image.cellviews.values().any(|file| file.layout.is_some())
        {
            return Err(format!(
                "Library '{name}' holds layouts drawn for {library}; this project is pinned to {project}."
            ));
        }
        let before = self.capture_library_place(&name);
        self.workspace.content.insert_library_image(
            &mut self.library_manager,
            &image,
            false,
            &|_| true,
        )?;
        self.library_lifecycle_changed();
        self.record_library_reference_edit(format!("embed library {name}"), &name, before);
        Ok(name)
    }

    /// In a browser, read a referenced library from a picked folder for this
    /// session only, read-only; the saved project is left as it was.
    pub fn read_library_for_session(
        &mut self,
        library: &str,
        folder_name: &str,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<(), String> {
        let reference = self
            .library_manager
            .reference(library)
            .cloned()
            .ok_or_else(|| format!("Library '{library}' is not kept in a folder of its own."))?;
        self.require_library_closed(library, "reading it again")?;
        let read = read_library_files(files)
            .map_err(|problem| folder_refusal(Path::new(folder_name), problem))?;
        let reading = folders::from_read(
            &reference,
            Err(Missing::NoFolder(folder_name.to_owned())),
            &read,
            &self.project_technology(),
            ReferenceLoad::ReadForSession(folder_name.to_owned()),
        );
        if let ReferenceLoad::DifferentLibrary(holds) = &reading.session.load {
            return Err(format!(
                "{folder_name} holds library '{holds}', not '{library}' as this project knows it."
            ));
        }
        self.put_reading(library, reading, true)
    }

    /// Make a referenced library Editable or Read-only. Make read-only is
    /// refused while the library has unsaved edits.
    pub fn set_library_mode(&mut self, library: &str, mode: LibraryMode) -> Result<(), String> {
        self.require_library_change("Changing a library's access")?;
        let current = self
            .library_manager
            .reference(library)
            .map(|reference| reference.mode)
            .ok_or_else(|| format!("Library '{library}' is not kept in a folder of its own."))?;
        if current == mode {
            return Ok(());
        }
        if !mode.is_editable() {
            self.require_no_library_edits(library, "making it read-only")?;
        }
        let before = self.capture_library_place(library);
        if !mode.is_editable() {
            self.release_library_locks_of(library);
        }
        if let Some(reference) = self.library_manager.reference_mut(library) {
            reference.mode = mode;
        }
        if let Some(mut edit) = self.library_manager.edit_library(library) {
            edit.set_read_only(!mode.is_editable());
        }
        self.library_lifecycle_changed();
        let description = if mode.is_editable() {
            format!("make library {library} editable")
        } else {
            format!("make library {library} read-only")
        };
        self.record_library_reference_edit(description, library, before);
        Ok(())
    }

    /// Point `library` at another folder whose manifest names the same
    /// library. A folder holding it under another identity is relinked only
    /// once `accept_other_identity` says the user agreed.
    pub fn relink_library(
        &mut self,
        library: &str,
        folder: &Path,
        accept_other_identity: bool,
    ) -> Result<RelinkOutcome, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (library, folder, accept_other_identity);
            Err(
                "A browser reads a library's folder for this session: use Read from folder."
                    .to_owned(),
            )
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut reference = self
                .library_manager
                .reference(library)
                .cloned()
                .ok_or_else(|| {
                    format!("Library '{library}' is not kept in a folder of its own.")
                })?;
            self.require_library_closed(library, "relinking it")?;
            self.require_no_library_edits(library, "relinking it")?;
            let read =
                read_library_folder(folder).map_err(|problem| folder_refusal(folder, problem))?;
            let manifest = &read.manifest.record;
            if manifest.name != library {
                return Err(format!(
                    "{} holds library '{}', not '{library}'.",
                    folder.display(),
                    manifest.name
                ));
            }
            if manifest.id != reference.id && !accept_other_identity {
                return Ok(RelinkOutcome::AsksFirst {
                    recorded: reference.id,
                    found: manifest.id,
                });
            }
            reference.path = self.library_path_choice(folder)?.stored;
            reference.id = manifest.id;
            let reading = folders::from_read(
                &reference,
                Ok(folder.to_path_buf()),
                &read,
                &self.project_technology(),
                ReferenceLoad::Loaded,
            );
            self.put_reading(library, reading, !reference.mode.is_editable())?;
            self.library_manager.insert_reference(reference);
            Ok(RelinkOutcome::Relinked)
        }
    }

    /// "Keep my version": the next save writes this session's edits over a
    /// file that changed on disk since editing began.
    pub fn keep_my_library_file(
        &mut self,
        library: &str,
        file: &LibraryFile,
    ) -> Result<(), String> {
        self.refresh_library_edits();
        let session = self
            .library_files
            .library_mut(library)
            .and_then(|session| session.files.get_mut(file))
            .ok_or_else(|| format!("{library}/{file} is not a file this session read."))?;
        if !(session.edited && matches!(session.on_disk, OnDisk::Changed(_))) {
            return Err(format!(
                "{library}/{file} has not changed on disk since you started editing it."
            ));
        }
        session.keep_mine = true;
        Ok(())
    }

    /// "Take the file's version": discard this session's edits to the file
    /// and load it as it is on disk. The returned work brings the editor to
    /// the drawing when it is the one in front.
    pub fn take_library_file_from_disk(
        &mut self,
        library: &str,
        file: &LibraryFile,
    ) -> Result<WorkspacePruned, String> {
        let restore = self.reload_library_file(library, file)?;
        self.release_library_lock(library, file);
        Ok(WorkspacePruned {
            restore_active_schematic: restore,
            ..WorkspacePruned::default()
        })
    }

    /// Reload the picked files from disk. A file with edits here is never
    /// overwritten: the whole refresh is refused, naming it, before any
    /// file is read. Returns how many were reloaded, and the work that
    /// brings the editor to the drawing in front when it was one of them.
    pub fn refresh_library_files(
        &mut self,
        picked: &[(String, LibraryFile)],
    ) -> Result<(usize, WorkspacePruned), String> {
        self.refresh_library_edits();
        let mut refused = Vec::new();
        for (library, file) in picked {
            let session = self
                .library_files
                .library(library)
                .and_then(|session| session.files.get(file));
            match session {
                Some(session) if session.edited => refused.push(format!(
                    "{library}/{file}: {}",
                    rspice_design::library::folder::CHANGED_SINCE_EDITING
                )),
                Some(session) if matches!(session.on_disk, OnDisk::Changed(_)) => {}
                _ => refused.push(format!("{library}/{file}: not changed on disk")),
            }
        }
        if !refused.is_empty() {
            return Err(format!("Nothing was refreshed. {}.", refused.join("; ")));
        }
        let mut restore = false;
        for (library, file) in picked {
            restore |= self.reload_library_file(library, file)?;
        }
        Ok((
            picked.len(),
            WorkspacePruned {
                restore_active_schematic: restore,
                ..WorkspacePruned::default()
            },
        ))
    }

    /// Remove another holder's lock on one file: "Break lock…", which names
    /// the holder first.
    pub fn break_library_lock(&mut self, library: &str, file: &LibraryFile) -> Result<(), String> {
        let folder = self
            .library_folder(library)
            .map(Path::to_path_buf)
            .ok_or_else(|| format!("Library '{library}' has no folder here."))?;
        rspice_project::library_folder::break_library_lock(&folder, file).map_err(|error| {
            format!("The lock on {library}/{file} could not be removed: {error}")
        })?;
        if let Some(session) = self
            .library_files
            .library_mut(library)
            .and_then(|session| session.files.get_mut(file))
        {
            session.lock = None;
        }
        Ok(())
    }

    /// The edit gate for a file of a referenced library: the first edit
    /// takes its lock. Refused when the library is read-only, the file is
    /// locked elsewhere ("Being edited by jsmith on LAB-PC3"), it changed on
    /// disk since it was loaded ("Refresh it first"), or it cannot be
    /// written. A library held in the project needs no claim.
    pub fn claim_library_file(&mut self, library: &str, file: &LibraryFile) -> Result<(), String> {
        let Some(reference) = self.library_manager.reference(library) else {
            return Ok(());
        };
        if !reference.mode.is_editable() {
            return Err(format!(
                "Library '{library}' is read-only. Make it editable to change it."
            ));
        }
        let session = self
            .library_files
            .library(library)
            .ok_or_else(|| format!("Library '{library}' has not been read."))?;
        if !matches!(session.load, ReferenceLoad::Loaded) {
            return Err(format!(
                "Library '{library}' was not read from its folder, so it cannot be changed here."
            ));
        }
        if let Some(state) = session.files.get(file) {
            match &state.on_disk {
                OnDisk::AsLoaded => {}
                OnDisk::Changed(_) => {
                    return Err(format!(
                        "{library}/{file} changed on disk since it was loaded. Refresh it first."
                    ));
                }
                OnDisk::TechnologyDiffers => {
                    return Err(format!(
                        "{library}/{file} was drawn for another technology and was not read."
                    ));
                }
                OnDisk::Missing(_) | OnDisk::Unreadable(_) => {
                    return Err(format!("{library}/{file} could not be read."));
                }
            }
            if state.lock.is_some() {
                return Ok(());
            }
        }
        self.take_library_lock(library, file)
    }

    #[cfg(target_arch = "wasm32")]
    fn take_library_lock(&mut self, _: &str, _: &LibraryFile) -> Result<(), String> {
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn take_library_lock(&mut self, library: &str, file: &LibraryFile) -> Result<(), String> {
        let folder = self
            .library_folder(library)
            .map(Path::to_path_buf)
            .ok_or_else(|| format!("Library '{library}' has no folder here."))?;
        let lock = folders::own_lock(self.lock_project_name());
        let outcome = rspice_project::library_folder::take_library_lock(
            &folder,
            file,
            &lock,
            &rspice_output::process_liveness,
        )
        .map_err(|error| format!("{library}/{file} cannot be written: {error}"))?;
        match outcome {
            LockOutcome::Taken | LockOutcome::AlreadyHeld | LockOutcome::TakenOver(_) => {
                if let Some(session) = self.library_files.library_mut(library) {
                    session.files.entry(file.clone()).or_default().lock = Some(lock);
                }
                Ok(())
            }
            LockOutcome::HeldBy(holder) => Err(format!("Being edited by {}", holder.holder())),
        }
    }

    /// The name a lock gives this project: its file, or its identity while
    /// it has none.
    pub(super) fn lock_project_name(&self) -> String {
        self.workspace.content.project.path.as_ref().map_or_else(
            || self.workspace.content.project.id().to_string(),
            |path| path.display().to_string(),
        )
    }

    /// Release every lock this session holds: on close, and before another
    /// project replaces this one.
    pub fn release_library_locks(&mut self) {
        let libraries = self
            .library_files
            .libraries()
            .map(|(name, _)| name.to_owned())
            .collect::<Vec<_>>();
        for library in libraries {
            self.release_library_locks_of(&library);
        }
    }

    pub(crate) fn release_library_locks_of(&mut self, library: &str) {
        let files = self
            .library_files
            .library(library)
            .map(|session| {
                session
                    .files
                    .iter()
                    .filter(|(_, file)| file.lock.is_some())
                    .map(|(file, _)| file.clone())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        for file in files {
            self.release_library_lock(library, &file);
        }
    }

    fn release_library_lock(&mut self, library: &str, file: &LibraryFile) {
        let folder = self.library_folder(library).map(Path::to_path_buf);
        let Some(state) = self
            .library_files
            .library_mut(library)
            .and_then(|session| session.files.get_mut(file))
        else {
            return;
        };
        if let (Some(lock), Some(folder)) = (state.lock.take(), folder) {
            let _ = rspice_project::library_folder::release_library_lock(&folder, file, &lock);
        }
    }

    /// Mark each file of each read library whose parts here encode to other
    /// bytes than it loaded as edited, and each part this session made as a
    /// file to write. Runs on check and on Save; never on a frame.
    pub fn refresh_library_edits(&mut self) {
        let names = self
            .library_files
            .libraries()
            .filter(|(_, session)| {
                matches!(
                    session.load,
                    ReferenceLoad::Loaded | ReferenceLoad::ReadForSession(_)
                )
            })
            .map(|(name, _)| name.to_owned())
            .collect::<Vec<_>>();
        for name in names {
            let Some(id) = self
                .library_manager
                .reference(&name)
                .map(|reference| reference.id)
            else {
                continue;
            };
            let Some(image) =
                self.workspace
                    .content
                    .library_image(&self.library_manager, &name, id)
            else {
                continue;
            };
            let Some(session) = self.library_files.library_mut(&name) else {
                continue;
            };
            for file in image.files() {
                session.files.entry(file).or_default();
            }
            for (file, state) in &mut session.files {
                if matches!(state.on_disk, OnDisk::TechnologyDiffers) {
                    state.edited = false;
                    continue;
                }
                let revision = state.loaded.map_or(0, |loaded| loaded.revision);
                let now = image
                    .encode(file, revision)
                    .and_then(Result::ok)
                    .map(|encoded| encoded.digest);
                state.edited = now != state.content;
            }
        }
    }

    /// Load one file of a referenced library as it is on disk now, over
    /// whatever this session holds of it. Returns whether it was the
    /// drawing in front.
    #[cfg(target_arch = "wasm32")]
    fn reload_library_file(&mut self, library: &str, file: &LibraryFile) -> Result<bool, String> {
        Err(format!(
            "{library}/{file} cannot be read again here: read the folder for this session again."
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn reload_library_file(&mut self, library: &str, file: &LibraryFile) -> Result<bool, String> {
        use rspice_design::library::folder::{
            CellFile, CellViewFile, FileRevision, LibraryManifest, digest_file_bytes,
            encode_library_file,
        };
        let folder = self
            .library_folder(library)
            .map(Path::to_path_buf)
            .ok_or_else(|| format!("Library '{library}' has no folder here."))?;
        let (bytes, stamp) = rspice_project::library_folder::read_library_file(&folder, file)
            .map_err(|problem| format!("{library}/{file} could not be read: {problem:?}"))?;
        let digest = digest_file_bytes(&bytes);
        let unreadable = |error: rspice_design::library::folder::LibraryFileError| {
            format!("{library}/{file} could not be read: {error}")
        };
        let mut restore = false;
        let (revision, content) = match file {
            LibraryFile::Manifest => {
                let record = LibraryManifest::decode(&bytes).map_err(unreadable)?;
                self.edit_library_record(library, |current| {
                    current.technology = record.technology.clone();
                    current.metadata = record.metadata.clone().into_iter().collect();
                })?;
                (record.revision, encode_library_file(&record))
            }
            LibraryFile::Cell { cell } => {
                let record = CellFile::decode(&bytes).map_err(unreadable)?;
                self.edit_library_record(library, |current| {
                    let mut updated = record.cell();
                    if let Some(existing) = current.cells.get(cell) {
                        updated.views = existing.views.clone();
                    }
                    current.cells.insert(cell.clone(), updated);
                })?;
                (record.revision, encode_library_file(&record))
            }
            LibraryFile::CellView { cell, .. } => {
                let record = CellViewFile::decode(&bytes).map_err(unreadable)?;
                let reference = crate::state::CellViewRef::new(
                    library,
                    cell,
                    file.view_name().unwrap_or_default(),
                );
                self.edit_library_record(library, |current| {
                    current
                        .cells
                        .entry(cell.clone())
                        .or_insert_with(|| crate::state::Cell::new(cell.clone()))
                        .add_view(record.view.view());
                })?;
                self.workspace
                    .content
                    .insert_cell_view_parts(library, file, &record, &|_| true)?;
                let key = reference.key();
                self.workspace.session.schematic_sessions.remove(&key);
                restore = self.workspace.content.active_view == reference;
                (record.revision, encode_library_file(&record))
            }
        };
        let content = content.ok().map(|encoded| encoded.digest);
        if let Some(session) = self.library_files.library_mut(library) {
            let state = session.files.entry(file.clone()).or_default();
            state.loaded = Some(FileRevision { revision, digest });
            state.content = content;
            state.stamp = stamp;
            state.on_disk = OnDisk::AsLoaded;
            state.edited = false;
            state.keep_mine = false;
        }
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        Ok(restore)
    }

    /// Rewrite one catalog library's own record, its cells kept.
    #[cfg(not(target_arch = "wasm32"))]
    fn edit_library_record(
        &mut self,
        library: &str,
        edit: impl FnOnce(&mut crate::state::Library),
    ) -> Result<(), String> {
        let mut record = self
            .library_manager
            .get_library(library)
            .cloned()
            .ok_or_else(|| format!("Library '{library}' is not loaded."))?;
        edit(&mut record);
        self.library_manager.remove_library(library);
        self.library_manager.add_library(record);
        Ok(())
    }

    /// Replace whatever the project holds of `library` with a folder's
    /// reading: its catalog record and parts, and this session's record.
    fn put_reading(
        &mut self,
        library: &str,
        reading: FolderReading,
        read_only: bool,
    ) -> Result<(), String> {
        if self.library_manager.get_library(library).is_some() {
            self.workspace
                .content
                .remove_library_parts(&self.library_manager, library);
            self.library_manager.remove_library(library);
        }
        self.release_library_locks_of(library);
        if let Some(image) = &reading.image {
            self.workspace.content.insert_library_image(
                &mut self.library_manager,
                image,
                read_only,
                &|_| true,
            )?;
        }
        self.library_files.insert(library, reading.session);
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        Ok(())
    }

    /// Take a library wholly out of the project: its parts, its catalog
    /// record, its reference and this session's record, with its locks.
    pub(crate) fn take_library_out(&mut self, library: &str) {
        self.release_library_locks_of(library);
        if self.library_manager.get_library(library).is_some() {
            self.workspace
                .content
                .remove_library_parts(&self.library_manager, library);
            self.library_manager.remove_library(library);
        }
        self.library_manager.remove_reference(library);
        self.library_files.remove(library);
    }

    pub(crate) fn library_lifecycle_changed(&mut self) {
        self.workspace.content.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
    }

    fn require_library_change(&self, operation: &str) -> Result<(), String> {
        if self.safe_mode.project_read_only() {
            return Err(format!(
                "{operation} is unavailable because the project is open read-only."
            ));
        }
        Ok(())
    }

    fn require_new_library_name(&self, name: &str) -> Result<(), String> {
        if self.library_manager.get_library(name).is_some()
            || self.library_manager.is_referenced(name)
        {
            return Err(format!("The project already has a library named '{name}'."));
        }
        if let Some(conflict) = super::super::library_catalog::conflicting_library_name(
            &self.library_manager,
            name,
            None,
        ) {
            return Err(format!(
                "Library '{name}' conflicts with the existing library '{conflict}', whose name differs only in case or normalization."
            ));
        }
        Ok(())
    }

    /// Refuse while a cellview of `library` is open in a tab.
    pub(crate) fn require_library_closed(&self, library: &str, doing: &str) -> Result<(), String> {
        let open = self
            .workspace
            .content
            .open_views
            .iter()
            .filter(|open| open.reference.library == library)
            .map(|open| open.reference.display_path())
            .collect::<Vec<_>>();
        if open.is_empty() {
            return Ok(());
        }
        Err(format!("Close {} before {doing}.", open.join(", ")))
    }

    fn require_no_library_edits(&mut self, library: &str, doing: &str) -> Result<(), String> {
        self.workspace.save_active_schematic(&self.schematic);
        self.refresh_library_edits();
        let edited = self
            .library_files
            .library(library)
            .map(|session| session.files.values().filter(|file| file.edited).count())
            .unwrap_or_default();
        if edited == 0 {
            return Ok(());
        }
        Err(format!(
            "{edited} file{} of library '{library}' {} unsaved edits. Save or take the file's version before {doing}.",
            if edited == 1 { "" } else { "s" },
            if edited == 1 { "has" } else { "have" }
        ))
    }

    pub(super) fn project_technology(&self) -> String {
        self.workspace
            .content
            .project
            .technology
            .clone()
            .unwrap_or_default()
    }
}

fn folder_refusal(folder: &Path, problem: FolderProblem) -> String {
    match problem {
        FolderProblem::NoFolder => format!("{} is not a folder.", folder.display()),
        FolderProblem::Unreadable(reason) => {
            format!(
                "{} cannot be read as a library: {reason}.",
                folder.display()
            )
        }
    }
}
