//! What a frame watches of the cellviews of libraries kept in folders of
//! their own. The cellview brought to the front has its lock read, so one
//! another session is editing reads "Being edited by …" and refuses an edit
//! before any is made; and a cellview edited here takes its lock at the
//! first edit, as an edit mode does, not at the next check or Save.

use std::path::Path;

use rspice_design::library::folder::{LibraryFile, LibraryFileLock};

use super::{Model, ReferenceLoad, folders};
use crate::state::CellViewRef;
use crate::time_compat::Instant;

/// How long a lock found on the cellview in front stands before it is read
/// again, so the view becomes editable soon after its holder lets go.
const READ_LOCK_AGAIN: std::time::Duration = std::time::Duration::from_secs(2);

#[derive(Debug, Clone, Default)]
pub(super) struct EditWatch {
    /// The cellview in front when its lock was last read, and when.
    front: Option<(CellViewRef, Instant)>,
    /// The cellviews with unsaved edits as the last frame found them. None
    /// until a frame has looked since the folders were read: what a restore
    /// put back takes no lock until the user edits that cellview again.
    edited: Option<Vec<CellViewRef>>,
}

impl Model {
    /// Watch the cellviews of editable libraries kept in folders of their
    /// own, once a frame. The one brought to the front has its lock read,
    /// and read again every few seconds while someone else holds it; one
    /// edited here since the last frame takes its lock at once. Nothing else
    /// is read from disk.
    pub fn watch_library_edits(&mut self) {
        self.read_front_lock();
        let edited = self.cellviews_edited_here();
        let Some(before) = self.library_files.watch.edited.replace(edited.clone()) else {
            return;
        };
        for cellview in edited.iter().filter(|cellview| !before.contains(cellview)) {
            if let Some((library, file)) = self.editable_cellview_file(cellview) {
                // A refusal records the holder, which the banner states.
                let _ = self.claim_library_file(&library, &file);
            }
        }
    }

    /// Someone else's lock on the cellview in front, when its library is
    /// kept in a folder of its own and editable here: the view is read-only
    /// while it stands, and its banner and every refused edit name the
    /// holder.
    pub fn active_view_locked_elsewhere(&self) -> Option<&LibraryFileLock> {
        let (library, file) = self.editable_cellview_file(&self.workspace.content.active_view)?;
        let state = self.library_files.library(&library)?.files.get(&file)?;
        if state.lock.is_some() {
            return None;
        }
        state.locked_by.as_ref()
    }

    /// The library and file of `cellview`, when its library is kept in a
    /// folder of its own, read from it and editable here.
    pub(super) fn editable_cellview_file(
        &self,
        cellview: &CellViewRef,
    ) -> Option<(String, LibraryFile)> {
        let library = &cellview.library;
        let editable = self
            .library_manager
            .reference(library)
            .is_some_and(|reference| reference.mode.is_editable());
        let loaded = self
            .library_files
            .library(library)
            .is_some_and(|session| matches!(session.load, ReferenceLoad::Loaded));
        if !(editable && loaded) {
            return None;
        }
        let view = self
            .library_manager
            .get_library(library)?
            .get_cell(&cellview.cell)?
            .views
            .get(&cellview.view)?;
        Some((
            library.clone(),
            LibraryFile::cell_view(cellview.cell.clone(), view),
        ))
    }

    /// Each open cellview of a library kept in a folder of its own with
    /// edits here not yet saved.
    pub(super) fn cellviews_edited_here(&self) -> Vec<CellViewRef> {
        let content = &self.workspace.content;
        let dirty = |cellview: &CellViewRef| {
            if *cellview == content.active_view {
                self.schematic.session.is_dirty
            } else {
                self.workspace
                    .session
                    .schematic_sessions
                    .get(&cellview.key())
                    .is_some_and(|session| session.is_dirty)
            }
        };
        let mut edited = content
            .open_views
            .iter()
            .filter(|open| open.dirty || dirty(&open.reference))
            .map(|open| open.reference.clone())
            .collect::<Vec<_>>();
        if dirty(&content.active_view) && !edited.contains(&content.active_view) {
            edited.push(content.active_view.clone());
        }
        edited.retain(|cellview| self.library_manager.is_referenced(&cellview.library));
        edited
    }

    /// Read the lock of the cellview in front when it has just come there,
    /// and again while someone else holds it.
    fn read_front_lock(&mut self) {
        let front = &self.workspace.content.active_view;
        let due = self
            .library_files
            .watch
            .front
            .as_ref()
            .is_none_or(|(seen, read)| {
                seen != front
                    || (self.active_view_locked_elsewhere().is_some()
                        && read.elapsed() >= READ_LOCK_AGAIN)
            });
        if !due {
            return;
        }
        let front = front.clone();
        let file = self.editable_cellview_file(&front);
        self.library_files.watch.front = Some((front, Instant::now()));
        let Some((library, file)) = file else {
            return;
        };
        let Some(folder) = self.library_folder(&library).map(Path::to_path_buf) else {
            return;
        };
        let own = self
            .library_files
            .library(&library)
            .and_then(|session| session.files.get(&file))
            .and_then(|state| state.lock.clone());
        let found = folders::lock_elsewhere(&folder, &file, own.as_ref());
        let known = self
            .library_files
            .library(&library)
            .and_then(|session| session.files.get(&file))
            .and_then(|state| state.locked_by.as_ref());
        if known != found.as_ref()
            && let Some(session) = self.library_files.library_mut(&library)
        {
            session.files.entry(file).or_default().locked_by = found;
        }
    }
}
