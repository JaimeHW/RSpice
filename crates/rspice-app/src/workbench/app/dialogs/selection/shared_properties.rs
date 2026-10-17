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

use crate::properties::property_bridge::{
    apply_properties_to_component, collect_properties_from_component, property_value_to_string,
    validate_component_properties,
};
use crate::state::{Component, ComponentType, PropertyValue};
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const TITLE: &str = "Properties";
const PRIMARY: &str = "Apply";
const DESCRIPTION: &str = "Edit the properties the selected instances share.";
/// What a field shows when the selected instances disagree.
const MIXED: &str = "mixed";

/// One property every selected instance's kind owns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SharedField {
    pub(crate) key: String,
    pub(crate) label: String,
    /// The value every instance holds (empty when none sets it), or `None`
    /// when they differ.
    pub(crate) common: Option<String>,
    pub(crate) text: String,
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
pub(crate) struct SharedPropertiesDialogState {
    pub(crate) open: bool,
    pub(crate) ids: Vec<u64>,
    pub(crate) fields: Vec<SharedField>,
    /// The drawing the form was opened on; an edit since refuses Apply.
    pub(crate) content_version: u64,
    pub(crate) error: Option<String>,
}

/// The selected instances, when there are several the form can edit: every
/// selected object an instance, and every one of a kind with a property
/// schema.
fn selected_instances(state: &AppState) -> Option<Vec<u64>> {
    let selection = &state.schematic.session.editor.selection;
    let ids: Vec<u64> = state
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| selection.has_component(component.id))
        .filter(|component| {
            component.kind != ComponentType::CellInstance
                && state.property_registry.get(component.kind).is_some()
        })
        .map(|component| component.id)
        .collect();
    (ids.len() >= 2 && ids.len() == selection.components.len()).then_some(ids)
}

pub(crate) fn shared_properties_available(state: &AppState) -> bool {
    !state.schematic_edit_read_only() && selected_instances(state).is_some()
}

/// Open the form on the selection. Returns whether it opened.
pub(crate) fn open_shared_properties(state: &mut AppState) -> bool {
    if state.dialogs.application_modal_open() || state.deny_read_only_edit() {
        return false;
    }
    let Some(ids) = selected_instances(state) else {
        return false;
    };
    let fields = shared_fields(state, &ids);
    state.dialogs.shared_properties = SharedPropertiesDialogState {
        open: true,
        ids,
        fields,
        content_version: state.schematic.content_version(),
        error: None,
    };
    true
}

fn instances<'a>(state: &'a AppState, ids: &[u64]) -> Vec<&'a Component> {
    let document = state.schematic.document();
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
fn shared_fields(state: &AppState, ids: &[u64]) -> Vec<SharedField> {
    let components = instances(state, ids);
    let registry = &state.property_registry;
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
pub(crate) fn apply_shared_properties(state: &mut AppState) -> Result<usize, String> {
    let form = state.dialogs.shared_properties.clone();
    if form.content_version != state.schematic.content_version() {
        return Err(
            "The drawing changed while the form was open. Close it and press Q again.".to_owned(),
        );
    }
    let edited: Vec<&SharedField> = form.fields.iter().filter(|field| field.edited()).collect();
    if edited.is_empty() {
        return Ok(0);
    }
    let registry = &state.property_registry;
    let mut edits = Vec::new();
    for expected in instances(state, &form.ids) {
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
        &mut state.schematic,
        edits,
        "edit selected instances",
    )?;
    if changed > 0 {
        state.sync_active_schematic_to_workspace();
    }
    Ok(changed)
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_shared_properties_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.shared_properties.open {
            return;
        }
        let names: Vec<String> = instances(&self.state, &self.state.dialogs.shared_properties.ids)
            .into_iter()
            .map(|component| component.name.clone())
            .collect();
        let context = format!("{} instances \u{b7} {}", names.len(), names.join(" "));
        let form = &mut self.state.dialogs.shared_properties;
        let any_edited = form.fields.iter().any(SharedField::edited);
        let status = match (&form.error, form.fields.is_empty()) {
            (Some(error), _) => Err(error.as_str()),
            (None, true) => Ok(Some("The selected instances share no property.")),
            (None, false) => Ok(None),
        };
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(context.as_str()))
            .describe(DESCRIPTION)
            .primary_enabled(any_edited)
            .status(status)
            .status_lines(2)
            .show(ctx, |rows| {
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
            DialogChoice::Primary => match apply_shared_properties(&mut self.state) {
                Ok(changed) => {
                    self.state.dialogs.shared_properties.open = false;
                    self.state.prompt(match changed {
                        0 => "Nothing changed.".to_owned(),
                        1 => "Changed 1 instance.".to_owned(),
                        count => format!("Changed {count} instances."),
                    });
                }
                Err(error) => self.state.dialogs.shared_properties.error = Some(error),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.shared_properties.open = false;
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    fn resistor(id: u64, name: &str, value: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component
    }

    fn two_resistors() -> AppState {
        let mut state = AppState::default();
        let document = state.schematic.document_mut_for_test();
        document.components = vec![resistor(1, "R1", "1k"), resistor(2, "R2", "2k")];
        document.wires.clear();
        state.schematic.init_undo_history();
        let selection = &mut state.schematic.session.editor.selection;
        selection.select_component(1);
        selection.select_component(2);
        state
    }

    fn field<'a>(state: &'a mut AppState, key: &str) -> &'a mut SharedField {
        state
            .dialogs
            .shared_properties
            .fields
            .iter_mut()
            .find(|field| field.key == key)
            .unwrap_or_else(|| panic!("the form offers {key}"))
    }

    fn property(state: &AppState, id: u64, key: &str) -> Option<String> {
        let component = state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.id == id)
            .expect("the instance");
        collect_properties_from_component(component, &state.property_registry)
            .get(key)
            .map(property_value_to_string)
    }

    /// The resistances differ, so the field reads "mixed" and stays as each
    /// instance has it; the multiplier, set on neither, is written to both;
    /// one undo takes it back (audit G-10).
    #[test]
    fn a_mixed_field_stays_and_an_edited_one_is_written_in_one_undo_step() {
        let mut state = two_resistors();
        assert!(open_shared_properties(&mut state));
        assert_eq!(field(&mut state, "r").common, None, "1k and 2k are mixed");
        assert_eq!(field(&mut state, "r").text, "");
        assert_eq!(field(&mut state, "m").common.as_deref(), Some(""));
        assert!(
            !state
                .dialogs
                .shared_properties
                .fields
                .iter()
                .any(|field| field.key == "name"),
            "two instances never share one name"
        );
        field(&mut state, "m").text = "2".to_owned();

        assert_eq!(apply_shared_properties(&mut state), Ok(2));
        assert_eq!(state.schematic.document().components[0].value, "1k");
        assert_eq!(state.schematic.document().components[1].value, "2k");
        assert_eq!(property(&state, 1, "m").as_deref(), Some("2"));
        assert_eq!(property(&state, 2, "m").as_deref(), Some("2"));
        assert!(state.schematic.undo(), "Apply is one undo step");
        assert_eq!(property(&state, 1, "m"), None);
        assert_eq!(property(&state, 2, "m"), None);
        assert!(!state.schematic.can_undo());
    }

    /// Typing into the mixed field sets every instance to that value.
    #[test]
    fn typing_into_a_mixed_field_sets_every_instance() {
        let mut state = two_resistors();
        assert!(open_shared_properties(&mut state));
        field(&mut state, "r").text = "4.7k".to_owned();
        assert_eq!(apply_shared_properties(&mut state), Ok(2));
        assert_eq!(state.schematic.document().components[0].value, "4.7k");
        assert_eq!(state.schematic.document().components[1].value, "4.7k");
    }

    /// A resistor and a capacitor share only what both kinds own: neither
    /// the resistance nor the capacitance is offered.
    #[test]
    fn only_what_every_kind_owns_is_offered() {
        let mut state = two_resistors();
        let document = state.schematic.document_mut_for_test();
        document.components[1].kind = ComponentType::Capacitor;
        assert!(open_shared_properties(&mut state));
        let keys: Vec<&str> = state
            .dialogs
            .shared_properties
            .fields
            .iter()
            .map(|field| field.key.as_str())
            .collect();
        assert!(!keys.contains(&"r") && !keys.contains(&"c"), "{keys:?}");
        assert!(keys.contains(&"m"), "{keys:?}");
    }

    /// One instance is not this form's: Q there opens the single form.
    #[test]
    fn one_instance_is_not_offered() {
        let mut state = two_resistors();
        state.schematic.session.editor.selection.clear();
        state.schematic.session.editor.selection.select_component(1);
        assert!(!shared_properties_available(&state));
    }

    /// The form at desktop and phone widths, in both themes.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_shared_properties() {
        use crate::ui::tokens::Mode;
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for (size_stem, size) in [
            ("1024x640", egui::vec2(1024.0, 640.0)),
            ("390x844", egui::vec2(390.0, 844.0)),
        ] {
            for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                let mut app = RSpiceApp::test_instance();
                let document = app.state.schematic.document_mut_for_test();
                document.components = vec![
                    resistor(1, "R1", "1k"),
                    resistor(2, "R2", "2k"),
                    resistor(3, "R3", "1k"),
                ];
                for component in &mut document.components {
                    component.params = "m=2".to_owned();
                }
                let selection = &mut app.state.schematic.session.editor.selection;
                for id in 1..=3 {
                    selection.select_component(id);
                }
                assert!(open_shared_properties(&mut app.state));
                let canvas = super::super::super::render_settled_form(mode, size, |ctx| {
                    app.render_shared_properties_dialog(ctx);
                });
                let height = canvas.content_height().max(1);
                let path = directory.join(format!("shared-properties-{size_stem}-{mode_stem}.png"));
                std::fs::write(&path, canvas.png(height)).expect("write dialog render");
            }
        }
    }
}
