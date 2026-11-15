//! Q on several selected instances: one form for the properties they share,
//! as Virtuoso's Edit Properties applies to all selected.
//!
//! Only a property every selected instance's kind owns is offered. A field
//! whose value agrees across the selection shows it; one that differs shows
//! "mixed" in its empty box, and is left as each instance has it unless
//! something is typed there. Apply is one undo step, and nothing changes
//! unless every instance accepts its new values.

use std::collections::HashMap;

use egui::{Context, Id};

use crate::properties::PropertyEditorSchema;
use crate::properties::property_bridge::{
    apply_properties_to_component, collect_properties_from_component, property_value_to_string,
    validate_component_properties,
};
use crate::state::{Component, ComponentType, PropertyValue};
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::actions::workspace::{deny_read_only_edit, sync_active_schematic};
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

const TITLE: &str = "Properties";
const PRIMARY: &str = "Apply";
const DESCRIPTION: &str = "Edit the properties the selected instances share.";
/// What a field shows when the selected instances disagree.
const MIXED: &str = "mixed";

/// One property every selected instance's kind owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedField {
    pub key: String,
    pub(crate) label: String,
    /// The value every instance holds (empty when none sets it), or `None`
    /// when they differ.
    pub common: Option<String>,
    pub text: String,
}

impl SharedField {
    /// Whether the reader changed the field: typed into a mixed one, or
    /// changed an agreed one.
    pub(crate) fn edited(&self) -> bool {
        match &self.common {
            Some(common) => self.text != *common,
            None => !self.text.is_empty(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SharedPropertiesDialogState {
    pub open: bool,
    pub(crate) ids: Vec<u64>,
    pub fields: Vec<SharedField>,
    /// The drawing the form was opened on; an edit since refuses Apply.
    pub(crate) content_version: u64,
    pub(crate) error: Option<String>,
}

/// The selected instances, when there are several the form can edit: every
/// selected object an instance, and every one of a kind with a property
/// schema.
fn selected_instances(registry: &PropertyEditorSchema, model: &Model) -> Option<Vec<u64>> {
    let selection = &model.schematic.session.editor.selection;
    let ids: Vec<u64> = model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .filter(|component| {
            component.kind != ComponentType::CellInstance && registry.get(component.kind).is_some()
        })
        .map(|component| component.id)
        .collect();
    (ids.len() >= 2 && ids.len() == selection.components.len()).then_some(ids)
}

/// Whether the form can open: the schematic is editable and the selection
/// is several instances it can edit, read through the property schema.
pub(crate) fn shared_properties_available(registry: &PropertyEditorSchema, model: &Model) -> bool {
    !model.schematic_edit_read_only() && selected_instances(registry, model).is_some()
}

/// Open the form on the selection, unless a dialog holds the workbench
/// (`modal_open`) or the schematic refuses the edit. Returns whether it
/// opened.
pub fn open_shared_properties(
    dialog: &mut SharedPropertiesDialogState,
    registry: &PropertyEditorSchema,
    modal_open: bool,
    model: &Model,
    wb: &mut WorkbenchSession,
) -> bool {
    if modal_open || deny_read_only_edit(model, wb) {
        return false;
    }
    let Some(ids) = selected_instances(registry, model) else {
        return false;
    };
    let fields = shared_fields(registry, model, &ids);
    *dialog = SharedPropertiesDialogState {
        open: true,
        ids,
        fields,
        content_version: model.schematic.content_version(),
        error: None,
    };
    true
}

fn instances<'a>(model: &'a Model, ids: &[u64]) -> Vec<&'a Component> {
    let document = model.schematic.document();
    ids.iter()
        .filter_map(|id| {
            document
                .components
                .iter()
                .find(|component| component.id == *id)
        })
        .collect()
}

/// The properties every instance in `ids` owns, in the first one's sheet
/// order, each with the value they agree on if they do.
fn shared_fields(registry: &PropertyEditorSchema, model: &Model, ids: &[u64]) -> Vec<SharedField> {
    let components = instances(model, ids);
    let Some(sheet) = components
        .first()
        .and_then(|first| registry.get(first.kind))
    else {
        return Vec::new();
    };
    let values: Vec<HashMap<String, PropertyValue>> = components
        .iter()
        .map(|component| collect_properties_from_component(component, registry))
        .collect();
    sheet
        .iter()
        .filter(|definition| definition.name != "name" && definition.name != "symbol")
        .filter(|definition| {
            components.iter().all(|component| {
                registry
                    .get(component.kind)
                    .is_some_and(|sheet| sheet.get(&definition.name).is_some())
            })
        })
        .map(|definition| {
            let texts: Vec<String> = values
                .iter()
                .map(|properties| {
                    properties
                        .get(&definition.name)
                        .map(property_value_to_string)
                        .unwrap_or_default()
                })
                .collect();
            let common = texts
                .iter()
                .all(|text| *text == texts[0])
                .then(|| texts[0].clone());
            SharedField {
                key: definition.name.clone(),
                label: if definition.display_name.is_empty() {
                    definition.name.clone()
                } else {
                    definition.display_name.clone()
                },
                text: common.clone().unwrap_or_default(),
                common,
            }
        })
        .collect()
}

/// The edited text as the value a field of `current`'s type takes.
fn retyped(current: &PropertyValue, text: &str) -> PropertyValue {
    match current {
        PropertyValue::String(_) => PropertyValue::String(text.to_owned()),
        PropertyValue::Number { .. } | PropertyValue::Expression(_) => {
            PropertyValue::Expression(text.to_owned())
        }
        PropertyValue::Enum { options, .. } => PropertyValue::Enum {
            selected: text.to_owned(),
            options: options.clone(),
        },
        PropertyValue::Boolean(value) => {
            PropertyValue::Boolean(match text.trim().to_ascii_lowercase().as_str() {
                "true" | "yes" | "on" | "1" => true,
                "false" | "no" | "off" | "0" => false,
                _ => *value,
            })
        }
    }
}

/// Write every edited field to every instance, as one undo step. Returns
/// how many instances changed, or which one refused and why.
pub fn apply_shared_properties(
    form: &SharedPropertiesDialogState,
    registry: &PropertyEditorSchema,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<usize, String> {
    let form = form.clone();
    if form.content_version != model.schematic.content_version() {
        return Err(
            "The drawing changed while the form was open. Close it and press Q again.".to_owned(),
        );
    }
    let edited: Vec<&SharedField> = form.fields.iter().filter(|field| field.edited()).collect();
    if edited.is_empty() {
        return Ok(0);
    }
    let mut edits = Vec::new();
    for expected in instances(model, &form.ids) {
        let sheet = registry.get(expected.kind);
        let mut properties = collect_properties_from_component(expected, registry);
        for field in &edited {
            let Some(current) = properties.get(&field.key).cloned().or_else(|| {
                sheet
                    .and_then(|sheet| sheet.get(&field.key))
                    .map(|definition| definition.default_value.clone())
            }) else {
                continue;
            };
            properties.insert(field.key.clone(), retyped(&current, &field.text));
        }
        let mut candidate = expected.clone();
        apply_properties_to_component(&mut candidate, &properties, registry)
            .map_err(|error| format!("{}: {error}", expected.name))?;
        if let Some((field, error)) = validate_component_properties(&candidate, sheet)
            .into_iter()
            .next()
        {
            return Err(format!("{} {field}: {error}", expected.name));
        }
        edits.push((expected.clone(), candidate));
    }
    if edits.len() != form.ids.len() {
        return Err("A selected instance no longer exists.".to_owned());
    }
    let changed = crate::state::bulk_edit::replace_components(
        &mut model.schematic,
        edits,
        "edit selected instances",
    )?;
    if changed > 0 {
        sync_active_schematic(model, workbench);
    }
    Ok(changed)
}

/// Draw the shared-properties form over its draft, the property sheets it
/// reads, and the model it edits.
pub(in crate::workbench) fn render_shared_properties_dialog(
    dialog: &mut SharedPropertiesDialogState,
    registry: &PropertyEditorSchema,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }
    let names: Vec<String> = instances(model, &dialog.ids)
        .into_iter()
        .map(|component| component.name.clone())
        .collect();
    let instances = names.join(", ");
    let form = &mut *dialog;
    let any_edited = form.fields.iter().any(SharedField::edited);
    let status = match (&form.error, form.fields.is_empty()) {
        (Some(error), _) => Err(error.as_str()),
        (None, true) => Ok(Some("The selected instances share no property.")),
        (None, false) => Ok(None),
    };
    let choice = CommandForm::new(TITLE, PRIMARY)
        .describe(DESCRIPTION)
        .primary_enabled(any_edited)
        .status(status)
        .show(ctx, |rows| {
            rows.value("Instances", &instances);
            let mut first = None;
            for field in &mut form.fields {
                let id = Id::new(("shared-property", field.key.as_str()));
                // An empty box no instance sets stays empty: a default
                // drawn as a hint reads as a value someone set.
                let hint = if field.common.is_none() { MIXED } else { "" };
                rows.text(&field.label, id, &mut field.text, hint);
                first.get_or_insert(id);
            }
            first
        });
    match choice {
        DialogChoice::Primary => {
            match apply_shared_properties(dialog, registry, model, workbench) {
                Ok(changed) => {
                    dialog.open = false;
                    workbench.prompt(match changed {
                        0 => "Nothing changed.".to_owned(),
                        1 => "Changed 1 instance.".to_owned(),
                        count => format!("Changed {count} instances."),
                    });
                }
                Err(error) => dialog.error = Some(error),
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.open = false;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {
    use super::*;
    use crate::state::Point;
    #[cfg(test)]
    use crate::workbench::design::test_host::DesignTestHost;

    /// Whether the form can open, over the parts it reads.
    #[cfg(test)]
    fn shared_properties_available(state: &DesignTestHost) -> bool {
        super::shared_properties_available(&state.design.property_registry, &state.model)
    }

    pub fn resistor(id: u64, name: &str, value: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component
    }

    #[cfg(test)]
    fn two_resistors() -> DesignTestHost {
        let mut state = DesignTestHost::default();
        let document = state.model.schematic.document_mut_for_test();
        document.components = vec![resistor(1, "R1", "1k"), resistor(2, "R2", "2k")];
        document.wires.clear();
        state.model.schematic.init_undo_history();
        let selection = &mut state.model.schematic.session.editor.selection;
        selection.select_component(1);
        selection.select_component(2);
        state
    }

    /// One instance is not this form's: Q there opens the single form.
    #[test]
    fn one_instance_is_not_offered() {
        let mut state = two_resistors();
        state.model.schematic.session.editor.selection.clear();
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(1);
        assert!(!shared_properties_available(&state));
    }
}
