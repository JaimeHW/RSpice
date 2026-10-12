//! A compiled module's parameters become an instance's typed properties.
//!
//! These pins compile real modules and publish them through the frame loop's
//! own poll, then read what the library, the property sheet, the design check
//! and the deck say about an instance of the module.

use super::veriloga_design_tests::{DESKTOP, open_in_design, open_project_app};
use super::veriloga_interface_tests::{
    add_module_cell, compile_and_publish, configured_deck, edit_source,
    ensure_veriloga_configuration, view,
};
use super::*;
use crate::state::{
    CellViewRef, Point, PropertySheet, PropertyType, PropertyValue, View, ViewType,
};
use rspice_design::library::metadata::{
    CellParameter, ParameterDeclaration, ParameterRange, ParameterScope, ParameterValueType,
    VERILOGA_PARAMETER_CONTRACT_KEY, cell_parameter_contract,
};

const LIBRARY: &str = "behavioral";

/// Module `name`, a conductance of `gain`, declaring `declarations`.
fn module(name: &str, declarations: &str) -> String {
    format!(
        "module {name}(p, n);\n  inout p, n;\n  electrical p, n;\n{declarations}  analog I(p, n) <+ gain * V(p, n);\nendmodule\n"
    )
}

/// One parameter of each kind the property sheet handles: a described,
/// unit-bearing real with an open lower end, a closed upper end and an
/// excluded value; an integer; one whose default is computed; and one whose
/// bound depends on another parameter.
const KINDS: &str = "  (* desc=\"Small-signal gain\", units=\"S\", type=\"instance\" *)\n  parameter real gain = 2.5 from (0:10] exclude 5;\n  parameter integer stages = 3 from [1:8];\n  parameter real vmax = 0.5*stages;\n  parameter real lim = 1 from (0:vmax];\n";

/// The contract `reference`'s view states, read the way every reader reads it.
fn stated_contract(app: &RSpiceApp, reference: &CellViewRef) -> Vec<CellParameter> {
    let cell = app
        .state
        .library_manager
        .get_library(&reference.library)
        .and_then(|library| library.get_cell(&reference.cell))
        .expect("the cell");
    cell_parameter_contract(cell, cell.get_view(&reference.view)).expect("a valid contract")
}

fn declaration(parameter: &CellParameter) -> &ParameterDeclaration {
    parameter
        .declaration
        .as_ref()
        .unwrap_or_else(|| panic!("{} carries its declaration", parameter.name))
}

fn range_text(parameter: &CellParameter) -> Option<String> {
    declaration(parameter)
        .range
        .as_ref()
        .map(ParameterRange::source_text)
}

/// Place `reference` on the top schematic as the library shelf does, binding
/// the allowlist its contract states now, under a configuration that executes
/// Verilog-A views.
fn place_from_shelf(state: &mut AppState, reference: &CellViewRef) -> u64 {
    let candidate =
        crate::state::library_cell_placement_candidates(&state.library_manager, &state.workspace)
            .into_iter()
            .find(|candidate| {
                candidate.library == reference.library
                    && candidate.cell == reference.cell
                    && candidate.view == reference.view
            })
            .expect("the view is on the shelf");
    assert!(candidate.ready, "{}", candidate.unavailable_reason);
    state.open_workspace_view(CellViewRef::default_top());
    let id = state
        .schematic
        .add_library_cell_component(Point::new(20, 20), candidate.binding);
    ensure_veriloga_configuration(state);
    id
}

fn placed(state: &AppState, id: u64) -> crate::state::Component {
    state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .cloned()
        .expect("the placed instance")
}

fn set_params(state: &mut AppState, id: u64, params: &str) {
    state
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed instance")
        .params = params.to_owned();
}

fn sheet_of(state: &AppState, id: u64) -> PropertySheet {
    crate::workbench::app::authoritative_component_property_sheet(state, &placed(state, id))
        .expect("the sheet resolves")
        .expect("an instance of a compiled module has a sheet")
}

/// A compile states the module's parameters on its view with everything a
/// property editor needs; a recompile that changes nothing writes nothing and
/// leaves the project clean.
#[test]
fn a_clean_compile_states_the_modules_parameters_and_an_unchanged_one_writes_nothing() {
    let mut app = open_project_app();
    let reference = add_module_cell(&mut app.state, "amp", "amp", &module("amp", KINDS));
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);

    let stated = stated_contract(&app, &reference);
    assert_eq!(
        stated
            .iter()
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        ["gain", "stages", "vmax", "lim"]
    );
    let gain = declaration(&stated[0]);
    assert_eq!(
        (
            gain.value_type,
            gain.scope,
            gain.units.as_deref(),
            gain.description.as_deref()
        ),
        (
            ParameterValueType::Real,
            ParameterScope::Instance,
            Some("S"),
            Some("Small-signal gain")
        )
    );
    assert_eq!(stated[0].default_value.as_deref(), Some("2.5"));
    assert_eq!(
        range_text(&stated[0]).as_deref(),
        Some("from (0:10] exclude 5")
    );
    assert_eq!(
        declaration(&stated[1]).value_type,
        ParameterValueType::Integer
    );
    assert_eq!(stated[1].default_value.as_deref(), Some("3"));
    assert_eq!(
        declaration(&stated[1]).scope,
        ParameterScope::Model,
        "an undeclared scope reads as the module's"
    );
    assert_eq!(
        stated[2].default_value, None,
        "vmax is computed from stages"
    );
    assert_eq!(range_text(&stated[3]).as_deref(), Some("from (0:vmax]"));
    assert!(app.state.workspace.content.project_metadata_dirty);

    app.state.workspace.content.project_metadata_dirty = false;
    let revision = app.state.library_manager.revision();
    let before = view(&app, &reference, "veriloga").map(|view| view.metadata.clone());
    compile_and_publish(&mut app);
    assert_eq!(
        view(&app, &reference, "veriloga").map(|view| view.metadata.clone()),
        before
    );
    assert_eq!(app.state.library_manager.revision(), revision);
    assert!(
        !app.state.workspace.content.project_metadata_dirty,
        "an unchanged recompile dirties nothing"
    );
}

/// A read-only library is never written, whether the module defines the
/// cell's ports or a schematic does; a writable one whose schematic agrees
/// with the module still takes the module's parameters.
#[test]
fn a_read_only_library_is_never_given_a_parameter_contract() {
    let contract_of = |read_only: bool, with_schematic: bool| {
        let mut app = open_project_app();
        let reference = add_module_cell(&mut app.state, "amp", "amp", &module("amp", KINDS));
        if with_schematic {
            app.state
                .library_manager
                .get_library_mut(LIBRARY)
                .and_then(|library| library.get_cell_mut("amp"))
                .expect("the cell")
                .add_view(View::new("schematic", ViewType::Schematic));
            let mut master = crate::state::SchematicState::default();
            for (index, name) in ["p", "n"].into_iter().enumerate() {
                let id = master.add_component(
                    crate::state::ComponentType::Port,
                    Point::new(i32::try_from(index).unwrap_or(0) * 40, 0),
                );
                let port = master
                    .document_mut_for_test()
                    .components
                    .iter_mut()
                    .find(|component| component.id == id)
                    .expect("the port");
                port.value = name.to_owned();
                port.params = "dir=inout".to_owned();
            }
            app.state.workspace.insert_schematic_editor(
                CellViewRef::new(LIBRARY, "amp", "schematic").key(),
                master,
            );
        }
        app.state
            .library_manager
            .get_library_mut(LIBRARY)
            .expect("the library")
            .read_only = read_only;
        open_in_design(&mut app.state, &reference);
        compile_and_publish(&mut app);
        view(&app, &reference, "veriloga")
            .and_then(|view| view.metadata.get(VERILOGA_PARAMETER_CONTRACT_KEY))
            .cloned()
    };
    assert_eq!(contract_of(true, false), None);
    assert_eq!(contract_of(true, true), None);
    assert!(
        contract_of(false, true).is_some(),
        "a module whose ports agree with the schematic still states its parameters"
    );
}

/// The parameters an instance's property sheet offers are the allowlist the
/// configured deck binds, both read from the view's one contract.
#[test]
fn the_property_sheet_and_configured_materialization_read_one_allowlist() {
    let mut app = open_project_app();
    let reference = add_module_cell(&mut app.state, "amp", "amp", &module("amp", KINDS));
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let id = place_from_shelf(&mut app.state, &reference);

    let sheet = sheet_of(&app.state, id);
    let offered = sheet
        .iter()
        .map(|definition| definition.name.clone())
        .filter(|name| name != "name" && name != "m")
        .collect::<Vec<_>>();

    let state = &app.state;
    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .expect("the configured design resolves");
    let materialized = projection
        .plan()
        .bindings()
        .find_map(|execution| {
            execution
                .materialized_binding()
                .filter(|binding| binding.cell == "amp")
        })
        .expect("the instance is materialized");
    assert!(materialized.parameters_declared());
    assert_eq!(
        materialized
            .parameter_order
            .iter()
            .map(|name| name.to_ascii_lowercase())
            .collect::<Vec<_>>(),
        offered
    );
    assert_eq!(offered, ["gain", "stages", "vmax", "lim"]);
}

/// An instance placed before its module gained a parameter takes the new one
/// once the module recompiles, while a name the module never declared is
/// still refused.
#[test]
fn an_instance_placed_before_its_module_gained_a_parameter_takes_it() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "amp",
        "amp",
        &module("amp", "  parameter real gain = 1m;\n"),
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let id = place_from_shelf(&mut app.state, &reference);
    assert_eq!(
        placed(&app.state, id)
            .library_cell
            .map(|binding| binding.parameter_order),
        Some(vec!["gain".to_owned()]),
        "the placement took the contract as it stood"
    );

    open_in_design(&mut app.state, &reference);
    edit_source(
        &mut app,
        &module(
            "amp",
            "  parameter real gain = 1m;\n  parameter real offset = 0;\n",
        ),
    );
    compile_and_publish(&mut app);
    app.state.open_workspace_view(CellViewRef::default_top());

    assert!(sheet_of(&app.state, id).get("offset").is_some());
    set_params(&mut app.state, id, "offset=1m");
    let deck = configured_deck(&app).expect("the new parameter netlists");
    assert!(
        deck.lines().any(|line| line.contains("offset=1m")),
        "{deck}"
    );

    set_params(&mut app.state, id, "bogus=1");
    let refusal = configured_deck(&app).expect_err("an undeclared name is refused");
    assert!(refusal.contains("bogus"), "{refusal}");
}

/// Each parameter is a field typed by its declaration: a real with its unit
/// and its range's open and closed ends, an integer that refuses a fraction,
/// an excluded value refused by name, a bound that depends on another
/// parameter stated rather than enforced, and a computed default that says so.
#[test]
fn each_module_parameter_is_a_typed_range_checked_property() {
    let mut app = open_project_app();
    let reference = add_module_cell(&mut app.state, "amp", "amp", &module("amp", KINDS));
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let id = place_from_shelf(&mut app.state, &reference);
    let sheet = sheet_of(&app.state, id);
    let field = |name: &str| {
        sheet
            .get(name)
            .unwrap_or_else(|| panic!("a {name} field"))
            .clone()
    };
    let number = PropertyValue::number;

    let gain = field("gain");
    assert_eq!(gain.prop_type, PropertyType::Number);
    assert_eq!(gain.unit.as_deref(), Some("S"));
    assert_eq!(gain.category, "Instance parameters");
    assert_eq!(gain.default_value, number(2.5));
    assert!(
        gain.validate(&number(0.0))
            .is_err_and(|error| error.contains("greater than 0")),
        "the open lower end refuses 0"
    );
    assert!(gain.validate(&number(10.0)).is_ok(), "the closed upper end");
    assert!(gain.validate(&number(10.5)).is_err());
    assert_eq!(
        gain.validate(&number(5.0)),
        Err("gain cannot be 5: that value is excluded".to_owned())
    );

    let stages = field("stages");
    assert_eq!(stages.prop_type, PropertyType::Number);
    assert_eq!(stages.category, "Model parameters");
    assert_eq!(
        stages.validate(&number(1.5)),
        Err("stages must be a whole number".to_owned())
    );
    assert!(stages.validate(&number(3.0)).is_ok());

    let lim = field("lim");
    assert_eq!(
        lim.max_value, None,
        "a dependent bound is the module's to check"
    );
    assert!(lim.validate(&number(1e9)).is_ok());
    assert!(
        lim.validate(&number(0.0)).is_err(),
        "its constant end is not"
    );
    assert!(
        lim.description.contains("from (0:vmax]"),
        "{}",
        lim.description
    );

    let vmax = field("vmax");
    assert_eq!(vmax.prop_type, PropertyType::Expression);
    assert_eq!(vmax.default_value, PropertyValue::expression(""));
    assert_eq!(vmax.default_hint.as_deref(), Some("computed"));

    // The component editor and the design check judge typed values by these
    // fields too.
    let mut component = placed(&app.state, id);
    component.params = "gain=5 stages=1.5".to_owned();
    let refusals =
        crate::properties::property_bridge::validate_component_properties(&component, Some(&sheet));
    assert!(
        refusals
            .iter()
            .any(|(field, error)| field == "gain" && error.contains("cannot be 5")),
        "{refusals:?}"
    );
    assert!(
        refusals
            .iter()
            .any(|(field, error)| field == "stages" && error.contains("whole number")),
        "{refusals:?}"
    );
}

/// A value an instance sets for a parameter its module has since dropped is a
/// design-check finding and never reaches the deck; removing it lets the run
/// netlist. A module that drops its last parameter accepts none.
#[test]
fn a_parameter_the_module_dropped_is_found_refused_and_removable() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "amp",
        "amp",
        &module(
            "amp",
            "  parameter real gain = 1m;\n  parameter real offset = 0;\n",
        ),
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let id = place_from_shelf(&mut app.state, &reference);
    set_params(&mut app.state, id, "gain=2m offset=1m");
    assert!(configured_deck(&app).is_ok());

    open_in_design(&mut app.state, &reference);
    edit_source(&mut app, &module("amp", "  parameter real gain = 1m;\n"));
    compile_and_publish(&mut app);
    app.state.open_workspace_view(CellViewRef::default_top());

    assert_eq!(
        crate::workbench::app::undeclared_instance_parameters(&app.state, &placed(&app.state, id)),
        [("offset".to_owned(), "1m".to_owned())]
    );
    let report = super::dialogs::check_and_save_validation::CheckAndSaveValidationReport::collect(
        &app.state,
    )
    .expect("the design check runs");
    assert!(
        report.blockers().iter().any(|finding| {
            finding.message.contains("offset=1m")
                && finding.message.contains("does not declare")
                && finding.message.contains("Remove")
        }),
        "{:?}",
        report.blockers()
    );
    let refusal = configured_deck(&app).expect_err("the stale value is never emitted");
    assert!(
        refusal.contains("parameter `offset` is not declared"),
        "{refusal}"
    );

    assert_eq!(
        app.state.remove_undeclared_instance_parameter(id, "offset"),
        Ok(true)
    );
    assert_eq!(placed(&app.state, id).params, "gain=2m");
    let deck = configured_deck(&app).expect("the run netlists once it is removed");
    assert!(deck.lines().any(|line| line.contains("gain=2m")), "{deck}");

    open_in_design(&mut app.state, &reference);
    edit_source(
        &mut app,
        "module amp(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ 1m * V(p, n);\nendmodule\n",
    );
    compile_and_publish(&mut app);
    app.state.open_workspace_view(CellViewRef::default_top());
    let refusal = configured_deck(&app)
        .expect_err("a module with no parameters accepts none, rather than passing them through");
    assert!(
        refusal.contains("parameter `gain` is not declared"),
        "{refusal}"
    );
}

/// Renders for review: the instance inspector for a device with several kinds
/// of parameter, the same instance carrying a value its module dropped, and
/// the Verilog-A tab inspector's Parameters section.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_veriloga_parameters_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let mut write = |name: &str, canvas: &crate::ui::raster::Canvas, height: usize| {
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "gain_cell",
        "gain_cell",
        &module(
            "gain_cell",
            &format!("{KINDS}  (* units=\"Ohm\" *) parameter real rload = 1e3 from [0:inf);\n"),
        ),
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| app.render_frame_chrome(ui));
    write(
        "veriloga-tab-parameters-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    let id = place_from_shelf(&mut app.state, &reference);
    set_params(&mut app.state, id, "gain=3 stages=4");
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    app.state.workbench.inspector_visible = true;
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| app.render_frame_chrome(ui));
    write(
        "veriloga-instance-parameters-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    set_params(&mut app.state, id, "gain=3 stages=4 offset=1m");
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| app.render_frame_chrome(ui));
    write(
        "veriloga-instance-undeclared-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    // The same rows where the schematic cannot be edited.
    app.state.schematic.session.read_only = true;
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| app.render_frame_chrome(ui));
    write(
        "veriloga-instance-read-only-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );
}
