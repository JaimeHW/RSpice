//! Schematic component-editor transaction state.
//!
//! Owns typed drafts, validation, nested model/PWL workflows, and the
//! authority snapshot required to publish one isolated component mutation.

use crate::properties::model_browser::ModelBrowserState;
use crate::properties::pwl_editor::PwlEditorState;
use crate::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use crate::state::property_types::{
    PropertyDefinition, PropertySheet, PropertyType, PropertyValue,
};
use crate::state::{Component, ComponentType};
use std::collections::{HashMap, HashSet};

/// Transaction state for the schematic component editor.
///
/// Manages the complete lifecycle of property editing including:
/// - Opening for a specific component
/// - Tracking modifications
/// - Validation and error reporting
/// - Apply/cancel/revert operations
#[derive(Debug, Clone, Default)]
pub struct TabbedPropertyDialogState {
    /// Whether the dialog is currently open
    pub open: bool,

    /// ID of the component being edited
    pub component_id: Option<u64>,

    /// Name of the component being edited (e.g., "R1", "V1")
    pub component_name: Option<String>,

    /// Type of the component being edited
    pub component_type: Option<ComponentType>,

    /// Current property values being edited
    pub values: HashMap<String, PropertyValue>,

    /// Original values (for revert)
    pub original_values: HashMap<String, PropertyValue>,

    /// Set of property names that have been modified
    pub modified: HashSet<String>,

    /// Validation errors by property name
    pub validation_errors: HashMap<String, String>,

    /// What the engine will do with this source's fields that their labels do
    /// not say. Advisories never block a commit — a refusal is reported through
    /// `commit_error` like any other contract failure — so they are kept apart
    /// from `validation_errors` rather than sharing its map.
    pub source_advisories: Vec<crate::state::SourceContractFinding>,

    /// Lossless text drafts for numeric and expression-capable editors. These
    /// are intentionally separate from typed values so intermediate or
    /// invalid input survives repaint and remains isolated until valid.
    numeric_text_drafts: HashMap<String, String>,
    original_numeric_text_drafts: HashMap<String, String>,
    /// Whether `present_numeric_drafts` has already run for this open.
    numeric_drafts_presented: bool,
    numeric_draft_errors: HashMap<String, String>,

    /// Whether to show advanced properties
    pub show_advanced: bool,

    /// Global error message (e.g., "Cannot apply changes")
    pub global_error: Option<String>,

    /// Host-level cross-field or topology validation failure from the most
    /// recent Apply/OK attempt. It remains visible until the draft changes.
    pub commit_error: Option<String>,

    /// Retained transaction-level failure that disables publication without
    /// erasing the user's draft (read-only, stale target, or document swap).
    pub session_error: Option<String>,

    /// Complete durable baseline and document authority captured at open.
    pub component_baseline: Option<Component>,
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,

    /// Validated field delta prepared for the host's next atomic component
    /// mutation. Kept private so unvalidated draft values can never be
    /// mistaken for an authorized commit.
    prepared_commit: HashMap<String, PropertyValue>,

    /// PWL editor state (for PWL sources)
    pub pwl_editor: PwlEditorState,

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
/// opens. Grouping these values keeps the editor boundary explicit and makes
/// it impossible for call sites to accidentally swap generation counters.
#[derive(Debug, Clone)]
pub struct ComponentPropertySession {
    component_baseline: Component,
    design_execution_epoch: u64,
    active_schematic_epoch: u64,
    view_path: String,
    data_root: Option<std::path::PathBuf>,
    preview_timing: crate::simulation::stimulus_realize::PreviewTiming,
}

impl ComponentPropertySession {
    pub fn new(
        component_baseline: Component,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: String,
    ) -> Self {
        Self {
            component_baseline,
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
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

/// Live, non-editable evidence shown beside a component's typed parameters.
///
/// The property draft remains owned by [`TabbedPropertyDialogState`]. This
/// projection is rebuilt from authoritative application state every frame so
/// connectivity, model provenance, and retained operating-point evidence can
/// never become a second source of truth.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentEditorContext {
    pub glyph: String,
    pub subtitle: String,
    pub instance_path: String,
    pub library_cell: String,
    pub family: String,
    pub model: Option<ComponentModelContext>,
    pub operating_point: Option<ComponentOperatingPointContext>,
    pub terminals: Vec<ComponentTerminalContext>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentModelContext {
    pub name: String,
    /// Exact catalog library identity when the binding was resolved from a
    /// model library. Model names alone are not globally unique.
    pub library: Option<String>,
    pub source: String,
    pub section: String,
    pub status: String,
    pub can_open: bool,
    pub can_qualify: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ComponentOperatingPointContext {
    pub run_id: u64,
    pub analysis: String,
    pub current: bool,
    pub rows: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentTerminalContext {
    pub pin: String,
    pub direction: String,
    pub net: Option<String>,
}

/// Result of the component editor interaction.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TabbedDialogResult {
    /// No action taken
    #[default]
    None,
    /// User clicked Apply - commit and retain the editor.
    Applied,
    /// User clicked OK - commit and close after the host accepts the change.
    AppliedAndClose,
    /// Open the selected model's source/catalog record.
    OpenModel,
    /// Open qualification evidence for the selected model.
    OpenQualification,
    /// Close the editor and cross-probe the selected instance in Results.
    CrossProbe,
    /// User clicked Cancel - changes discarded
    Cancelled,
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
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
            data_root,
            preview_timing,
        } = session;
        self.data_root = data_root;
        self.preview_timing = preview_timing;
        self.open = true;
        self.component_id = Some(component_id);
        self.component_name = Some(component_name.into());
        self.component_type = Some(component_type);
        self.values = current_values.clone();
        self.original_values = current_values;
        self.initialize_numeric_text_drafts(sheet);
        self.numeric_drafts_presented = false;
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
        self.commit_error = None;
        self.session_error = None;
        self.component_baseline = Some(component_baseline);
        self.design_execution_epoch = design_execution_epoch;
        self.active_schematic_epoch = active_schematic_epoch;
        self.view_path = view_path;
        self.prepared_commit.clear();
        self.show_advanced = false;
        self.model_browser = ModelBrowserState::default();

        self.pwl_editor = if component_type.is_pwl_source() {
            let source = self
                .values
                .get("pwl_data")
                .map(PropertyValue::display_string)
                .unwrap_or_default();
            PwlEditorState::from_string(
                &source,
                if component_type == ComponentType::CurrentSourcePwl {
                    "A"
                } else {
                    "V"
                },
            )
        } else {
            PwlEditorState::default()
        };
    }

    /// Close the dialog AND clear all state (for Cancel)
    pub fn close(&mut self) {
        self.close_visual();
        self.component_id = None;
        self.component_name = None;
        self.component_type = None;
        self.values.clear();
        self.original_values.clear();
        self.modified.clear();
        self.validation_errors.clear();
        self.numeric_text_drafts.clear();
        self.original_numeric_text_drafts.clear();
        self.numeric_draft_errors.clear();
        self.global_error = None;
        self.commit_error = None;
        self.session_error = None;
        self.component_baseline = None;
        self.design_execution_epoch = 0;
        self.active_schematic_epoch = 0;
        self.view_path.clear();
        self.prepared_commit.clear();
        self.pwl_editor = PwlEditorState::default();
        self.model_browser = ModelBrowserState::default();
    }

    /// Close the dialog visually but KEEP values for caller to apply.
    pub fn close_visual(&mut self) {
        self.open = false;
    }

    /// Clear dialog state after values have been applied.
    pub fn clear_after_apply(&mut self) {
        self.open = false;
        self.component_id = None;
        self.component_name = None;
        self.component_type = None;
        self.values.clear();
        self.original_values.clear();
        self.modified.clear();
        self.validation_errors.clear();
        self.numeric_text_drafts.clear();
        self.original_numeric_text_drafts.clear();
        self.numeric_draft_errors.clear();
        self.global_error = None;
        self.commit_error = None;
        self.session_error = None;
        self.component_baseline = None;
        self.design_execution_epoch = 0;
        self.active_schematic_epoch = 0;
        self.view_path.clear();
        self.prepared_commit.clear();
        self.pwl_editor = PwlEditorState::default();
        self.model_browser = ModelBrowserState::default();
    }

    /// Close and discard the isolated draft immediately.
    ///
    /// The component-editor mockup has a direct Cancel contract rather than
    /// a secondary discard-confirmation state.
    pub fn attempt_close(&mut self) -> bool {
        self.close();
        true
    }

    /// Mark only the fields from a partial commit as the new baseline. Draft
    /// fields that failed validation remain modified and visible for repair.
    pub fn mark_fields_applied(&mut self, names: impl IntoIterator<Item = String>) {
        for name in names {
            if let Some(value) = self.values.get(&name).cloned() {
                self.original_values.insert(name.clone(), value);
            }
            self.modified.remove(&name);
            self.validation_errors.remove(&name);
            self.numeric_draft_errors.remove(&name);
            if let Some(text) = self.numeric_text_drafts.get(&name).cloned() {
                self.original_numeric_text_drafts.insert(name.clone(), text);
            }
            if name == "pwl_data" {
                self.pwl_editor.is_modified = false;
            }
        }
        self.prepared_commit.clear();
        self.refresh_validation_summary();
    }

    /// Revert all changes to original values
    #[cfg(test)]
    pub fn revert(&mut self) {
        self.values = self.original_values.clone();
        self.numeric_text_drafts = self.original_numeric_text_drafts.clone();
        self.numeric_draft_errors.clear();
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
        self.commit_error = None;
        self.prepared_commit.clear();
        if self.component_type.is_some_and(|kind| kind.is_pwl_source()) {
            let source = self
                .values
                .get("pwl_data")
                .map(PropertyValue::display_string)
                .unwrap_or_default();
            self.pwl_editor = PwlEditorState::from_string(
                &source,
                if self.component_type == Some(ComponentType::CurrentSourcePwl) {
                    "A"
                } else {
                    "V"
                },
            );
        }
    }

    /// Set a property value.
    ///
    /// Tracks modification status and validates the value.
    pub fn set_value(&mut self, name: &str, value: PropertyValue) {
        let is_modified = self
            .original_values
            .get(name)
            .map(|orig| orig != &value)
            .unwrap_or(true);

        if is_modified {
            self.modified.insert(name.to_string());
        } else {
            self.modified.remove(name);
        }

        self.values.insert(name.to_string(), value);
        self.validation_errors.remove(name);
        self.commit_error = None;
        self.refresh_validation_summary();
    }

    /// Return retained source text for a numeric or expression editor.
    pub fn numeric_text_draft(&self, name: &str) -> Option<&str> {
        self.numeric_text_drafts.get(name).map(String::as_str)
    }

    /// Publish the latest source text and parse status from one numeric or
    /// expression editor. Invalid source stays dirty without contaminating
    /// typed values.
    pub fn update_numeric_text_draft(
        &mut self,
        name: &str,
        text: String,
        parse_error: Option<String>,
    ) {
        let source_changed = self
            .numeric_text_drafts
            .get(name)
            .is_none_or(|current| current != &text);
        self.numeric_text_drafts.insert(name.to_owned(), text);

        if let Some(error) = parse_error {
            self.numeric_draft_errors
                .insert(name.to_owned(), error.clone());
            self.validation_errors.insert(name.to_owned(), error);
            self.modified.insert(name.to_owned());
        } else {
            self.numeric_draft_errors.remove(name);
            self.validation_errors.remove(name);
            // A parameter the instance never authored is absent from both
            // maps. That is the schema default, not an edit: treating the
            // missing pair as a difference marked every unauthored field
            // modified the instant the editor opened.
            let is_modified = match (self.values.get(name), self.original_values.get(name)) {
                (None, None) => false,
                (value, original) => value != original,
            };
            if is_modified {
                self.modified.insert(name.to_owned());
            } else {
                self.modified.remove(name);
            }
        }

        if source_changed {
            self.commit_error = None;
        }
        self.refresh_validation_summary();
    }

    /// Get the current value of a property
    /// The transient the stimulus preview evaluates this source against.
    #[must_use]
    pub(super) fn preview_timing(&self) -> crate::simulation::stimulus_realize::PreviewTiming {
        self.preview_timing
    }

    pub fn get_value(&self, name: &str) -> Option<&PropertyValue> {
        self.values.get(name)
    }

    /// Check if a property has been modified
    pub fn is_modified(&self, name: &str) -> bool {
        self.modified.contains(name)
    }

    /// Check if any properties have been modified
    pub fn has_modifications(&self) -> bool {
        !self.modified.is_empty()
    }

    /// Mirror the PWL editor's live validity into the parent transaction.
    ///
    /// The point editor retains invalid raw drafts independently of the typed
    /// property map, so the dialog must publish that status after every PWL
    /// interaction rather than waiting for an Apply attempt.
    pub(super) fn sync_pwl_validation_error(&mut self) {
        if !self.component_type.is_some_and(|kind| kind.is_pwl_source()) {
            return;
        }
        if self.pwl_editor.is_valid() {
            self.validation_errors.remove("pwl_data");
        } else {
            self.validation_errors.insert(
                "pwl_data".to_owned(),
                self.pwl_editor
                    .validation_error
                    .clone()
                    .unwrap_or_else(|| "PWL waveform data is invalid".to_owned()),
            );
        }
        self.refresh_validation_summary();
    }

    /// Whether the document's commit policy has at least one publishable
    /// action. Atomic mode blocks on every known invalid draft; partial mode
    /// remains available only when a distinct valid modified field exists.
    pub fn can_apply(&self, policy: crate::state::PropertyCommitPolicy) -> bool {
        if self.modified.is_empty() {
            return false;
        }
        match policy {
            crate::state::PropertyCommitPolicy::Atomic => self.validation_errors.is_empty(),
            crate::state::PropertyCommitPolicy::ApplyValidFields => self
                .modified
                .iter()
                .any(|name| !self.validation_errors.contains_key(name)),
        }
    }

    /// Validate all properties against the sheet definitions.
    ///
    /// Returns true if all validations pass.
    pub fn validate_all(&mut self, sheet: &PropertySheet) -> bool {
        self.validation_errors.clear();
        self.global_error = None;

        for def in sheet.iter() {
            let value = self.values.get(&def.name).unwrap_or(&def.default_value);
            if let Err(error) = def
                .validate(value)
                .and_then(|()| validate_property_expression(def, value))
            {
                self.validation_errors.insert(def.name.clone(), error);
            }
        }
        self.validation_errors.extend(
            self.numeric_draft_errors
                .iter()
                .map(|(name, error)| (name.clone(), error.clone())),
        );
        if self.component_type.is_some_and(|kind| kind.is_pwl_source())
            && !self.pwl_editor.is_valid()
        {
            self.validation_errors.insert(
                "pwl_data".to_owned(),
                self.pwl_editor
                    .validation_error
                    .clone()
                    .unwrap_or_else(|| "PWL waveform data is invalid".to_owned()),
            );
        }

        self.refresh_validation_summary();
        self.validation_errors.is_empty()
    }

    /// Validate the draft and prepare exactly the delta authorized by the
    /// document's commit policy.
    ///
    /// Atomic mode prepares nothing unless every field is valid. Partial mode
    /// prepares only valid modified fields; invalid draft values remain
    /// isolated in this dialog and are never passed to the component bridge.
    pub fn prepare_commit(
        &mut self,
        sheet: &PropertySheet,
        policy: crate::state::PropertyCommitPolicy,
    ) -> bool {
        self.commit_error = None;
        self.prepared_commit.clear();
        let all_valid = self.validate_all(sheet);
        if !all_valid && policy == crate::state::PropertyCommitPolicy::Atomic {
            return false;
        }

        for name in &self.modified {
            if self.validation_errors.contains_key(name) {
                continue;
            }
            if let Some(value) = self.values.get(name) {
                self.prepared_commit.insert(name.clone(), value.clone());
            }
        }

        if self.prepared_commit.is_empty() {
            if all_valid {
                self.global_error = Some("No modified properties to apply".to_owned());
            }
            return false;
        }

        if !all_valid {
            self.global_error = Some(format!(
                "{} invalid field(s) retained; {} valid field(s) will be applied",
                self.validation_errors.len(),
                self.prepared_commit.len()
            ));
        }
        true
    }

    /// Transfer the already validated field delta to the component host.
    pub fn take_prepared_commit(&mut self) -> HashMap<String, PropertyValue> {
        std::mem::take(&mut self.prepared_commit)
    }

    /// Re-present the untouched drafts in engineering notation, once, as soon
    /// as a render pass supplies the presentation policy.
    ///
    /// `open_for_component` runs before any policy is in hand, so it seeds the
    /// exact decimal — correct but unreadable for a rise time
    /// (`0.000000001 s`). This cannot run every frame: it would snap a user
    /// who is deliberately typing that exact form back to `1ns` mid-edit.
    pub(super) fn present_numeric_drafts(
        &mut self,
        sheet: &PropertySheet,
        quantity_policy: QuantityPresentationPolicy,
        number_locale: UiNumberLocale,
    ) {
        if self.numeric_drafts_presented {
            return;
        }
        self.numeric_drafts_presented = true;
        let presented = sheet
            .iter()
            .filter(|def| {
                matches!(
                    def.prop_type,
                    PropertyType::Number | PropertyType::Expression
                )
            })
            .filter(|def| {
                // Only re-present a draft nobody has touched. A caller can
                // write a draft between `open_for_component` and the first
                // paint — a retained invalid entry, for instance — and
                // rewriting that would silently discard their edit.
                self.numeric_text_drafts.get(&def.name)
                    == self.original_numeric_text_drafts.get(&def.name)
            })
            .map(|def| {
                let value = self.values.get(&def.name).unwrap_or(&def.default_value);
                (
                    def.name.clone(),
                    super::editors::editor_source_text(def, value, quantity_policy, number_locale),
                )
            })
            .collect::<Vec<_>>();
        for (name, text) in presented {
            self.numeric_text_drafts.insert(name.clone(), text.clone());
            self.original_numeric_text_drafts.insert(name, text);
        }
    }

    fn initialize_numeric_text_drafts(&mut self, sheet: &PropertySheet) {
        self.numeric_text_drafts.clear();
        self.original_numeric_text_drafts.clear();
        self.numeric_draft_errors.clear();
        for def in sheet.iter().filter(|def| {
            matches!(
                def.prop_type,
                PropertyType::Number | PropertyType::Expression
            )
        }) {
            let value = self.values.get(&def.name).unwrap_or(&def.default_value);
            let text = numeric_source_text(def, value);
            self.numeric_text_drafts
                .insert(def.name.clone(), text.clone());
            self.original_numeric_text_drafts
                .insert(def.name.clone(), text);
        }
    }

    fn refresh_validation_summary(&mut self) {
        self.global_error = (!self.validation_errors.is_empty())
            .then(|| format!("{} validation error(s)", self.validation_errors.len()));
    }
}

/// Build the retained editor source from the schema value without applying a
/// presentation-only precision limit. Quantity-aware fields receive an
/// explicit base unit so merely opening their tab cannot turn a valid stored
/// value into an invalid draft under the strict input policy.
/// Whether the editor's own text carries this property's unit.
///
/// The unit-safe parser refuses a bare number for these kinds, so the unit is
/// part of the value's syntax rather than metadata about it. The caption reads
/// the same predicate and suppresses its unit chip for them, so the unit is
/// presented exactly once.
pub(super) fn unit_is_part_of_value_text(definition: &PropertyDefinition) -> bool {
    definition.prop_type == PropertyType::Number
        && matches!(
            definition.unit.as_deref(),
            Some("s" | "Hz" | "°" | "deg" | "rad" | "K" | "°C" | "°F")
        )
}

/// Append the unit the value's own syntax must carry, if any.
///
/// One owner for the suffix so the exact form and the engineering form the
/// editor offers can never disagree about it.
pub(super) fn unit_suffixed(definition: &PropertyDefinition, magnitude: &str) -> String {
    match definition.unit.as_deref() {
        Some("s") => format!("{magnitude} s"),
        Some("Hz") => format!("{magnitude} Hz"),
        Some("°" | "deg") => format!("{magnitude} deg"),
        Some("rad") => format!("{magnitude} rad"),
        Some("K") => format!("{magnitude} K"),
        Some("°C") => format!("{magnitude} °C"),
        Some("°F") => format!("{magnitude} °F"),
        _ => magnitude.to_owned(),
    }
}

pub(super) fn numeric_source_text(
    definition: &PropertyDefinition,
    value: &PropertyValue,
) -> String {
    match value {
        PropertyValue::Number { value, .. } => unit_suffixed(definition, &value.to_string()),
        PropertyValue::Expression(expression) => expression.clone(),
        _ => value.display_string(),
    }
}

fn validate_property_expression(
    definition: &PropertyDefinition,
    value: &PropertyValue,
) -> Result<(), String> {
    let PropertyValue::Expression(source) = value else {
        return Ok(());
    };
    if !matches!(
        definition.prop_type,
        PropertyType::Number | PropertyType::Expression
    ) {
        return Ok(());
    }
    let trimmed = source.trim();
    if trimmed.is_empty() {
        return if definition.prop_type == PropertyType::Expression && !definition.required {
            Ok(())
        } else {
            Err(format!("{} expression is empty", definition.display_name))
        };
    }
    super::editors::parse_expression_source(
        definition,
        source,
        crate::quantity::QuantityPresentationPolicy::default(),
        crate::quantity::UiNumberLocale::default(),
    )
    .map(drop)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::PropertyCommitPolicy;
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
            ComponentPropertySession::new(
                Component::new(7, ComponentType::Resistor, crate::state::Point::origin()),
                4,
                9,
                "user/top/schematic".to_owned(),
            ),
        );
        state.set_value("gain", numeric(2.0));
        state.set_value("offset", numeric(3.0));
        state
    }

    #[test]
    fn atomic_policy_never_prepares_a_partial_invalid_draft() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);

        assert!(!state.prepare_commit(&sheet, PropertyCommitPolicy::Atomic));
        assert!(state.take_prepared_commit().is_empty());
        assert!(state.validation_errors.contains_key("offset"));
        assert!(state.is_modified("gain"));
    }

    #[test]
    fn partial_policy_prepares_only_valid_modified_fields() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);

        assert!(state.prepare_commit(&sheet, PropertyCommitPolicy::ApplyValidFields));
        let prepared = state.take_prepared_commit();
        assert_eq!(prepared.get("gain"), Some(&numeric(2.0)));
        assert!(!prepared.contains_key("offset"));

        state.mark_fields_applied(prepared.into_keys());
        assert!(!state.is_modified("gain"));
        assert!(state.is_modified("offset"));
        assert!(state.validation_errors.contains_key("offset"));
    }

    #[test]
    fn cancel_discards_the_transaction_and_resets_child_state() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);
        state.model_browser.open = true;

        state.close();
        assert!(!state.open);
        assert!(!state.model_browser.open);
        assert!(state.component_baseline.is_none());
    }

    #[test]
    fn new_pwl_component_opens_on_registry_default_as_a_clean_valid_transaction() {
        let registry = crate::state::PropertyRegistry::new();
        let component = Component::new(
            9,
            ComponentType::VoltageSourcePwl,
            crate::state::Point::origin(),
        );
        let values = crate::properties::property_bridge::collect_properties_from_component(
            &component, &registry,
        );
        let sheet = registry.get(ComponentType::VoltageSourcePwl).unwrap();
        let mut state = TabbedPropertyDialogState::default();

        state.open_for_component(
            component.id,
            "V9",
            component.kind,
            sheet,
            values,
            ComponentPropertySession::new(component, 4, 9, "user/top/schematic".to_owned()),
        );
        state.sync_pwl_validation_error();

        assert_eq!(
            state.get_value("pwl_data"),
            Some(&PropertyValue::String("0 0 1u 1 2u 0".to_owned()))
        );
        assert!(state.pwl_editor.is_valid());
        assert!(state.validation_errors.is_empty());
        assert!(!state.has_modifications());
    }

    #[test]
    fn explicitly_authored_empty_pwl_transaction_is_invalid() {
        let sheet = pwl_sheet();
        let mut values = HashMap::new();
        values.insert("pwl_data".to_owned(), PropertyValue::String(String::new()));
        let mut state = TabbedPropertyDialogState::default();

        state.open_for_component(
            9,
            "V9",
            ComponentType::VoltageSourcePwl,
            &sheet,
            values,
            ComponentPropertySession::new(
                Component::new(
                    9,
                    ComponentType::VoltageSourcePwl,
                    crate::state::Point::origin(),
                ),
                4,
                9,
                "user/top/schematic".to_owned(),
            ),
        );
        state.sync_pwl_validation_error();

        assert_eq!(state.pwl_editor.raw_source_draft(), Some(""));
        assert!(state.validation_errors.contains_key("pwl_data"));
        assert!(!state.pwl_editor.is_valid());
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
            ComponentPropertySession::new(
                Component::new(
                    9,
                    ComponentType::VoltageSourcePwl,
                    crate::state::Point::origin(),
                ),
                4,
                9,
                "user/top/schematic".to_owned(),
            ),
        );

        state.pwl_editor.edit_buffers[1].1 = "1e".to_owned();
        assert!(state.pwl_editor.apply_buffer_edits().is_err());
        state.set_value(
            "pwl_data",
            PropertyValue::String(state.pwl_editor.to_string()),
        );
        state.sync_pwl_validation_error();

        assert_eq!(
            state.get_value("pwl_data"),
            Some(&PropertyValue::String("0 0 1n 1e".to_owned()))
        );
        assert!(state.has_modifications());
        state.close();
        assert!(!state.open);
    }

    #[test]
    fn live_pwl_validation_blocks_apply_and_repair_clears_parent_error() {
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
            ComponentPropertySession::new(
                Component::new(
                    9,
                    ComponentType::VoltageSourcePwl,
                    crate::state::Point::origin(),
                ),
                4,
                9,
                "user/top/schematic".to_owned(),
            ),
        );

        state.pwl_editor.edit_buffers[1].1 = "1e".to_owned();
        assert!(state.pwl_editor.apply_buffer_edits().is_err());
        state.set_value(
            "pwl_data",
            PropertyValue::String(state.pwl_editor.to_string()),
        );
        state.sync_pwl_validation_error();
        assert!(state.validation_errors.contains_key("pwl_data"));
        assert!(!state.can_apply(PropertyCommitPolicy::Atomic));

        state.pwl_editor.edit_buffers[1].1 = "2".to_owned();
        state.pwl_editor.apply_buffer_edits().unwrap();
        state.set_value(
            "pwl_data",
            PropertyValue::String(state.pwl_editor.to_string()),
        );
        state.sync_pwl_validation_error();
        assert!(!state.validation_errors.contains_key("pwl_data"));
        assert!(state.can_apply(PropertyCommitPolicy::Atomic));
    }

    #[test]
    fn invalid_numeric_source_is_retained_until_repaired() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);
        state.revert();

        state.update_numeric_text_draft(
            "gain",
            "1e".to_owned(),
            Some("invalid number: 1e".to_owned()),
        );

        assert_eq!(state.numeric_text_draft("gain"), Some("1e"));
        assert_eq!(state.get_value("gain"), Some(&numeric(1.0)));
        assert!(state.is_modified("gain"));
        assert!(!state.can_apply(PropertyCommitPolicy::Atomic));
        assert!(!state.can_apply(PropertyCommitPolicy::ApplyValidFields));
        assert!(!state.prepare_commit(&sheet, PropertyCommitPolicy::Atomic));
        assert_eq!(state.numeric_text_draft("gain"), Some("1e"));
        assert!(state.validation_errors.contains_key("gain"));

        state.update_numeric_text_draft("gain", "2".to_owned(), None);
        state.set_value("gain", numeric(2.0));
        assert!(state.prepare_commit(&sheet, PropertyCommitPolicy::Atomic));
        assert_eq!(
            state.take_prepared_commit().get("gain"),
            Some(&numeric(2.0))
        );
    }

    #[test]
    fn partial_commit_preserves_invalid_numeric_source_and_revert_restores_baseline() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);
        state.revert();
        state.update_numeric_text_draft("gain", "2".to_owned(), None);
        state.set_value("gain", numeric(2.0));
        state.update_numeric_text_draft(
            "offset",
            "-".to_owned(),
            Some("invalid number: -".to_owned()),
        );

        assert!(!state.can_apply(PropertyCommitPolicy::Atomic));
        assert!(state.can_apply(PropertyCommitPolicy::ApplyValidFields));
        assert!(state.prepare_commit(&sheet, PropertyCommitPolicy::ApplyValidFields));
        let prepared = state.take_prepared_commit();
        assert_eq!(prepared.get("gain"), Some(&numeric(2.0)));
        assert!(!prepared.contains_key("offset"));
        state.mark_fields_applied(prepared.into_keys());
        assert_eq!(state.numeric_text_draft("offset"), Some("-"));
        assert!(state.is_modified("offset"));

        state.revert();
        assert_eq!(state.numeric_text_draft("gain"), Some("2"));
        assert_eq!(state.numeric_text_draft("offset"), Some("0"));
        assert!(!state.has_modifications());
    }

    #[test]
    fn expression_drafts_are_syntax_checked_before_publication() {
        let definition = PropertyDefinition::new("value")
            .with_display_name("Value")
            .with_type(PropertyType::Expression)
            .with_default(PropertyValue::expression("gain"))
            .required();
        assert!(
            validate_property_expression(
                &definition,
                &PropertyValue::Expression("gain +".to_owned())
            )
            .is_err()
        );
        assert!(
            validate_property_expression(
                &definition,
                &PropertyValue::Expression("gain * 2".to_owned())
            )
            .is_ok()
        );

        let optional = PropertyDefinition::new("leakage")
            .with_type(PropertyType::Expression)
            .with_default(PropertyValue::expression(""));
        assert!(validate_property_expression(&optional, &PropertyValue::expression("")).is_ok());
    }

    #[test]
    fn real_registry_phase_expression_cannot_bypass_units_or_range() {
        let registry = crate::state::PropertyRegistry::new();
        let phase = registry
            .get(ComponentType::VoltageSource)
            .and_then(|sheet| sheet.get("acphase"))
            .expect("voltage-source AC phase definition");

        assert!(validate_property_expression(phase, &PropertyValue::expression("90")).is_err());
        assert!(
            validate_property_expression(phase, &PropertyValue::expression("400 deg")).is_err()
        );
        assert!(validate_property_expression(phase, &PropertyValue::expression("90 deg")).is_ok());
        assert!(
            validate_property_expression(phase, &PropertyValue::expression("phase_parameter"))
                .is_ok()
        );
    }

    #[test]
    fn expression_capable_fields_initialize_a_lossless_retained_draft() {
        let registry = crate::state::PropertyRegistry::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();
        let stored = 89.123_456_789_012_3;
        let values = HashMap::from([("acphase".to_owned(), PropertyValue::number(stored))]);
        let component = Component::new(
            12,
            ComponentType::VoltageSource,
            crate::state::Point::origin(),
        );
        let mut state = TabbedPropertyDialogState::default();

        state.open_for_component(
            component.id,
            "V12",
            component.kind,
            sheet,
            values,
            ComponentPropertySession::new(component, 4, 9, "user/top/schematic".to_owned()),
        );

        let expected = format!("{} deg", stored);
        assert_eq!(state.numeric_text_draft("acphase"), Some(expected.as_str()));
    }

    #[test]
    fn successful_ok_cleanup_closes_and_clears_the_editor_transaction() {
        let sheet = constrained_sheet();
        let mut state = edited_dialog(&sheet);

        state.clear_after_apply();

        assert!(!state.open);
        assert!(state.component_id.is_none());
        assert!(state.component_baseline.is_none());
        assert!(state.values.is_empty());
        assert!(state.modified.is_empty());
    }
}
