//! Symbol definition dialog state.

use crate::state::CellViewRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SymbolImportBindingChoice {
    #[default]
    ExplicitModelContract,
    UnboundForReview,
}

/// Retained, isolated import draft. Source text is kept until the user closes
/// the transaction so a parse or contract error is always retryable without
/// reopening the platform picker.
#[derive(Debug, Clone, Default)]
pub(crate) struct SymbolImportDialogState {
    pub(crate) open: bool,
    pub(crate) source_name: String,
    pub(crate) source_text: String,
    pub(crate) target_library: String,
    pub(crate) target_name: String,
    pub(crate) binding_choice: SymbolImportBindingChoice,
    pub(crate) binding_source: Option<CellViewRef>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) source_error: Option<String>,
    pub(crate) validation_error: Option<String>,
    #[cfg(target_arch = "wasm32")]
    pub(crate) picker_token: Option<crate::workbench::browser::file_import::TextImportToken>,
}

impl SymbolImportDialogState {
    pub(crate) fn close_and_discard(&mut self) {
        #[cfg(target_arch = "wasm32")]
        if let Some(token) = self.picker_token.take()
            && crate::workbench::browser::file_import::text_import_is_current(token)
        {
            let _ = crate::workbench::browser::file_import::cancel_active_text_import();
        }
        *self = Self::default();
    }

    pub(crate) fn accept_source(&mut self, name: String, text: String) {
        self.source_name = name;
        self.source_text = text;
        self.source_error = None;
        self.validation_error = None;
        self.dirty = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ParameterFormPhase {
    #[default]
    Review,
    Designer,
}

/// The parameter form draft is completed by the typed symbol-domain module;
/// this shell retains navigation, target identity and transaction treatment.
#[derive(Debug, Clone, Default)]
pub(crate) struct SymbolParameterFormDialogState {
    pub(crate) open: bool,
    pub(crate) target: Option<CellViewRef>,
    pub(crate) phase: ParameterFormPhase,
    pub(crate) original_definition: Option<crate::state::ModelBoundSymbolDefinition>,
    pub(crate) draft_form: Option<crate::state::SymbolParameterForm>,
    pub(crate) selected_section: Option<String>,
    pub(crate) selected_field: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub(crate) validation_error: Option<String>,
}

impl SymbolParameterFormDialogState {
    pub(crate) fn close_and_discard(&mut self) {
        *self = Self::default();
    }
}
