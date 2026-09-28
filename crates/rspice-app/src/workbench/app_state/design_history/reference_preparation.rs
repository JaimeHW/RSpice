//! Prepare reference edits across configured occurrences and buffered documents.

use super::*;
pub(super) use crate::state::workspace::reference_from_key;

pub(super) struct PreparedReferenceHistory {
    pub(super) before: BTreeMap<String, SchematicSnapshot>,
    pub(super) after: BTreeMap<String, SchematicSnapshot>,
    pub(super) changes: ReferenceChanges,
    pub(super) references: crate::state::workspace::PreparedReferences,
}

impl SchematicReferenceTransaction {
    fn into_history(self, forward: bool) -> PreparedReferenceHistory {
        let (before, after, changes) = if forward {
            (self.before, self.after, self.references)
        } else {
            (self.after, self.before, self.references.reversed())
        };
        PreparedReferenceHistory {
            before: capture_schematic_map(before),
            after: capture_schematic_map(after),
            changes,
            references: self.prepared_references,
        }
    }
}

impl AppState {
    /// Restore the guarded design edit while resolving its current consumers.
    /// Outputs, configurations and other documents can acquire references after
    /// the original commit; their current roots and occurrences remain authority.
    pub(super) fn prepare_reference_history(
        &self,
        before: &BTreeMap<String, SchematicSnapshot>,
        after: &BTreeMap<String, SchematicSnapshot>,
        forward: bool,
    ) -> Result<PreparedReferenceHistory, String> {
        let (expected, destination) = if forward {
            (before, after)
        } else {
            (after, before)
        };
        if !expected.keys().eq(destination.keys()) || !schematic_map_matches(self, expected) {
            return Err(
                "The reference edit's documents changed before history could be applied."
                    .to_owned(),
            );
        }
        let mut sources = BTreeMap::new();
        let mut candidates = BTreeMap::new();
        for (key, target) in destination {
            let reference = reference_from_key(key)?;
            let source = schematic_for_reference(self, &reference)
                .ok_or_else(|| format!("Reference document '{key}' is unavailable."))?;
            validate_reference_document(self, key, source)?;
            let candidate = source.reference_history_candidate(target);
            sources.insert(key.clone(), source.clone_editor());
            candidates.insert(key.clone(), candidate);
        }
        self.prepare_schematic_reference_transaction(sources, candidates)
            .map(|transaction| transaction.into_history(forward))
    }

    pub(super) fn schematic_reference_sources(
        &self,
    ) -> BTreeMap<String, crate::state::SchematicEditorRef<'_>> {
        self.workspace.schematic_reference_sources(
            &self.workspace.content.active_schematic_reference(),
            self.schematic.editor_ref(),
        )
    }

    pub(super) fn prepare_schematic_reference_transaction(
        &self,
        before: BTreeMap<String, SchematicState>,
        after: BTreeMap<String, SchematicState>,
    ) -> Result<SchematicReferenceTransaction, String> {
        for (key, source) in &before {
            validate_reference_document(self, key, source.editor_ref())?;
        }
        let transaction = self.workspace.prepare_schematic_reference_transaction(
            &self.library_manager,
            &self.workspace.content.active_schematic_reference(),
            self.schematic.editor_ref(),
            before,
            after,
        )?;
        for (key, source) in &transaction.before {
            validate_reference_document(self, key, source.editor_ref())?;
        }
        Ok(transaction)
    }
}

pub(super) fn validate_reference_document(
    state: &AppState,
    key: &str,
    source: crate::state::SchematicEditorRef<'_>,
) -> Result<(), String> {
    let reference = reference_from_key(key)?;
    if !state.project_lifecycle.project_open || document_read_only(state, &reference) {
        return Err(format!(
            "Reference editing requires an open, writable schematic '{key}'."
        ));
    }
    if source.design.pending_operation_id().is_some() {
        return Err(format!(
            "Finish or cancel the schematic gesture in '{key}' before changing references."
        ));
    }
    Ok(())
}
