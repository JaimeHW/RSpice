//! The models feature's session: the Models & PDKs workspace's page and
//! catalog view, the device-model editor and its recovery, unsigned PDK
//! authoring and measurement correlation.

/// What the models feature keeps between frames.
#[derive(Debug, Clone, Default)]
pub struct ModelsState {
    pub models_page: crate::state::ModelsPage,
    /// Models & PDKs catalog filters and stable presentation selections.
    /// Model, source, symbol, and PDK records remain in their owning domains.
    pub models_view: crate::workbench::models::workspace_state::ModelsWorkbenchViewState,
    /// Selected model name within the currently selected model library.
    pub selected_model: Option<String>,
    /// Runtime-only transaction and presentation state for the project-owned
    /// device-model editor. Committed source and qualification records remain
    /// in the project model-library domain.
    pub model_editor: crate::workbench::documents::model_editor::ModelEditorState,
    /// Exact unsaved model candidate retained for device-local crash/session
    /// recovery. Its immutable base is reconstructed and authenticated before
    /// it is allowed back into the runtime editor.
    pub model_editor_recovery:
        Option<crate::workbench::documents::model_editor::ModelEditorRecoveryDraft>,
    /// Application-owned recovery for unsigned PDK authoring edits that have
    /// not completed the explicit durable PDK configuration transaction.
    pub pdk_technology_authoring:
        crate::workbench::models::workspace_state::PdkTechnologyAuthoringRecovery,
    /// Runtime-only selection and transactional drafts for measurement
    /// correlation. Committed suites and evidence remain in the project-owned
    /// model-library domain.
    pub model_correlation:
        crate::workbench::documents::model_correlation::ModelCorrelationWorkspaceState,
    /// PDK Settings dialog state. Boxed: it carries a whole PDK configuration
    /// working copy and its original.
    pub pdk_settings_dialog:
        Box<crate::workbench::app_state::session::pdk_settings::PdkSettingsDialogState>,
    /// Standalone model browser state (for Tools menu access)
    pub model_browser_state:
        rspice_app_workbench::workbench::app::dialogs::model_browser::ModelBrowserState,
}

impl ModelsState {
    /// What the model commands' selection rules read: this slot's editor and
    /// selection, the model and the platform state.
    pub fn model_editor_reads<'a>(
        &'a self,
        model: &'a rspice_app_state::workbench::app_state::model::Model,
        platform: &'a rspice_app_services::workbench::slots::platform::PlatformState,
    ) -> crate::workbench::documents::model_editor::ModelEditorReads<'a> {
        crate::workbench::documents::model_editor::ModelEditorReads {
            editor: &self.model_editor,
            selected_model: &self.selected_model,
            model,
            platform,
        }
    }

    /// What opening a model in the editor writes: this slot's editor and
    /// selection, the model and the session, beside the platform state.
    pub fn model_editor_parts<'a>(
        &'a mut self,
        model: &'a mut rspice_app_state::workbench::app_state::model::Model,
        workbench: &'a mut crate::workbench::state::WorkbenchSession,
        platform: &'a rspice_app_services::workbench::slots::platform::PlatformState,
    ) -> crate::workbench::documents::model_editor::ModelEditorParts<'a> {
        crate::workbench::documents::model_editor::ModelEditorParts {
            editor: &mut self.model_editor,
            selected_model: &mut self.selected_model,
            model,
            workbench,
            platform,
        }
    }

    pub fn capture_authoring_recovery(&mut self) {
        self.model_editor_recovery = self
            .model_editor
            .draft
            .as_ref()
            .filter(|draft| draft.is_dirty())
            .map(crate::workbench::documents::model_editor::ModelEditorRecoveryDraft::capture);
    }

    /// Reopen the model editor's unsaved draft a restored session carried,
    /// against the model libraries and project revision it restored with. A
    /// draft that no longer applies is dropped, with a warning saying why.
    pub fn reopen_authoring_recovery(
        &mut self,
        model: &mut rspice_app_state::workbench::app_state::model::Model,
    ) {
        use rspice_app_state::diagnostics::{LogSeverity, LogSource};
        if let Some(recovery) = self.model_editor_recovery.clone() {
            match recovery.restore(
                &model.model_library_manager,
                model.workspace.content.project.revision(),
            ) {
                Ok(draft) => self.model_editor.draft = Some(draft),
                Err(error) => {
                    self.model_editor_recovery = None;
                    model.record(
                        LogSeverity::Warning,
                        LogSource::Studio,
                        format!("Unsaved model-editor recovery was not reopened because {error}."),
                    );
                }
            }
        }
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

    pub fn clear_project_model_editor(&mut self) {
        self.model_editor = crate::workbench::documents::model_editor::ModelEditorState::default();
        self.model_editor_recovery = None;
    }

    /// The open project is being replaced or closed: the model editor and its
    /// recovery copy belong to the project going away.
    pub fn on_project_changed(&mut self) {
        self.clear_project_model_editor();
    }
}

/// What another part reads of the models feature: its unsaved authoring and
/// each model's release gate.
impl crate::workbench::framework::feature::ModelsPort for ModelsState {
    fn has_unsaved_authoring_changes(&self) -> bool {
        ModelsState::has_unsaved_authoring_changes(self)
    }

    fn model_editor_has_unsaved_changes(&self) -> bool {
        ModelsState::model_editor_has_unsaved_changes(self)
    }

    fn model_gate_facts(
        &self,
        model: &rspice_app_state::workbench::app_state::model::Model,
    ) -> Vec<crate::workbench::framework::ports::ModelGateFact> {
        crate::workbench::models::qualification::model_gate_facts(model, &self.model_editor)
    }
}
