//! Taking every placement of one deleted library object out of the project,
//! as one undo step however many drawings it reached.

use super::*;

/// Taking every placement of one deleted library object out of the project.
///
/// The reader answered one question — what should happen to the drawings that
/// placed this master — so however many documents the answer reaches, it is
/// one step here. A record per document would make Undo walk back through a
/// decision the reader made once.
#[derive(Debug, Clone)]
pub(super) struct InstanceRemovalRecord {
    pub(super) description: String,
    documents: Vec<InstanceRemovalDocument>,
}

/// One drawing the removal touched, with the design on each side of it.
#[derive(Debug, Clone)]
struct InstanceRemovalDocument {
    reference: CellViewRef,
    before: SchematicSnapshot,
    after: SchematicSnapshot,
}

impl AppState {
    /// Record one answer to "what happens to the placements", however many
    /// drawings it reached. Each document is named so undo puts its objects
    /// back on the sheets they were drawn on, and the reader is returned to
    /// the drawing they were editing when they answered.
    pub(crate) fn record_instance_removal_transaction(
        &mut self,
        description: impl Into<String>,
        documents: Vec<(CellViewRef, SchematicState, SchematicState)>,
    ) {
        let documents = documents
            .into_iter()
            .map(|(reference, before, after)| InstanceRemovalDocument {
                reference,
                before: SchematicSnapshot::capture(before.document()),
                after: SchematicSnapshot::capture(after.document()),
            })
            .filter(|document| !document.before.is_equal(&document.after))
            .collect::<Vec<_>>();
        if documents.is_empty() {
            return;
        }
        let mut compensations = Vec::with_capacity(documents.len());
        for document in &documents {
            compensations.push(DocumentCompensation::with_recorded_sheets(
                self,
                document.reference.clone(),
            ));
        }
        let active = self.workspace.content.active_schematic_reference();
        let header = RecordHeader::committed(compensations, Some(active.clone()), Some(active));
        self.push_project_record(
            header,
            ProjectDesignBody::InstanceRemoval(Box::new(InstanceRemovalRecord {
                description: description.into(),
                documents,
            })),
        );
    }
}

impl InstanceRemovalRecord {
    pub(super) fn after_design_matches(&self, state: &AppState) -> bool {
        self.documents
            .iter()
            .all(|document| document.matches(state, &document.after))
    }

    pub(super) fn before_design_matches(&self, state: &AppState) -> bool {
        self.documents
            .iter()
            .all(|document| document.matches(state, &document.before))
    }

    pub(super) fn validate_mutation(
        &self,
        state: &AppState,
        operation: &str,
    ) -> Result<(), String> {
        if !state.project_lifecycle.is_open() {
            return Err(format!(
                "Instance removal cannot be {operation} without an open project."
            ));
        }
        for document in &self.documents {
            if document_read_only(state, &document.reference) {
                return Err(format!(
                    "Instance removal cannot be {operation} while '{}' is read-only.",
                    document.reference.display_path()
                ));
            }
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Instance removal cannot be undone because those drawings changed.".to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        for document in &self.documents {
            document.restore(state, &document.before)?;
        }
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        Ok(())
    }

    pub(super) fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Instance removal cannot be redone because those drawings changed.".to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        for document in &self.documents {
            document.restore(state, &document.after)?;
        }
        state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
        Ok(())
    }
}

impl InstanceRemovalDocument {
    fn matches(&self, state: &AppState, expected: &SchematicSnapshot) -> bool {
        schematic_for_reference(state, &self.reference)
            .is_some_and(|schematic| expected.is_equal_document(schematic.document()))
    }

    fn restore(&self, state: &mut AppState, snapshot: &SchematicSnapshot) -> Result<(), String> {
        let key = self.reference.key();
        if state.workspace.content.active_schematic_reference() == self.reference {
            state.schematic.apply_snapshot(snapshot);
            state
                .workspace
                .insert_schematic_editor(key, state.schematic.clone());
            return Ok(());
        }
        let Some(mut stored) = state.workspace.schematic_editor_mut(&key) else {
            return Err(format!(
                "Instance removal cannot restore '{}' because it is no longer loaded.",
                self.reference.display_path()
            ));
        };
        stored.editor.apply_snapshot(snapshot);
        Ok(())
    }
}
