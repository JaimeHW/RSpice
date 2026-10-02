//! Component-property drafts and parameter forms over canonical schemas and values.
//!
//! Document authority, component publication, and external services belong to
//! the host. This draft prepares a validated field delta without mutating a design.

mod form;
pub use form::{PropertyBrowseRequest, render_component_parameters, section_band};

use rspice_app_types::property::{PropertyDefinition, PropertySheet, PropertyType, PropertyValue};
use rspice_app_types::quantity::{QuantityPresentationPolicy, UiNumberLocale};
use rspice_design::schematic::component_type::ComponentType;
use rspice_design::schematic::document_policy::PropertyCommitPolicy;
use std::collections::{HashMap, HashSet};

use crate::property_values::{editor_source_text, numeric_source_text};
use crate::pwl_editor::PwlEditorState;

/// Editable values and lossless text for one component property sheet.
#[derive(Debug, Clone, Default)]
pub struct ComponentPropertyDraft {
    /// Type of the component being edited
    pub component_type: Option<ComponentType>,

    /// Current property values being edited
    pub values: HashMap<String, PropertyValue>,

    /// Last accepted values for modification tracking and partial Apply
    pub original_values: HashMap<String, PropertyValue>,

    /// Set of property names that have been modified
    pub modified: HashSet<String>,

    /// Validation errors by property name
    pub validation_errors: HashMap<String, String>,

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

    /// Validated field delta prepared for the host's next atomic component
    /// mutation. Kept private so unvalidated draft values can never be
    /// mistaken for an authorized commit.
    prepared_commit: HashMap<String, PropertyValue>,

    /// PWL editor state (for PWL sources)
    pub pwl_editor: PwlEditorState,
}

impl ComponentPropertyDraft {
    /// Start a clean draft from the host's current component values.
    pub fn reset(
        &mut self,
        component_type: ComponentType,
        sheet: &PropertySheet,
        current_values: HashMap<String, PropertyValue>,
    ) {
        self.component_type = Some(component_type);
        self.values = current_values.clone();
        self.original_values = current_values;
        self.initialize_numeric_text_drafts(sheet);
        self.numeric_drafts_presented = false;
        self.modified.clear();
        self.validation_errors.clear();
        self.global_error = None;
        self.commit_error = None;
        self.prepared_commit.clear();
        self.show_advanced = false;

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

    /// Discard values, invalid text, and any prepared delta when the host closes.
    pub fn clear(&mut self) {
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
        self.prepared_commit.clear();
        self.pwl_editor = PwlEditorState::default();
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

    /// Rebase the accepted fields on what the host actually published, including
    /// normalized names and rewritten references. Rejected drafts stay isolated.
    pub fn rebase_applied_values(
        &mut self,
        values: HashMap<String, PropertyValue>,
        sheet: &PropertySheet,
        quantity_policy: QuantityPresentationPolicy,
        number_locale: UiNumberLocale,
    ) {
        let prior = std::mem::replace(&mut self.original_values, values);
        self.values.retain(|name, _| self.modified.contains(name));
        for (name, value) in &self.original_values {
            if !self.modified.contains(name) {
                self.values.insert(name.clone(), value.clone());
            }
        }
        for def in sheet.iter().filter(|def| {
            matches!(
                def.prop_type,
                PropertyType::Number | PropertyType::Expression
            ) && prior.get(&def.name) != self.original_values.get(&def.name)
        }) {
            let value = self
                .original_values
                .get(&def.name)
                .unwrap_or(&def.default_value);
            let text = editor_source_text(def, value, quantity_policy, number_locale);
            self.original_numeric_text_drafts
                .insert(def.name.clone(), text.clone());
            if !self.modified.contains(&def.name) {
                self.numeric_text_drafts.insert(def.name.clone(), text);
            }
        }
    }

    /// Set a property value.
    ///
    /// Tracks modifications and clears the field's prior validation error.
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

    /// Get the current typed value of a property.
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
    pub fn sync_pwl_validation_error(&mut self) {
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
    pub fn can_apply(&self, policy: PropertyCommitPolicy) -> bool {
        if self.modified.is_empty() {
            return false;
        }
        match policy {
            PropertyCommitPolicy::Atomic => self.validation_errors.is_empty(),
            PropertyCommitPolicy::ApplyValidFields => self
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
    pub fn prepare_commit(&mut self, sheet: &PropertySheet, policy: PropertyCommitPolicy) -> bool {
        self.commit_error = None;
        self.prepared_commit.clear();
        let all_valid = self.validate_all(sheet);
        if !all_valid && policy == PropertyCommitPolicy::Atomic {
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
    /// `reset` runs before any policy is in hand, so it seeds the
    /// exact decimal — correct but unreadable for a rise time
    /// (`0.000000001 s`). This cannot run every frame: it would snap a user
    /// who is deliberately typing that exact form back to `1ns` mid-edit.
    pub fn present_numeric_drafts(
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
                // write a draft between `reset` and the first
                // paint — a retained invalid entry, for instance — and
                // rewriting that would silently discard their edit.
                self.numeric_text_drafts.get(&def.name)
                    == self.original_numeric_text_drafts.get(&def.name)
            })
            .map(|def| {
                let value = self.values.get(&def.name).unwrap_or(&def.default_value);
                (
                    def.name.clone(),
                    editor_source_text(def, value, quantity_policy, number_locale),
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
    rspice_design::properties::value::parse_expression_source(
        definition,
        source,
        QuantityPresentationPolicy::default(),
        UiNumberLocale::default(),
    )
    .map(drop)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::properties::{PropertyCatalog, component::collect_properties_with_sheet};
    use rspice_design::schematic::component::Component;
    use rspice_design_model::Point;

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

    fn clean_draft(sheet: &PropertySheet) -> ComponentPropertyDraft {
        let mut values = HashMap::new();
        values.insert("gain".to_owned(), numeric(1.0));
        values.insert("offset".to_owned(), numeric(0.0));
        let mut state = ComponentPropertyDraft::default();
        state.reset(ComponentType::Resistor, sheet, values);
        state
    }

    fn edited_draft(sheet: &PropertySheet) -> ComponentPropertyDraft {
        let mut state = clean_draft(sheet);
        state.set_value("gain", numeric(2.0));
        state.set_value("offset", numeric(3.0));
        state
    }

    #[test]
    fn atomic_policy_never_prepares_a_partial_invalid_draft() {
        let sheet = constrained_sheet();
        let mut state = edited_draft(&sheet);

        assert!(!state.prepare_commit(&sheet, PropertyCommitPolicy::Atomic));
        assert!(state.take_prepared_commit().is_empty());
        assert!(state.validation_errors.contains_key("offset"));
        assert!(state.is_modified("gain"));
    }

    #[test]
    fn partial_policy_prepares_only_valid_modified_fields() {
        let sheet = constrained_sheet();
        let mut state = edited_draft(&sheet);

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
    fn new_pwl_component_opens_on_registry_default_as_a_clean_valid_transaction() {
        let registry = PropertyCatalog::new();
        let component = Component::new(9, ComponentType::VoltageSourcePwl, Point::origin());
        let sheet = registry.get(ComponentType::VoltageSourcePwl).unwrap();
        let values = collect_properties_with_sheet(&component, Some(sheet));
        let mut state = ComponentPropertyDraft::default();

        state.reset(component.kind, sheet, values);
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
        let mut state = ComponentPropertyDraft::default();

        state.reset(ComponentType::VoltageSourcePwl, &sheet, values);
        state.sync_pwl_validation_error();

        assert_eq!(state.pwl_editor.raw_source_draft(), Some(""));
        assert!(state.validation_errors.contains_key("pwl_data"));
        assert!(!state.pwl_editor.is_valid());
    }

    #[test]
    fn live_pwl_validation_blocks_apply_and_repair_clears_parent_error() {
        let sheet = pwl_sheet();
        let mut values = HashMap::new();
        values.insert(
            "pwl_data".to_owned(),
            PropertyValue::String("0 0 1n 1".to_owned()),
        );
        let mut state = ComponentPropertyDraft::default();
        state.reset(ComponentType::VoltageSourcePwl, &sheet, values);

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
        let mut state = clean_draft(&sheet);

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
    fn partial_commit_preserves_invalid_numeric_source_until_reset() {
        let sheet = constrained_sheet();
        let mut state = clean_draft(&sheet);
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

        state.reset(
            ComponentType::Resistor,
            &sheet,
            state.original_values.clone(),
        );
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
        let registry = PropertyCatalog::new();
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
        let registry = PropertyCatalog::new();
        let sheet = registry.get(ComponentType::VoltageSource).unwrap();
        let stored = 89.123_456_789_012_3;
        let values = HashMap::from([("acphase".to_owned(), PropertyValue::number(stored))]);
        let component = Component::new(12, ComponentType::VoltageSource, Point::origin());
        let mut state = ComponentPropertyDraft::default();

        state.reset(component.kind, sheet, values);

        let expected = format!("{} deg", stored);
        assert_eq!(state.numeric_text_draft("acphase"), Some(expected.as_str()));
    }
}
