//! A Layout view's physical content as steps of the one Undo.
//!
//! Creating a Layout view's physical document and every committed edit batch
//! to one is one step holding the document as it was, or its absence, and
//! as it became. Undo puts the document back and Redo forward; each marks
//! the view modified and its tab unsaved, as the edit did, and brings the
//! layout's tab to the front when it is open. A step stands on the document
//! its last move left: an edit to the layout made since refuses it and says
//! so, rather than overwriting that edit.

use super::*;
use crate::state::PhysicalLayoutDocument;

/// One committed change to a Layout view's physical document.
#[derive(Debug, Clone)]
pub struct PhysicalLayoutEditRecord {
    description: String,
    owner: CellViewRef,
    /// `None` when the step created the document.
    before: Option<PhysicalLayoutDocument>,
    after: PhysicalLayoutDocument,
}

impl Model {
    /// Record the change `owner`'s physical document just took, from
    /// `before`, as one step of Undo described by `description`.
    pub(crate) fn record_physical_layout_edit(
        &mut self,
        owner: &CellViewRef,
        before: Option<PhysicalLayoutDocument>,
        description: String,
    ) {
        let Some(after) = self
            .workspace
            .content
            .physical_layout_document(owner)
            .cloned()
        else {
            return;
        };
        self.push_project_record(
            RecordHeader::committed(Vec::new(), Some(owner.clone()), Some(owner.clone())),
            ProjectDesignBody::PhysicalLayoutEdit(Box::new(PhysicalLayoutEditRecord {
                description,
                owner: owner.clone(),
                before,
                after,
            })),
        );
    }
}

impl PhysicalLayoutEditRecord {
    pub(super) fn description(&self) -> &str {
        &self.description
    }

    fn holds(&self, state: &Model, document: Option<&PhysicalLayoutDocument>) -> bool {
        state
            .workspace
            .content
            .physical_layout_document(&self.owner)
            == document
    }

    pub(super) fn after_design_matches(&self, state: &Model) -> bool {
        self.holds(state, Some(&self.after))
    }

    pub(super) fn before_design_matches(&self, state: &Model) -> bool {
        self.holds(state, self.before.as_ref())
    }

    pub(super) fn validate_mutation(&self, state: &Model, operation: &str) -> Result<(), String> {
        if !state.project_open.is_open() {
            return Err(format!(
                "The layout edit cannot be {operation} without an open project."
            ));
        }
        if state.safe_mode.project_read_only() {
            return Err(format!(
                "The layout edit cannot be {operation} while the project is read-only."
            ));
        }
        let library = state
            .library_manager
            .get_library(&self.owner.library)
            .ok_or_else(|| {
                format!(
                    "The layout edit cannot be {operation}: library '{}' no longer exists.",
                    self.owner.library
                )
            })?;
        if library.read_only {
            return Err(format!(
                "The layout edit cannot be {operation}: library '{}' is read-only.",
                self.owner.library
            ));
        }
        library
            .get_cell(&self.owner.cell)
            .and_then(|cell| cell.get_view(&self.owner.view))
            .map(|_| ())
            .ok_or_else(|| {
                format!(
                    "The layout edit cannot be {operation}: '{}' no longer exists.",
                    self.owner.display_path()
                )
            })
    }

    pub(super) fn apply_before(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "The layout edit cannot be undone because the layout changed since.".to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        let before = self.before.clone();
        self.restore(state, before)
    }

    pub(super) fn apply_after(&mut self, state: &mut Model) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "The layout edit cannot be redone because the layout changed since.".to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        let after = Some(self.after.clone());
        self.restore(state, after)
    }

    /// Make the owner's document `document`, or take it away, as the edit
    /// itself did: the view modified, its tab unsaved, the design's
    /// execution inputs moved on.
    fn restore(
        &self,
        state: &mut Model,
        document: Option<PhysicalLayoutDocument>,
    ) -> Result<(), String> {
        let content = &mut state.workspace.content;
        let prepared = content
            .prepare_replaced_physical_layout_documents(&BTreeMap::from([(
                self.owner.key(),
                document,
            )]))
            .map_err(|error| format!("The layout could not be restored: {error}"))?;
        content.commit_prepared_physical_layout_catalog(prepared);
        if let Some(open) = content
            .open_views
            .iter_mut()
            .find(|open| open.reference == self.owner)
        {
            open.dirty = true;
        }
        if let Some(mut library) = state.library_manager.edit_library(&self.owner.library) {
            library.mark_view_modified(&self.owner.cell, &self.owner.view);
        }
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        Ok(())
    }
}
