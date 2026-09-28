//! Editor sessions accompany project-owned schematic designs without entering persistence.

use super::ProjectWorkspace;
use crate::state::schematic::{SchematicEditorMut, SchematicEditorRef, SchematicState};

impl ProjectWorkspace {
    pub(crate) fn schematic_editor(&self, key: &str) -> Option<SchematicEditorRef<'_>> {
        self.session.schematic_editor(&self.content, key)
    }
    pub(crate) fn clone_schematic_editor(&self, key: &str) -> Option<SchematicState> {
        self.session.clone_schematic_editor(&self.content, key)
    }
    pub(crate) fn schematic_editor_mut(&mut self, key: &str) -> Option<SchematicEditorMut<'_>> {
        self.session.schematic_editor_mut(&mut self.content, key)
    }
    pub(crate) fn for_each_schematic_editor_mut(
        &mut self,
        edit: impl FnMut(&str, &mut SchematicState),
    ) {
        self.session
            .for_each_schematic_editor_mut(&mut self.content, edit)
    }
    pub(crate) fn insert_schematic_editor(
        &mut self,
        key: String,
        editor: SchematicState,
    ) -> Option<SchematicState> {
        self.session
            .insert_schematic_editor(&mut self.content, key, editor)
    }
    pub(crate) fn remove_schematic_editor(&mut self, key: &str) -> Option<SchematicState> {
        self.session.remove_schematic_editor(&mut self.content, key)
    }
}

impl super::WorkspaceSession {
    pub(crate) fn schematic_editor<'a>(
        &'a self,
        content: &'a rspice_project::ProjectWorkspace,
        key: &str,
    ) -> Option<SchematicEditorRef<'a>> {
        content
            .schematic_buffers
            .get(key)
            .map(|design| SchematicEditorRef {
                design,
                session: self.schematic_sessions.get(key),
            })
    }

    pub(crate) fn clone_schematic_editor(
        &self,
        content: &rspice_project::ProjectWorkspace,
        key: &str,
    ) -> Option<SchematicState> {
        self.schematic_editor(content, key)
            .map(|source| source.clone_editor())
    }

    pub(crate) fn schematic_editor_mut<'a>(
        &'a mut self,
        content: &'a mut rspice_project::ProjectWorkspace,
        key: &str,
    ) -> Option<SchematicEditorMut<'a>> {
        let design = content.schematic_buffers.get_mut(key)?;
        let session = self.schematic_sessions.entry(key.to_owned()).or_default();
        Some(SchematicState::borrow_parts(design, session))
    }

    pub(crate) fn for_each_schematic_editor_mut(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        mut edit: impl FnMut(&str, &mut SchematicState),
    ) {
        for (key, design) in &mut content.schematic_buffers {
            let session = self.schematic_sessions.entry(key.clone()).or_default();
            let mut borrowed = SchematicState::borrow_parts(design, session);
            edit(key, &mut borrowed.editor);
        }
    }

    pub(crate) fn insert_schematic_editor(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        key: String,
        editor: SchematicState,
    ) -> Option<SchematicState> {
        let (design, session) = editor.into_parts();
        let previous_design = content.schematic_buffers.insert(key.clone(), design);
        let previous_session = self.schematic_sessions.insert(key, session);
        previous_design
            .map(|design| SchematicState::from_parts(design, previous_session.unwrap_or_default()))
    }

    pub(crate) fn remove_schematic_editor(
        &mut self,
        content: &mut rspice_project::ProjectWorkspace,
        key: &str,
    ) -> Option<SchematicState> {
        let design = content.schematic_buffers.remove(key);
        let session = self.schematic_sessions.remove(key);
        design.map(|design| SchematicState::from_parts(design, session.unwrap_or_default()))
    }
}
