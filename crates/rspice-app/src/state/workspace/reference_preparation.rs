//! Prepare component references independently of editor history and permissions.

use super::*;

pub(crate) struct SchematicReferenceTransaction {
    pub(crate) before: BTreeMap<String, SchematicState>,
    pub(crate) after: BTreeMap<String, SchematicState>,
    pub(crate) references: ReferenceChanges,
    pub(crate) prepared_references: PreparedReferences,
}

impl ProjectWorkspace {
    pub(crate) fn schematic_reference_sources<'a>(
        &'a self,
        active_reference: &CellViewRef,
        active_schematic: SchematicEditorRef<'a>,
    ) -> BTreeMap<String, SchematicEditorRef<'a>> {
        let active = active_reference.key();
        let mut sources: BTreeMap<_, _> = self
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active))
            .map(|(key, design)| {
                (
                    key.clone(),
                    SchematicEditorRef {
                        design,
                        session: self.schematic_sessions.get(key),
                    },
                )
            })
            .collect();
        sources.insert(active, active_schematic);
        sources
    }

    /// Attach the prepared project documents to their existing editor sessions.
    pub(crate) fn prepare_schematic_reference_transaction(
        &self,
        libraries: &LibraryManager,
        active_reference: &CellViewRef,
        active_schematic: SchematicEditorRef<'_>,
        before: BTreeMap<String, SchematicState>,
        after: BTreeMap<String, SchematicState>,
    ) -> Result<SchematicReferenceTransaction, String> {
        let mut before_sessions = BTreeMap::new();
        let before = before
            .into_iter()
            .map(|(key, editor)| {
                let (design, session) = editor.into_parts();
                before_sessions.insert(key.clone(), session);
                (key, design)
            })
            .collect();
        let mut after_sessions = BTreeMap::new();
        let after = after
            .into_iter()
            .map(|(key, editor)| {
                let (design, session) = editor.into_parts();
                after_sessions.insert(key.clone(), session);
                (key, design)
            })
            .collect();
        let transaction = self
            .project_hierarchy(libraries, Some((active_reference, active_schematic)))
            .prepare_reference_transaction(before, after)?;
        let projected = self.schematic_reference_sources(active_reference, active_schematic);
        for key in &transaction.probe_documents {
            after_sessions
                .entry(key.clone())
                .or_insert_with(|| projected[key].session.cloned().unwrap_or_default())
                .is_dirty = true;
        }
        let restore = |designs: BTreeMap<String, rspice_design::schematic::owned::Schematic>,
                       mut sessions: BTreeMap<
            String,
            crate::state::schematic::SchematicSession,
        >| {
            designs
                .into_iter()
                .map(|(key, design)| {
                    let session = sessions
                        .remove(&key)
                        .unwrap_or_else(|| projected[&key].session.cloned().unwrap_or_default());
                    (key, SchematicState::from_parts(design, session))
                })
                .collect()
        };
        Ok(SchematicReferenceTransaction {
            before: restore(transaction.before, before_sessions),
            after: restore(transaction.after, after_sessions),
            references: transaction.references,
            prepared_references: transaction.prepared_references,
        })
    }
}
