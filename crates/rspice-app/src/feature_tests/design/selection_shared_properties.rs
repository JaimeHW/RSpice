//! Editing the properties a selection shares through the whole application: the dialog
//! asks the application whether a dialog holds the canvas.

use crate::workbench::app_state::AppState;
use rspice_app_design::properties::property_bridge::collect_properties_from_component;
use rspice_app_design::properties::property_bridge::property_value_to_string;
use rspice_app_design::workbench::app::dialogs::selection::shared_properties::SharedField;
use rspice_app_design::workbench::app::dialogs::selection::shared_properties::tests::resistor;
use rspice_app_state::state::ComponentType;

/// Open the form, over the parts it reads and edits.
fn open_shared_properties(state: &mut AppState) -> bool {
    let modal_open = state.dialog_modal_open();
    let design = &mut state.design;
    rspice_app_design::workbench::app::dialogs::selection::shared_properties::open_shared_properties(
        &mut design.shared_properties,
        &design.property_registry,
        modal_open,
        &state.model,
        &mut state.workbench,
    )
}

/// The apply, over the parts it reads and edits.
fn apply_shared_properties(state: &mut AppState) -> Result<usize, String> {
    rspice_app_design::workbench::app::dialogs::selection::shared_properties::apply_shared_properties(
        &state.design.shared_properties,
        &state.design.property_registry,
        &mut state.model,
        &mut state.workbench,
    )
}

fn two_resistors() -> AppState {
    let mut state = AppState::default();
    let document = state.model.schematic.document_mut_for_test();
    document.components = vec![resistor(1, "R1", "1k"), resistor(2, "R2", "2k")];
    document.wires.clear();
    state.model.schematic.init_undo_history();
    let selection = &mut state.model.schematic.session.editor.selection;
    selection.select_component(1);
    selection.select_component(2);
    state
}

fn field<'a>(state: &'a mut AppState, key: &str) -> &'a mut SharedField {
    state
        .design
        .shared_properties
        .fields
        .iter_mut()
        .find(|field| field.key == key)
        .unwrap_or_else(|| panic!("the form offers {key}"))
}

fn property(state: &AppState, id: u64, key: &str) -> Option<String> {
    let component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the instance");
    collect_properties_from_component(component, &state.design.property_registry)
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
            .design
            .shared_properties
            .fields
            .iter()
            .any(|field| field.key == "name"),
        "two instances never share one name"
    );
    field(&mut state, "m").text = "2".to_owned();

    assert_eq!(apply_shared_properties(&mut state), Ok(2));
    assert_eq!(state.model.schematic.document().components[0].value, "1k");
    assert_eq!(state.model.schematic.document().components[1].value, "2k");
    assert_eq!(property(&state, 1, "m").as_deref(), Some("2"));
    assert_eq!(property(&state, 2, "m").as_deref(), Some("2"));
    assert!(state.model.schematic.undo(), "Apply is one undo step");
    assert_eq!(property(&state, 1, "m"), None);
    assert_eq!(property(&state, 2, "m"), None);
    assert!(!state.model.schematic.can_undo());
}

/// Typing into the mixed field sets every instance to that value.
#[test]
fn typing_into_a_mixed_field_sets_every_instance() {
    let mut state = two_resistors();
    assert!(open_shared_properties(&mut state));
    field(&mut state, "r").text = "4.7k".to_owned();
    assert_eq!(apply_shared_properties(&mut state), Ok(2));
    assert_eq!(state.model.schematic.document().components[0].value, "4.7k");
    assert_eq!(state.model.schematic.document().components[1].value, "4.7k");
}

/// A resistor and a capacitor share only what both kinds own: neither
/// the resistance nor the capacitance is offered.
#[test]
fn only_what_every_kind_owns_is_offered() {
    let mut state = two_resistors();
    let document = state.model.schematic.document_mut_for_test();
    document.components[1].kind = ComponentType::Capacitor;
    assert!(open_shared_properties(&mut state));
    let keys: Vec<&str> = state
        .design
        .shared_properties
        .fields
        .iter()
        .map(|field| field.key.as_str())
        .collect();
    assert!(!keys.contains(&"r") && !keys.contains(&"c"), "{keys:?}");
    assert!(keys.contains(&"m"), "{keys:?}");
}
