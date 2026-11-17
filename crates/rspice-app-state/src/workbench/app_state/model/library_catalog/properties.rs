//! A library's and a cell's properties: what each is for and whether a
//! library takes changes, each edit one project transaction and one step of
//! Undo.

use super::Model;
use crate::state::ProjectLibraryMutation;

impl Model {
    /// Give `library` the description and access the properties dialog
    /// holds. Only a library the project keeps can be made read-only or
    /// writable again: a library read from elsewhere is as its source
    /// allows. An edit that changes nothing records nothing.
    pub fn edit_library_properties(
        &mut self,
        library: &str,
        description: &str,
        read_only: bool,
    ) -> Result<(), String> {
        let found = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        let description_changed = found.description() != description.trim();
        let access_changed = found.read_only != read_only;
        if !description_changed && !access_changed {
            return Ok(());
        }
        if access_changed && found.path.is_some() {
            return Err(format!(
                "Library '{library}' is read from outside the project; its access is its source's."
            ));
        }
        if description_changed && found.read_only && !access_changed {
            return Err(format!(
                "Library '{library}' is read-only; make it writable to change its description."
            ));
        }
        let mutation =
            self.preflight_project_library_mutation(ProjectLibraryMutation::EditProperties {
                library: library.to_owned(),
                cell: None,
            })?;
        let mut edit = self
            .library_manager
            .edit_library(library)
            .ok_or_else(|| format!("Library '{library}' disappeared during the edit"))?;
        edit.set_description(description);
        edit.set_read_only(read_only);
        drop(edit);
        self.publish_project_library_mutation(mutation);
        Ok(())
    }

    /// Give `library`/`cell` the description the properties dialog holds.
    /// An edit that changes nothing records nothing.
    pub fn edit_cell_properties(
        &mut self,
        library: &str,
        cell: &str,
        description: &str,
    ) -> Result<(), String> {
        let found = self
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' not found"))?;
        let record = found
            .get_cell(cell)
            .ok_or_else(|| format!("Cell '{library}/{cell}' not found"))?;
        if record.description == description.trim() {
            return Ok(());
        }
        if found.read_only {
            return Err(format!(
                "Library '{library}' is read-only; its cells cannot be changed."
            ));
        }
        let mutation =
            self.preflight_project_library_mutation(ProjectLibraryMutation::EditProperties {
                library: library.to_owned(),
                cell: Some(cell.to_owned()),
            })?;
        self.library_manager
            .edit_library(library)
            .and_then(|mut edit| edit.set_cell_description(cell, description))
            .ok_or_else(|| format!("Cell '{library}/{cell}' disappeared during the edit"))?;
        self.publish_project_library_mutation(mutation);
        Ok(())
    }
}
