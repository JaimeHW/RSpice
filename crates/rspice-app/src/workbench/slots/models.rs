//! The models feature's session: the Models & PDKs workspace's page and
//! catalog view, the device-model editor and its recovery, unsigned PDK
//! authoring and measurement correlation.

/// What the models feature keeps between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct ModelsState {
    pub models_page: crate::state::ModelsPage,
    /// Models & PDKs catalog filters and stable presentation selections.
    /// Model, source, symbol, and PDK records remain in their owning domains.
    pub models_view: crate::workbench::state::ModelsWorkbenchViewState,
    /// Selected model name within the currently selected model library.
    pub selected_model: Option<String>,
    /// Runtime-only transaction and presentation state for the project-owned
    /// device-model editor. Committed source and qualification records remain
    /// in the project model-library domain.
    pub model_editor: crate::workbench::documents::model_editor::ModelEditorState,
    /// Exact unsaved model candidate retained for device-local crash/session
    /// recovery. Its immutable base is reconstructed and authenticated before
    /// it is allowed back into the runtime editor.
    pub(crate) model_editor_recovery:
        Option<crate::workbench::documents::model_editor::ModelEditorRecoveryDraft>,
    /// Application-owned recovery for unsigned PDK authoring edits that have
    /// not completed the explicit durable PDK configuration transaction.
    pub(crate) pdk_technology_authoring: crate::workbench::state::PdkTechnologyAuthoringRecovery,
    /// Runtime-only selection and transactional drafts for measurement
    /// correlation. Committed suites and evidence remain in the project-owned
    /// model-library domain.
    pub model_correlation:
        crate::workbench::documents::model_correlation::ModelCorrelationWorkspaceState,
    /// PDK Settings dialog state. Boxed: it carries a whole PDK configuration
    /// working copy and its original.
    pub(crate) pdk_settings_dialog:
        Box<crate::workbench::app_state::session::pdk_settings::PdkSettingsDialogState>,
    /// Standalone model browser state (for Tools menu access)
    pub(crate) model_browser_state: crate::properties::model_browser::ModelBrowserState,
}

impl ModelsState {
    pub(crate) fn capture_authoring_recovery(&mut self) {
        self.model_editor_recovery = self
            .model_editor
            .draft
            .as_ref()
            .filter(|draft| draft.is_dirty())
            .map(crate::workbench::documents::model_editor::ModelEditorRecoveryDraft::capture);
    }

    #[must_use]
    pub(crate) fn model_editor_has_unsaved_changes(&self) -> bool {
        self.model_editor
            .draft
            .as_ref()
            .is_some_and(|draft| draft.is_dirty())
    }

    #[must_use]
    pub(crate) fn has_unsaved_authoring_changes(&self) -> bool {
        self.model_editor_has_unsaved_changes() || self.pdk_technology_authoring.dirty
    }

    pub(crate) fn clear_project_model_editor(&mut self) {
        self.model_editor = crate::workbench::documents::model_editor::ModelEditorState::default();
        self.model_editor_recovery = None;
    }

    /// The open project is being replaced or closed: the model editor and its
    /// recovery copy belong to the project going away.
    pub(crate) fn on_project_changed(&mut self) {
        self.clear_project_model_editor();
    }
}
