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
            let mut candidate = source.clone();
            target.apply(&mut candidate);
            // A probe follows its current occurrence. Restoring a captured
            // expression first would apply the inverse mapping twice or carry
            // an old root's meaning into a newly selected configuration.
            candidate.probes.clone_from(&source.probes);
            sources.insert(key.clone(), source.clone());
            candidates.insert(key.clone(), candidate);
        }
        self.prepare_schematic_reference_transaction(sources, candidates)
            .map(|transaction| transaction.into_history(forward))
    }

    pub(super) fn schematic_reference_sources(&self) -> BTreeMap<String, &SchematicState> {
        self.workspace.schematic_reference_sources(
            &self.workspace.active_schematic_reference(),
            &self.schematic,
        )
    }

    pub(super) fn prepare_schematic_reference_transaction(
        &self,
        before: BTreeMap<String, SchematicState>,
        after: BTreeMap<String, SchematicState>,
    ) -> Result<SchematicReferenceTransaction, String> {
        for (key, source) in &before {
            validate_reference_document(self, key, source)?;
        }
        let transaction = self.workspace.prepare_schematic_reference_transaction(
            &self.library_manager,
            &self.workspace.active_schematic_reference(),
            &self.schematic,
            before,
            after,
        )?;
        for (key, source) in &transaction.before {
            validate_reference_document(self, key, source)?;
        }
        Ok(transaction)
    }
}

pub(super) fn validate_reference_document(
    state: &AppState,
    key: &str,
    source: &SchematicState,
) -> Result<(), String> {
    let reference = reference_from_key(key)?;
    if !state.project_lifecycle.project_open || document_read_only(state, &reference) {
        return Err(format!(
            "Reference editing requires an open, writable schematic '{key}'."
        ));
    }
    if source.has_pending_operation() {
        return Err(format!(
            "Finish or cancel the schematic gesture in '{key}' before changing references."
        ));
    }
    Ok(())
}
