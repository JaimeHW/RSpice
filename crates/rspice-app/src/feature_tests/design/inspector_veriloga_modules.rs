//! The inspector's Verilog-A module cards drawn through a whole application.

use crate::state::{CellViewRef, Component, ComponentType, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::design::inspector::component::{
    MODEL_CARD_PARAMETER_NOTE, REMOVE_UNDECLARED_PARAMETER, UNDECLARED_PARAMETER_NOTE,
    identity_section, inline_parameter_contract, model_cards_of_module, parameters_section,
};
use crate::workbench::design::inspector::inline_instance::{begin_edit, field_value, update_edit};
use crate::workbench::design::inspector::model_evidence::component_model_evidence;
use crate::workbench::design::inspector::tests::veriloga_modules::{
    app_with_compiled_veriloga_instance, app_with_owned_veriloga_instance, module_parameter,
};
use crate::workbench::design::inspector::tests::*;
use crate::workbench::design_system::Problem;
use crate::workbench::state::InlineEditField;
use std::collections::HashMap;

/// The unit a parameter is in sits beside its field and what it is sits on
/// its label's hover, for a compiled module's parameter and for a native
/// device's alike; a module's computed default reads as such.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn parameter_rows_carry_units_and_descriptions_for_modules_and_native_devices() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[
            module_parameter("gain", Some("2.5"), Some("S"), Some("Transconductance")),
            module_parameter("vmax", None, Some("V"), None),
        ],
        "",
    );
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &component,
        &HashMap::new(),
    );
    let row = |key: &str| {
        rows.iter()
            .find(|row| row.key == key)
            .unwrap_or_else(|| panic!("a {key} row: {rows:?}"))
    };
    assert_eq!(row("gain").unit.as_deref(), Some("S"));
    assert_eq!(row("gain").hint, "2.5");
    assert!(
        row("gain")
            .description
            .as_deref()
            .is_some_and(|description| description.starts_with("Transconductance.")),
        "{:?}",
        row("gain").description
    );
    assert_eq!(row("vmax").hint, "computed");

    let evidence = component_model_evidence(&app.state.model, &component);
    let painted =
        painted_panel(|ui| parameters_section(ui, &mut app.design_cx(), &component, &evidence));
    assert!(
        painted.lines().any(|line| line == "S") && painted.lines().any(|line| line == "V"),
        "each unit is painted beside its field:\n{painted}"
    );
    let nodes =
        accesskit_nodes(|ui| parameters_section(ui, &mut app.design_cx(), &component, &evidence));
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("gain, S")
                && node
                    .description()
                    .is_some_and(|description| description.starts_with("Transconductance."))
        }),
        "the field is described by the module's own words"
    );

    let capacitor = Component::new(9, ComponentType::Capacitor, Point::origin());
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &capacitor,
        &HashMap::new(),
    );
    let tc1 = rows
        .iter()
        .find(|row| row.key == "tc1")
        .expect("a capacitor's temperature coefficient row");
    assert_eq!(tc1.unit.as_deref(), Some("1/°C"));
    assert_eq!(
        tc1.description.as_deref(),
        Some("First-order temperature coefficient")
    );
}

/// A number typed into a module's parameter stays as typed, and the `M` a
/// module's source writes for mega reaches the deck as mega.
#[test]
fn a_number_typed_for_a_module_stays_as_typed_and_reads_as_the_module_writes_it() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("10000000"), None, None)],
        "",
    );
    let field = InlineEditField::Parameter("gain".to_owned());
    for (typed, stored, value) in [
        ("1e5", "1e5", 1e5),
        ("100k", "100k", 1e5),
        ("10M", "10Meg", 1e7),
        ("10m", "10m", 1e-2),
    ] {
        assert!(begin_edit(
            &mut app.design_cx(),
            component.id,
            field.clone()
        ));
        update_edit(&mut app.design_cx(), typed.to_owned());
        assert!(
            app.state.commit_inline_component_edit().expect(typed),
            "{typed} commits"
        );
        let edited = app.state.model.schematic.document().components[0].clone();
        assert_eq!(field_value(&edited, &field), stored);
        let read = rspice_vocabulary::quantity::engineering::parse_engineering_value(stored);
        assert_eq!(read, Ok(value), "the deck reads {stored}");
    }
}

/// A module's array takes its list in either spelling, stored as one, and a
/// list of the wrong length says so without losing the module's own words,
/// in a field that is named with its unit and a message that is announced.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_array_parameter_takes_a_list_of_its_length_and_reports_one_that_is_not() {
    let mut breakpoints = module_parameter("pa", None, Some("V"), Some("Breakpoints"));
    let declaration = breakpoints.declaration.as_mut().expect("a declaration");
    declaration.is_array = true;
    declaration.array_length = Some(2);
    declaration.authored_default = Some("'{1k, 2}".to_owned());
    let (mut app, component) = app_with_compiled_veriloga_instance(&[breakpoints], "");
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &component,
        &HashMap::new(),
    );
    let row = rows.iter().find(|row| row.key == "pa").expect("a pa row");
    assert_eq!(row.hint, "'{1k, 2}");

    let field = InlineEditField::Parameter("pa".to_owned());
    assert!(begin_edit(
        &mut app.design_cx(),
        component.id,
        field.clone()
    ));
    update_edit(&mut app.design_cx(), "'{4, 5M}'".to_owned());
    assert!(
        app.state
            .commit_inline_component_edit()
            .expect("a list of two")
    );
    let edited = app.state.model.schematic.document().components[0].clone();
    assert_eq!(field_value(&edited, &field), "[4 5Meg]");

    assert!(begin_edit(
        &mut app.design_cx(),
        component.id,
        field.clone()
    ));
    update_edit(&mut app.design_cx(), "[1 2 3]".to_owned());
    let evidence = component_model_evidence(&app.state.model, &edited);
    let nodes =
        accesskit_nodes(|ui| parameters_section(ui, &mut app.design_cx(), &edited, &evidence));
    let error = "pa takes 2 values; this has 3";
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("pa, V")
                && node.invalid().is_some()
                && node.description().is_some_and(|description| {
                    description.starts_with("Breakpoints.") && description.contains(error)
                })
        }),
        "the field keeps the module's description and adds the error"
    );
    assert!(
        nodes.iter().any(|(_, node)| {
            node.role() == egui::accesskit::Role::Status
                && node.label() == Some(error)
                && node.live() == Some(egui::accesskit::Live::Polite)
        }),
        "the message is announced when it appears"
    );
}

/// A library card typed for the instance's module offers a Model row. Binding
/// one leaves only instance parameters on the panel, and a model parameter
/// the instance already sets is flagged with its repair.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_module_instance_binds_a_model_card_and_then_sets_only_instance_parameters() {
    use rspice_design::library::metadata::ParameterScope;

    let mut model_scoped = module_parameter("r", Some("1000"), None, None);
    model_scoped.declaration.as_mut().expect("declared").scope = ParameterScope::Model;
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[model_scoped, module_parameter("w", Some("1"), None, None)],
        "r=2k w=3",
    );
    let evidence = component_model_evidence(&app.state.model, &component);
    let painted = |app: &mut RSpiceApp, component: &Component| {
        painted_panel(|ui| parameters_section(ui, &mut app.design_cx(), component, &evidence))
    };
    assert!(
        !painted(&mut app, &component)
            .lines()
            .any(|line| line == "Model"),
        "no card for this module, no row"
    );

    let mut library = crate::state::model_library::ModelLibrary::new("cards");
    let card = |name: &str, spice_type: &str| {
        let mut card = crate::state::model_library::DeviceModel::new(
            name,
            crate::state::model_library::ModelType::Other,
        );
        card.spice_type = Some(spice_type.to_owned());
        card
    };
    library.add_model(card("slowamp", "AMP"));
    library.add_model(card("fastamp", "amp"));
    library.add_model(card("other", "MIXER"));
    app.state.model.model_library_manager.add_library(library);
    assert_eq!(
        model_cards_of_module(&app.state.model, "amp"),
        ["fastamp", "slowamp"]
    );
    assert!(
        painted(&mut app, &component)
            .lines()
            .any(|line| line == "Model"),
        "the row appears with the cards"
    );

    assert!(
        app.state
            .model
            .schematic
            .select_instance_model_card(component.id, Some("fastamp".to_owned()))
    );
    let bound = app.state.model.schematic.document().components[0].clone();
    assert_eq!(
        crate::state::explicit_component_model(&bound).as_deref(),
        Some("fastamp")
    );
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &bound,
        &crate::state::parse_params_string(&bound.params),
    );
    let row = |key: &str| rows.iter().find(|row| row.key == key);
    assert!(!row("w").expect("an instance parameter").undeclared);
    assert!(
        row("r").expect("the value already set").undeclared,
        "a value the card owns is flagged, not dropped: {rows:?}"
    );
    assert!(app.state.model.model_card_owns_parameter(&bound, "r"));
    assert!(!app.state.model.model_card_owns_parameter(&bound, "w"));
    assert_eq!(
        app.state.model.undeclared_instance_parameters(&bound),
        [("r".to_owned(), "2k".to_owned())]
    );
    assert!(
        painted(&mut app, &bound)
            .lines()
            .any(|line| line == MODEL_CARD_PARAMETER_NOTE),
        "the row says why"
    );

    // Unbinding is one undoable step back to the module's own parameters.
    assert!(app.state.model.schematic.undo());
    let unbound = app.state.model.schematic.document().components[0].clone();
    assert!(
        app.state
            .model
            .undeclared_instance_parameters(&unbound)
            .is_empty()
    );

    // A card that left the libraries stays named, with the reason.
    app.state.model.model_library_manager.clear();
    assert!(
        app.state
            .model
            .schematic
            .select_instance_model_card(component.id, Some("gone".to_owned()))
    );
    let orphaned = app.state.model.schematic.document().components[0].clone();
    let text = painted(&mut app, &orphaned);
    assert!(
        text.contains("Model gone not found in the loaded libraries"),
        "{text}"
    );
}

/// `m` is a module's `$mfactor`: a module instance keeps its multiplicity
/// row, and an edit there sets the typed multiplicity the deck emits, never
/// parameter text the netlister would refuse.
#[test]
fn m_on_a_module_instance_is_its_typed_multiplicity() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &component,
        &HashMap::new(),
    );
    assert!(rows.iter().any(|row| row.key == "m"), "{rows:?}");

    let field = InlineEditField::Parameter("m".to_owned());
    assert!(begin_edit(
        &mut app.design_cx(),
        component.id,
        field.clone()
    ));
    update_edit(&mut app.design_cx(), "2".to_owned());
    app.state
        .commit_inline_component_edit()
        .expect("a multiplicity of 2 commits");
    let edited = app.state.model.schematic.document().components[0].clone();
    assert_eq!(
        edited
            .multiplicity
            .map(crate::state::InstanceMultiplicity::value),
        Some(2.0)
    );
    assert!(
        crate::state::parse_params_string(&edited.params).is_empty(),
        "m is never written as parameter text: {}",
        edited.params
    );
    assert_eq!(field_value(&edited, &field), "2");
}

/// A value for a parameter the module no longer declares is shown, flagged,
/// and removable, and the remove is one undoable edit that touches nothing
/// else.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_undeclared_parameter_value_is_flagged_and_removable() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "gain=3 offset=1m",
    );
    let declared = crate::state::parse_params_string(&component.params);
    let rows =
        inline_parameter_contract(&app.state.design, &app.state.model, &component, &declared);
    let offset = rows
        .iter()
        .find(|row| row.key == "offset")
        .expect("the stale value has a row");
    assert!(offset.undeclared && !offset.editable, "{offset:?}");
    assert!(
        rows.iter()
            .filter(|row| row.key == "gain")
            .all(|row| !row.undeclared)
    );

    let evidence = component_model_evidence(&app.state.model, &component);
    let painted =
        painted_panel(|ui| parameters_section(ui, &mut app.design_cx(), &component, &evidence));
    assert!(
        painted.contains(UNDECLARED_PARAMETER_NOTE)
            && painted.contains(REMOVE_UNDECLARED_PARAMETER),
        "{painted}"
    );

    assert!(
        app.state
            .model
            .remove_undeclared_instance_parameter(component.id, "gain")
            .is_err(),
        "a declared parameter is edited, not removed"
    );
    assert_eq!(
        app.state
            .model
            .remove_undeclared_instance_parameter(component.id, "offset"),
        Ok(true)
    );
    let removed = &app.state.model.schematic.document().components[0];
    assert_eq!(removed.params, "gain=3");
    assert!(app.state.model.schematic.can_undo());
}

/// An instance of a module says what it is and nothing a model library
/// would: its view is a Verilog-A module, it runs that module, and no model
/// card or library section is offered. A current compile is not a warning.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_module_instance_states_its_view_and_module_and_no_model_library_rows() {
    let (mut app, component) = app_with_owned_veriloga_instance();

    let evidence = component_model_evidence(&app.state.model, &component);
    assert_eq!(evidence.model.as_deref(), Some("amp"));
    assert_eq!(evidence.problem, None, "a current compile says nothing");

    let identity = painted_panel(|ui| identity_section(ui, &mut app.design_cx(), &component));
    assert!(
        identity.lines().any(|line| line == "veriloga"),
        "{identity}"
    );
    assert!(!identity.contains("spice"), "{identity}");
    assert!(
        identity.lines().any(|line| line == "Open Verilog-A view")
            && !identity.contains("model source"),
        "{identity}"
    );
    let parameters =
        painted_panel(|ui| parameters_section(ui, &mut app.design_cx(), &component, &evidence));
    let labels = parameters.lines().collect::<Vec<_>>();
    assert!(
        !labels.contains(&"Module"),
        "the module is the cell the Library cell row names: {parameters}"
    );
    assert!(
        !labels.contains(&"editable"),
        "a writable document is the normal state: {parameters}"
    );
    assert!(
        !labels.contains(&"Model") && !labels.contains(&"Section"),
        "{parameters}"
    );

    // An edit since the compile is worth a look, and says so.
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&crate::state::ProjectSourceOwner::cell_view(
            CellViewRef::new("behavioral", "amp", "veriloga"),
        ))
        .expect("the bundle")
        .id();
    app.state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(
            bundle,
            "amp.va",
            "module amp(p, n);\nendmodule\n".to_owned(),
        )
        .expect("the edit applies");
    let evidence = component_model_evidence(&app.state.model, &component);
    assert_eq!(
        evidence.problem.as_ref().map(|(problem, _)| *problem),
        Some(Problem::Warning)
    );
    let parameters =
        painted_panel(|ui| parameters_section(ui, &mut app.design_cx(), &component, &evidence));
    assert!(parameters.contains("Recompile needed"), "{parameters}");
}
