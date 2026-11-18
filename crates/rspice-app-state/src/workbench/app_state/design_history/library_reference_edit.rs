//! Where a library lives, as a step of the one Undo.
//!
//! Attaching a library kept in a folder of its own, detaching it, moving an
//! embedded library to a folder, embedding one again and changing a
//! referenced library's access each change where a library lives rather
//! than what it holds. One step holds the library on each side: its
//! reference, its catalog record, its parts as its folder's files would
//! hold them, and what this session read of the folder. Undo makes the
//! library what it was and Redo what it became. Neither touches the disk:
//! a library moved to a folder and undone leaves the folder as it was
//! written, and the project file stops naming it.
//!
//! A step stands while the library is as the step left it: the same
//! reference, and every part encoding to the bytes it did. The Edit menu
//! asks only whether the reference and the catalog record are there; the
//! parts are compared when the step is taken.

use super::*;
use crate::state::Library;
use crate::workbench::app_state::model::{LibraryReference, ReferenceSession};
use rspice_design::library::folder::{LibraryFile, encode_library_file};
use rspice_project::LibraryImage;
use rspice_vocabulary::product::ContentDigest;

/// One change of where a library lives.
#[derive(Debug, Clone)]
pub struct LibraryReferenceEditRecord {
    description: String,
    library: String,
    before: LibraryPlace,
    after: LibraryPlace,
}

/// Where one library lives on one side of a step.
#[derive(Debug, Clone, Default)]
pub(crate) struct LibraryPlace {
    reference: Option<LibraryReference>,
    /// The catalog's record of it, exactly, cells and views included.
    record: Option<Library>,
    /// Its drawings, layouts and sources, as its folder's files hold them.
    parts: Option<LibraryImage>,
    /// What every part encodes to, the access flag aside, for the exact
    /// comparison.
    content: Option<BTreeMap<LibraryFile, ContentDigest>>,
    session: Option<ReferenceSession>,
}

impl LibraryPlace {
    fn same_content(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Model {
    /// The library `library` as it stands now, for one side of a step.
    pub(crate) fn capture_library_place(&self, library: &str) -> LibraryPlace {
        let reference = self.library_manager.reference(library).cloned();
        let record = self.library_manager.get_library(library).cloned();
        let parts = record.as_ref().and_then(|_| {
            let id = reference
                .as_ref()
                .map_or_else(uuid::Uuid::nil, |reference| reference.id);
            self.workspace
                .content
                .library_image(&self.library_manager, library, id)
        });
        let content = record.as_ref().zip(parts.as_ref()).map(|(record, parts)| {
            let mut content = parts
                .files()
                .into_iter()
                .filter_map(|file| {
                    let digest = parts.encode(&file, 0)?.ok()?.digest;
                    Some((file, digest))
                })
                .collect::<BTreeMap<_, _>>();
            let mut catalog = record.clone();
            catalog.read_only = false;
            if let Ok(encoded) = encode_library_file(&catalog) {
                content.insert(LibraryFile::Manifest, encoded.digest);
            }
            content
        });
        LibraryPlace {
            reference,
            record,
            parts,
            content,
            session: self.library_files.library(library).cloned(),
        }
    }

    /// Record the change of where `library` lives that began at `before`,
    /// now made, as one step of Undo described by `description`.
    pub(crate) fn record_library_reference_edit(
        &mut self,
        description: String,
        library: &str,
        before: LibraryPlace,
    ) {
        let after = self.capture_library_place(library);
        self.push_project_record(
            RecordHeader::project_only(),
            ProjectDesignBody::LibraryReferenceEdit(Box::new(LibraryReferenceEditRecord {
                description,
                library: library.to_owned(),
                before,
                after,
            })),
        );
    }

    /// Make `library` live as `place` holds it, from `current`.
    fn put_library_place(&mut self, library: &str, current: &LibraryPlace, place: &LibraryPlace) {
        if current.same_content(place) {
            if let Some(record) = &place.record
                && self
                    .library_manager
                    .get_library(library)
                    .is_some_and(|current| current.read_only != record.read_only)
                && let Some(mut edit) = self.library_manager.edit_library(library)
            {
                edit.set_read_only(record.read_only);
            }
            self.release_library_locks_of(library);
            self.library_manager.remove_reference(library);
            self.library_files.remove(library);
        } else {
            self.take_library_out(library);
            if let Some(record) = &place.record {
                self.library_manager.add_library(record.clone());
                if let Some(parts) = &place.parts {
                    for (file, cellview) in &parts.cellviews {
                        // The parts were taken from the stores as they
                        // stood, so each goes back as it was.
                        let _ = self.workspace.content.insert_cell_view_parts(
                            library,
                            file,
                            cellview,
                            &|_| true,
                        );
                    }
                }
            }
        }
        if let Some(reference) = &place.reference {
            self.library_manager.insert_reference(reference.clone());
        }
        if let Some(session) = &place.session {
            let mut session = session.clone();
            // A lock is this session's on disk, never a record's.
            for file in session.files.values_mut() {
                file.lock = None;
            }
            self.library_files.insert(library, session);
        }
        self.library_lifecycle_changed();
    }
}

impl LibraryReferenceEditRecord {
    pub(super) fn description(&self) -> &str {
        &self.description
    }

    fn stands(state: &Model, library: &str, place: &LibraryPlace) -> bool {
        state.library_manager.reference(library) == place.reference.as_ref()
            && state.library_manager.get_library(library).is_some() == place.record.is_some()
    }

    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        Self::stands(state, &self.library, &self.after)
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        Self::stands(state, &self.library, &self.before)
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "Library '{}' cannot be {operation} without an open project.",
                self.library
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "Library '{}' cannot be {operation} while the project is read-only.",
                self.library
            ));
        }
        Ok(())
    }

    /// The exact check: the library's parts encode to what the step left,
    /// and none of its cellviews is open where the step would take them.
    pub(super) fn validate_crossing(&self, state: &Model, forward: bool) -> Result<(), String> {
        let (from, to) = if forward {
            (&self.before, &self.after)
        } else {
            (&self.after, &self.before)
        };
        let operation = if forward { "redone" } else { "undone" };
        let now = state.capture_library_place(&self.library);
        if now.reference != from.reference || !now.same_content(from) {
            return Err(format!(
                "\"{}\" cannot be {operation} because library '{}' changed since.",
                self.description, self.library
            ));
        }
        if !from.same_content(to) {
            state.require_library_closed(&self.library, &format!("this step is {operation}"))?;
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut Model) -> Result<(), String> {
        self.validate_mutation(state, "undone")?;
        self.validate_crossing(state, false)?;
        state.put_library_place(&self.library, &self.after, &self.before);
        Ok(())
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        self.validate_mutation(state, "redone")?;
        self.validate_crossing(state, true)?;
        state.put_library_place(&self.library, &self.before, &self.after);
        Ok(())
    }
}
