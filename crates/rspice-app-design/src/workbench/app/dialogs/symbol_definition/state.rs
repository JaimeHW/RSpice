//! Symbol definition dialog state.

use crate::state::CellViewRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolImportBindingChoice {
    #[default]
    ExplicitModelContract,
    /// The model an LTspice symbol names in its own attributes.
    SymbolAttributes,
    UnboundForReview,
}

/// Retained, isolated import draft. Source text is kept until the user closes
/// the transaction so a parse or contract error is always retryable without
/// reopening the platform picker.
#[derive(Debug, Clone, Default)]
pub struct SymbolImportDialogState {
    pub open: bool,
    pub source_name: String,
    pub source_text: String,
    pub target_library: String,
    pub target_name: String,
    pub binding_choice: SymbolImportBindingChoice,
    pub binding_source: Option<CellViewRef>,
    pub dirty: bool,
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

    pub fn accept_source(&mut self, name: String, text: String) {
        // An LTspice symbol names the model it binds; it starts bound to it.
        if is_ltspice_symbol(&name) {
            self.binding_choice = SymbolImportBindingChoice::SymbolAttributes;
        } else if self.binding_choice == SymbolImportBindingChoice::SymbolAttributes {
            self.binding_choice = SymbolImportBindingChoice::ExplicitModelContract;
        }
        self.source_name = name;
        self.source_text = text;
        self.source_error = None;
        self.validation_error = None;
        self.dirty = true;
    }
}

pub(crate) fn is_ltspice_symbol(name: &str) -> bool {
    name.rsplit_once('.')
        .is_some_and(|(_, extension)| extension.eq_ignore_ascii_case("asy"))
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
pub struct SymbolParameterFormDialogState {
    pub open: bool,
    pub target: Option<CellViewRef>,
    pub(crate) phase: ParameterFormPhase,
    pub(crate) original_definition: Option<crate::state::ModelBoundSymbolDefinition>,
    pub draft_form: Option<crate::state::SymbolParameterForm>,
    pub(crate) selected_section: Option<String>,
    pub(crate) selected_field: Option<String>,
    pub(crate) dirty: bool,
    pub(crate) discard_confirm: bool,
    pub validation_error: Option<String>,
}

impl SymbolParameterFormDialogState {
    pub(crate) fn close_and_discard(&mut self) {
        *self = Self::default();
    }
}
