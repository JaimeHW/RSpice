//! Immutable authority shared by mockup-owned schematic edit workflows.
//!
//! A modal may collect options and then hand control to the canvas. Both
//! phases validate this complete snapshot so a command can never commit into
//! a different document, view, grid policy, selection, or geometry revision.

use rspice_schematic_editor::requests::EditorRequestSource;

use crate::state::{SchematicDocumentPolicy, SchematicSnapshot, Selection};

use crate::workbench::app_state::model::Model;

impl Model {
    /// Whether the active cellview refuses writes, whatever kind of document
    /// it is.
    ///
    /// Safe mode is one of the three owners of that refusal, alongside a
    /// read-only library master and a read-only hierarchy reference. It
    /// belongs here rather than only in the schematic gate because every
    /// document class — symbol, layout, hierarchy reference, review comment —
    /// writes into the same project that safe mode opened read-only.
    pub fn active_view_read_only(&self) -> bool {
        self.safe_mode.project_read_only()
            || self.workspace.content.active_read_only_reference()
            || self
                .library_manager
                .get_library(&self.workspace.content.active_view.library)
                .is_some_and(|library| library.read_only)
    }

    /// Single late-bound authority for every schematic mutation surface.
    ///
    /// Safe mode can be activated after a tool or dialog was armed, so callers
    /// must not rely only on the persisted schematic flag captured earlier.
    pub fn schematic_edit_read_only(&self) -> bool {
        self.schematic.session.read_only || self.active_view_read_only()
    }
}

/// The snapshot a schematic edit workflow captured, and the model it is
/// validated against again before it commits. Every entry point takes the
/// session that holds the model as `impl AsRef<Model>`.
#[derive(Debug, Clone)]
pub struct SchematicEditAuthority {
    pub source: EditorRequestSource,
    pub view_path: String,
    pub grid_size: i32,
    pub document_policy: SchematicDocumentPolicy,
    pub snapshot: SchematicSnapshot,
    pub selection: Selection,
}

impl SchematicEditAuthority {
    pub fn capture(state: &impl AsRef<Model>) -> Self {
        let model = state.as_ref();
        Self {
            source: schematic_editor_request_source(state),
            view_path: model.workspace.content.active_view.display_path(),
            grid_size: model.schematic.document().grid_size,
            document_policy: model.schematic.document().document_policy,
            snapshot: SchematicSnapshot::capture(model.schematic.document()),
            selection: model.schematic.session.editor.selection.clone(),
        }
    }

    /// Geometry and scope are shared; each workflow validates the library masters it uses.
    pub fn matches_source(&self, current: &EditorRequestSource) -> bool {
        self.source.project == current.project
            && self.source.document == current.document
            && self.source.occurrence == current.occurrence
            && self.source.design_epoch == current.design_epoch
            && self.source.document_epoch == current.document_epoch
            && self.source.content_version == current.content_version
            && self.source.topology_version == current.topology_version
            && self.source.sheet == current.sheet
    }

    pub fn validate(&self, state: &impl AsRef<Model>, command: &str) -> Result<(), String> {
        let model = state.as_ref();
        if model.schematic_edit_read_only() {
            return Err("The active schematic is read-only.".to_owned());
        }
        self.validate_presentation(state, command)
    }

    /// Validate an operation which changes only runtime presentation state.
    ///
    /// Selection is deliberately available in read-only cell views, but it
    /// must still be bound to the exact document, hierarchy owner, geometry,
    /// and prior selection reviewed by the user.
    pub fn validate_presentation(
        &self,
        state: &impl AsRef<Model>,
        command: &str,
    ) -> Result<(), String> {
        let model = state.as_ref();
        let current = schematic_editor_request_source(state);
        let reopen = |reason: &str| format!("{reason}. Close and reopen {command}.");
        if self.source.design_epoch != current.design_epoch {
            return Err(reopen("The design document changed"));
        }
        if self.source.document_epoch != current.document_epoch {
            return Err(reopen("The active schematic buffer changed"));
        }
        if self.source.topology_version != current.topology_version {
            return Err(reopen("The schematic topology changed"));
        }
        if self.view_path != model.workspace.content.active_view.display_path() {
            return Err(reopen("The active cell/view changed"));
        }
        if !self.matches_source(&current) {
            return Err(reopen("The schematic source or editing scope changed"));
        }
        if self.grid_size != model.schematic.document().grid_size
            || self.document_policy != model.schematic.document().document_policy
        {
            return Err(reopen("The schematic grid or editing policy changed"));
        }
        if self.selection != model.schematic.session.editor.selection {
            return Err(reopen("The selected-object set changed"));
        }
        if !self.snapshot.is_equal_document(model.schematic.document()) {
            return Err(reopen("The schematic geometry changed"));
        }
        Ok(())
    }
}

/// Source identity shared by editor requests and retained edit workflows.
pub fn schematic_editor_request_source(state: &impl AsRef<Model>) -> EditorRequestSource {
    let model = state.as_ref();
    let document = model.workspace.content.active_schematic_reference();
    let document_key = document.key();
    let sheet = model
        .workspace
        .content
        .design_management
        .sheet_catalog(&document_key)
        .map(|catalog| (catalog.active_sheet_id(), catalog.revision()));
    EditorRequestSource {
        project: model.workspace.content.project.id(),
        document,
        occurrence: model.workspace.content.active_occurrence().cloned(),
        design_epoch: model.design_execution_epoch,
        document_epoch: model.active_schematic_epoch,
        content_version: model.schematic.content_version(),
        topology_version: model.schematic.topology_version(),
        symbol_revision: symbol_context_revision(state),
        sheet,
    }
}

/// [`Model::symbol_context_revision`] for any holder of the model.
pub fn symbol_context_revision(state: &impl AsRef<Model>) -> u64 {
    state.as_ref().symbol_context_revision()
}
