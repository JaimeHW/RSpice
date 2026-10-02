//! Component-property dialog lifecycle, document authority, and host services.

use crate::properties::model_browser::ModelBrowserState;
use crate::state::property_types::{PropertySheet, PropertyValue};
use crate::state::{Component, ComponentType};
use rspice_schematic_editor::component_properties::ComponentPropertyDraft;
use rspice_schematic_editor::requests::EditorRequestSource;
use std::collections::HashMap;

/// Host transaction containing one reusable property draft.
#[derive(Debug, Clone, Default)]
pub struct TabbedPropertyDialogState {
    /// Whether the dialog is currently open
    pub open: bool,

    /// ID of the component being edited
    pub component_id: Option<u64>,

    /// Name of the component being edited (e.g., "R1", "V1")
    pub component_name: Option<String>,

    /// What the engine will do with this source's fields that their labels do
    /// not say. Advisories never block a commit — a refusal is reported through
    /// `draft.commit_error` like any other contract failure — so they are kept apart
    /// from `draft.validation_errors` rather than sharing its map.
    pub source_advisories: Vec<crate::state::SourceContractFinding>,

    /// Retained transaction-level failure that disables publication without
    /// erasing the user's draft (read-only, stale target, or document swap).
    pub session_error: Option<String>,

    /// Complete durable baseline and document authority captured at open.
    pub component_baseline: Option<Component>,
    pub source: Option<EditorRequestSource>,
    pub view_path: String,

    /// Retained field values, invalid text, validation, and prepared edits.
    pub draft: ComponentPropertyDraft,

    /// Model browser state (for semiconductor components)
    pub model_browser: ModelBrowserState,

    /// Project folder that an attached data file is copied into and stored
    /// relative to. `None` for an unsaved project, which has no folder to be
    /// relative to, so a picked file keeps its absolute path.
    pub(super) data_root: Option<std::path::PathBuf>,

    /// The transient the stimulus preview evaluates this source against.
    ///
    /// Every omitted waveform field resolves against a stop time, so the
    /// preview cannot draw a source without one; the host binds the plan's own
    /// transient when it has one, and the card says which it used.
    pub(super) preview_timing: crate::simulation::stimulus_realize::PreviewTiming,
}

/// Durable document authority captured when a component property transaction
/// opens. Isolated render tests can construct a session without document authority.
#[derive(Debug, Clone)]
pub struct ComponentPropertySession {
    component_baseline: Component,
    source: Option<EditorRequestSource>,
    view_path: String,
    data_root: Option<std::path::PathBuf>,
    preview_timing: crate::simulation::stimulus_realize::PreviewTiming,
}

impl ComponentPropertySession {
    pub fn new(
        component_baseline: Component,
        source: EditorRequestSource,
        view_path: String,
    ) -> Self {
        Self {
            source: Some(source),
            view_path,
            component_baseline,
            data_root: None,
            preview_timing: crate::simulation::stimulus_realize::PreviewTiming::default(),
        }
    }

    /// Render a test fixture without authorizing publication to a live document.
    #[cfg(test)]
    pub(crate) fn detached(component_baseline: Component) -> Self {
        Self {
            component_baseline,
            source: None,
            view_path: String::new(),
            data_root: None,
            preview_timing: crate::simulation::stimulus_realize::PreviewTiming::default(),
        }
    }

    /// Bind the project folder that attached data files are copied into and
    /// stored relative to. An unsaved project has none, so a file picked there
    /// is referenced where it lies.
    #[must_use]
    pub fn with_data_root(mut self, data_root: Option<std::path::PathBuf>) -> Self {
        self.data_root = data_root;
        self
    }

    /// Bind the transient the stimulus preview evaluates this source against.
    ///
    /// The plan owns which transient that is, and the preview card names it, so
    /// it is captured when the editor opens rather than read from the plan on
    /// every frame.
    #[must_use]
    pub fn with_preview_timing(
        mut self,
        preview_timing: crate::simulation::stimulus_realize::PreviewTiming,
    ) -> Self {
        self.preview_timing = preview_timing;
        self
    }
}

/// Live display projection and the host-only retained waveform file.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentPropertyContext {
    pub editor: rspice_schematic_editor::component_properties::ComponentEditorContext,
    pub retained_table: Option<RetainedTableFile>,
}

/// A definition's retained table as a file the engine can open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetainedTableFile {
    /// The data-file reference the adopted card carries. The retained copy
    /// stands in for that file and no other: a draft that has been pointed
    /// somewhere else is no longer the copy, and a run of it would not be
    /// offered these bytes either.
    pub reference: String,
    /// Where the copy is.
    pub path: std::path::PathBuf,
}

impl TabbedPropertyDialogState {
    /// Open the dialog for a specific component.
    ///
    /// Populates the dialog with the component's property sheet and current values.
    pub fn open_for_component(
        &mut self,
        component_id: u64,
        component_name: impl Into<String>,
        component_type: ComponentType,
        sheet: &PropertySheet,
        current_values: HashMap<String, PropertyValue>,
        session: ComponentPropertySession,
    ) {
        let ComponentPropertySession {
            component_baseline,
            source,
            view_path,
            data_root,
            preview_timing,
        } = session;
        self.data_root = data_root;
        self.preview_timing = preview_timing;
        self.open = true;
        self.component_id = Some(component_id);
        self.component_name = Some(component_name.into());
        self.draft.reset(component_type, sheet, current_values);
        self.session_error = None;
        self.component_baseline = Some(component_baseline);
        self.source = source;
        self.view_path = view_path;
        self.model_browser = ModelBrowserState::default();
    }

    /// Close the dialog and discard the isolated transaction.
    pub fn close(&mut self) {
        self.open = false;
        self.component_id = None;
        self.component_name = None;
        self.draft.clear();
        self.session_error = None;
        self.component_baseline = None;
        self.source = None;
        self.view_path.clear();
        self.model_browser = ModelBrowserState::default();
    }

    /// Close and discard the isolated draft immediately.
    ///
    /// Cancel discards the draft without a secondary confirmation.
    pub fn attempt_close(&mut self) -> bool {
        self.close();
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::property_types::{PropertyDefinition, PropertyType};

    fn numeric(value: f64) -> PropertyValue {
        PropertyValue::Number { value, unit: None }
    }

    fn constrained_sheet() -> PropertySheet {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("gain")
                .with_type(PropertyType::Number)
                .with_default(numeric(1.0))
                .with_range(0.0, 10.0),
        );
        sheet.add(
            PropertyDefinition::new("offset")
                .with_type(PropertyType::Number)
                .with_default(numeric(0.0))
                .with_range(-1.0, 1.0),
        );
        sheet
    }

    fn pwl_sheet() -> PropertySheet {
        let mut sheet = PropertySheet::new();
        sheet.add(
            PropertyDefinition::new("pwl_data")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::String("0 0 1n 1".to_owned()))
                .required(),
        );
        sheet
    }

    fn edited_dialog(sheet: &PropertySheet) -> TabbedPropertyDialogState {
        let mut values = HashMap::new();
        values.insert("gain".to_owned(), numeric(1.0));
        values.insert("offset".to_owned(), numeric(0.0));
        let mut state = TabbedPropertyDialogState::default();
        state.open_for_component(
            7,
            "X7",
            ComponentType::Resistor,
            sheet,
            values,
            ComponentPropertySession::detached(Component::new(
                7,
                ComponentType::Resistor,
                crate::state::Point::origin(),
            )),
        );
        state.draft.set_value("gain", numeric(2.0));
        state.draft.set_value("offset", numeric(3.0));
        state
    }

    #[test]
    fn cancel_discards_the_transaction_and_resets_child_state() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);
        state.model_browser.open = true;

        state.close();
        assert!(!state.open);
        assert!(!state.model_browser.open);
        assert!(state.component_id.is_none());
        assert!(state.component_baseline.is_none());
        assert!(state.draft.values.is_empty());
        assert!(state.draft.modified.is_empty());
    }

    #[test]
    fn invalid_pwl_cell_draft_marks_parent_dirty_and_is_discarded_atomically() {
        let sheet = pwl_sheet();
        let mut values = HashMap::new();
        values.insert(
            "pwl_data".to_owned(),
            PropertyValue::String("0 0 1n 1".to_owned()),
        );
        let mut state = TabbedPropertyDialogState::default();
        state.open_for_component(
            9,
            "V9",
            ComponentType::VoltageSourcePwl,
            &sheet,
            values,
            ComponentPropertySession::detached(Component::new(
                9,
                ComponentType::VoltageSourcePwl,
                crate::state::Point::origin(),
            )),
        );

        state.draft.pwl_editor.edit_buffers[1].1 = "1e".to_owned();
        assert!(state.draft.pwl_editor.apply_buffer_edits().is_err());
        state.draft.set_value(
            "pwl_data",
            PropertyValue::String(state.draft.pwl_editor.to_string()),
        );
        state.draft.sync_pwl_validation_error();

        assert_eq!(
            state.draft.get_value("pwl_data"),
            Some(&PropertyValue::String("0 0 1n 1e".to_owned()))
        );
        assert!(state.draft.has_modifications());
        state.close();
        assert!(!state.open);
    }
}
