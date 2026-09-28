//! Editor sessions accompany project-owned schematic designs without entering persistence.

use super::ProjectWorkspace;
use crate::state::schematic::{SchematicEditorMut, SchematicEditorRef, SchematicState};

impl ProjectWorkspace {
    pub(crate) fn schematic_editor(&self, key: &str) -> Option<SchematicEditorRef<'_>> {
        self.content
            .schematic_buffers
            .get(key)
            .map(|design| SchematicEditorRef {
                design,
                session: self.schematic_sessions.get(key),
            })
    }

    pub(crate) fn clone_schematic_editor(&self, key: &str) -> Option<SchematicState> {
        self.schematic_editor(key)
            .map(|source| source.clone_editor())
    }

    pub(crate) fn schematic_editor_mut(&mut self, key: &str) -> Option<SchematicEditorMut<'_>> {
        let design = self.content.schematic_buffers.get_mut(key)?;
        let session = self.schematic_sessions.entry(key.to_owned()).or_default();
        Some(SchematicState::borrow_parts(design, session))
    }

    pub(crate) fn for_each_schematic_editor_mut(
        &mut self,
        mut edit: impl FnMut(&str, &mut SchematicState),
    ) {
        for (key, design) in &mut self.content.schematic_buffers {
            let session = self.schematic_sessions.entry(key.clone()).or_default();
            let mut borrowed = SchematicState::borrow_parts(design, session);
            edit(key, &mut borrowed.editor);
        }
    }

    pub(crate) fn insert_schematic_editor(
        &mut self,
        key: String,
        editor: SchematicState,
    ) -> Option<SchematicState> {
        let (design, session) = editor.into_parts();
        let previous_design = self.content.schematic_buffers.insert(key.clone(), design);
        let previous_session = self.schematic_sessions.insert(key, session);
        previous_design
            .map(|design| SchematicState::from_parts(design, previous_session.unwrap_or_default()))
    }

    pub(crate) fn remove_schematic_editor(&mut self, key: &str) -> Option<SchematicState> {
        let design = self.content.schematic_buffers.remove(key);
        let session = self.schematic_sessions.remove(key);
        design.map(|design| SchematicState::from_parts(design, session.unwrap_or_default()))
    }
}
