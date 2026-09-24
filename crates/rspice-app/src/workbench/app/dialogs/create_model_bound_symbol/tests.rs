//! Create-symbol dialog tests.

use std::path::PathBuf;

use egui::{Context, Rect, vec2};

use super::controller::{
    build_create_symbol_definition, commit_create_model_bound_symbol, inferred_model_pins,
    open_create_model_bound_symbol_dialog, open_create_subcircuit_bound_symbol_dialog,
    parse_target, validate_create_symbol_draft,
};
use super::state::*;
use crate::state::{
    CellViewRef, ComponentType, Library, ModelBoundSymbolDefinition, Point, SymbolSourceContract,
    ViewType, model_library::DeviceModel, model_library::ModelType,
};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

fn render_accessibility_frame(
    app: &mut RSpiceApp,
    ctx: &Context,
    size: egui::Vec2,
) -> Vec<(egui::accesskit::NodeId, egui::accesskit::Node)> {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..egui::RawInput::default()
        },
        |ctx| app.render_create_model_bound_symbol_dialog(ctx),
    )
    .platform_output
    .accesskit_update
    .expect("create-symbol access tree")
    .nodes
}

fn node_bounds(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> egui::accesskit::Rect {
    nodes
        .iter()
        .find(|(_, node)| node.role() == role && node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| panic!("missing {role:?} node {label}"))
}

fn matching_bounds(
    nodes: &[(egui::accesskit::NodeId, egui::accesskit::Node)],
    role: egui::accesskit::Role,
    label: &str,
) -> Vec<egui::accesskit::Rect> {
    nodes
        .iter()
        .filter(|(_, node)| node.role() == role && node.label() == Some(label))
        .filter_map(|(_, node)| node.bounds())
        .collect()
}

fn rects_overlap(a: egui::accesskit::Rect, b: egui::accesskit::Rect) -> bool {
    a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1
}

fn state_with_bound_mos_model() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    state.library_manager.clear();
    state
        .library_manager
        .add_library(Library::new("analog_blocks"));
    let mut models = crate::state::model_library::ModelLibraryManager::new();
    let mut model_library = crate::state::model_library::ModelLibrary::new("vendor_cmos");
    let mut model =
        DeviceModel::new("nmos_core", ModelType::Nmos).with_geometry(45e-9, 20e-6, 90e-9, 100e-6);
    // Deliberately a Windows identity, whatever host is running: a project
    // authored there is opened on macOS and Linux, and its symbol definitions
    // are validated again on load. Replacing this with a path the running host
    // calls absolute would drop the only coverage of that move.
    model.file_path = Some(PathBuf::from(r"C:\models\cmos.lib"));
    model.add_parameter("vth0", 0.47);
    model_library.add_model(model);
    models.add_library(model_library);
    state.model_library_manager = models;
    state.workbench.selected_model = Some("nmos_core".to_owned());
    state
}

#[test]
fn target_parser_requires_one_canonical_library_cell_identity() {
    assert_eq!(parse_target("work / ota").unwrap(), ("work", "ota"));
    assert!(parse_target("work").is_err());
    assert!(parse_target("work / ota / symbol").is_err());
    assert!(parse_target("work / 12bad").is_err());
}

#[test]
fn inferred_standard_model_contracts_remain_ordered_and_review_gated() {
    let mos = inferred_model_pins(ModelType::Nmos);
    assert_eq!(
        mos.iter().map(|pin| pin.name.as_str()).collect::<Vec<_>>(),
        ["D", "G", "S", "B"]
    );
    assert_eq!(mos[1].electrical_type, CreateSymbolPinType::AnalogInput);
    assert!(inferred_model_pins(ModelType::Other).is_empty());
}

#[test]
fn opening_uses_real_writable_library_and_selected_model() {
    let mut state = AppState::default();
    state.library_manager.clear();
    state
        .library_manager
        .add_library(Library::new("analog_blocks"));
    let mut models = crate::state::model_library::ModelLibraryManager::new();
    let mut model_library = crate::state::model_library::ModelLibrary::new("vendor_analog");
    model_library.models.insert(
        "OPA189_A".to_owned(),
        DeviceModel::new("OPA189_A", ModelType::Other),
    );
    models.add_library(model_library);
    state.model_library_manager = models;
    state.workbench.selected_model = Some("OPA189_A".to_owned());

    open_create_model_bound_symbol_dialog(&mut state);

    let draft = &state.dialogs.create_model_bound_symbol;
    assert!(draft.open);
    assert_eq!(draft.target, "analog_blocks / precision_opamp");
    assert_eq!(draft.source_mode, CreateSymbolSourceMode::Model);
    assert!(
        draft.pins.is_empty(),
        "unknown macro-model terminals are never fabricated"
    );
    assert!(!draft.pin_contract_reviewed);
}

#[test]
fn commit_publishes_one_typed_revision_and_opens_the_symbol_editor() {
    let mut state = state_with_bound_mos_model();
    let initial_project_revision = state.workspace.project.revision();
    open_create_model_bound_symbol_dialog(&mut state);
    state
        .dialogs
        .create_model_bound_symbol
        .pin_contract_reviewed = true;

    commit_create_model_bound_symbol(&mut state).expect("model-bound symbol commit");

    assert!(!state.dialogs.create_model_bound_symbol.open);
    assert_eq!(state.workspace.active_view.view, "symbol");
    assert_eq!(state.workspace.active_view.library, "analog_blocks");
    assert_eq!(state.workspace.active_view.cell, "nmos_core");
    assert!(state.workspace.project.revision() > initial_project_revision);
    let cell = state
        .library_manager
        .get_library("analog_blocks")
        .and_then(|library| library.get_cell("nmos_core"))
        .expect("published cell");
    for (name, view_type) in [
        ("symbol", ViewType::Symbol),
        ("parameter_form", ViewType::Custom),
        ("spice", ViewType::Spice),
    ] {
        assert_eq!(
            cell.get_view(name).map(|view| view.view_type),
            Some(view_type)
        );
    }
    let definition =
        ModelBoundSymbolDefinition::load_from_view(cell.get_view("symbol").expect("symbol view"))
            .expect("valid definition")
            .expect("definition metadata");
    assert_eq!(definition.identity.revision, 1);
    assert_eq!(definition.netlist.device_prefix, "M");
    assert_eq!(
        definition.netlist.parameter_order,
        ["w", "l", "m", "nf", "ad", "as", "pd", "ps", "nrd", "nrs"]
    );
    assert!(
        definition.parameter_form.field("vth0").is_none(),
        "model-card coefficients are not legal per-instance overrides"
    );
    let width = definition.parameter_form.field("w").expect("width field");
    assert_eq!(
        width.constraints.minimum.as_deref(),
        Some("0.00000009000000000")
    );
    assert_eq!(
        width.constraints.maximum.as_deref(),
        Some("0.00010000000000000")
    );
}

#[test]
fn generated_fixture_is_real_editable_topology_in_the_atomic_publication() {
    let mut state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut state);
    let draft = &mut state.dialogs.create_model_bound_symbol;
    draft.pin_contract_reviewed = true;
    draft.simulation_test_fixture = true;
    draft.pins[3].electrical_type = CreateSymbolPinType::Ground;

    commit_create_model_bound_symbol(&mut state).expect("fixture publication");

    let fixture_ref = CellViewRef::new("analog_blocks", "nmos_core", "testbench");
    let fixture = state
        .workspace
        .schematic_buffers
        .get(&fixture_ref.key())
        .expect("published editable fixture");
    assert_eq!(
        fixture
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .count(),
        1
    );
    assert_eq!(
        fixture
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::Port)
            .count(),
        4
    );
    assert_eq!(
        fixture
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::Ground)
            .count(),
        1,
        "ground is generated only for the explicitly electrical-ground pin"
    );
    assert_eq!(fixture.wires.len(), 5);
    assert!(fixture.is_dirty);
    let dut = fixture
        .components
        .iter()
        .find(|component| component.kind == ComponentType::CellInstance)
        .expect("DUT");
    let binding = dut.library_cell.as_ref().expect("executable DUT binding");
    assert_eq!(binding.view, "spice");
    assert_eq!(binding.terminal_order, ["D", "G", "S", "B"]);
    assert_eq!(binding.model_section, None);
    // The published binding carries the model's own recorded identity, which
    // for this fixture is a Windows path. Judging it with `Path::is_absolute`
    // asks whether the *running* host would call it absolute, so the same
    // binding passes on Windows and fails on macOS. Absoluteness of a
    // persisted identity is a question about path syntax, not about the host
    // that happens to be asserting it.
    assert!(
        binding
            .source_path
            .as_deref()
            .is_some_and(crate::state::model_library::is_portable_absolute_path)
    );
}

#[test]
fn existing_schematic_source_is_identity_locked_and_preserves_the_master() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library = state.workspace.active_view.library.clone();
    let cell = state.workspace.active_view.cell.clone();
    let port_id = state
        .schematic
        .add_component(ComponentType::Port, Point::origin());
    let port = state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port");
    port.value = "IN".to_owned();
    state.sync_active_schematic_to_workspace();

    open_create_model_bound_symbol_dialog(&mut state);
    state
        .dialogs
        .create_model_bound_symbol
        .select_source(CreateSymbolSourceMode::ExistingSchematicPins);
    assert_eq!(
        state.dialogs.create_model_bound_symbol.target,
        format!("{library} / {cell}")
    );
    let definition = build_create_symbol_definition(&state).expect("schematic definition");
    assert!(matches!(
        definition.source,
        SymbolSourceContract::ExistingSchematicPins { .. }
    ));
    assert_eq!(definition.netlist.model, None);
    assert_eq!(definition.netlist.device_prefix, "X");

    state.dialogs.create_model_bound_symbol.target = format!("{library} / different_cell");
    assert!(
        validate_create_symbol_draft(&state)
            .expect_err("cross-cell identity must fail")
            .contains("exact target cell")
    );
}

#[test]
fn blank_contract_publishes_truthful_review_views_but_never_a_fixture() {
    let mut state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut state);
    {
        let draft = &mut state.dialogs.create_model_bound_symbol;
        draft.select_source(CreateSymbolSourceMode::BlankExplicitContract);
        draft.pins.push(CreateSymbolPinDraft::new(
            "IO",
            CreateSymbolPinType::AnalogBidirectional,
            CreateSymbolPinSide::Left,
        ));
        draft.symbol = false;
        draft.parameter_form = true;
    }
    let definition = build_create_symbol_definition(&state).expect("review definition");
    assert!(matches!(
        definition.source,
        SymbolSourceContract::BlankExplicitContract
    ));
    assert!(!definition.netlist.is_executable());
    assert!(!definition.generated_views.simulation_test_fixture);

    commit_create_model_bound_symbol(&mut state).expect("publish unbound review symbol");
    assert_eq!(state.workspace.active_view.view, "parameter_form");
    let cell = state
        .library_manager
        .get_library("analog_blocks")
        .and_then(|library| library.get_cell("nmos_core"))
        .expect("review cell");
    assert!(cell.get_view("symbol").is_none());
    assert!(cell.get_view("parameter_form").is_some());
    assert!(cell.get_view("testbench").is_none());
    assert!(cell.get_view("spice").is_none());
}

#[test]
fn accessibility_tree_exposes_the_mockup_dialog_and_complete_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);
    app.state
        .dialogs
        .create_model_bound_symbol
        .pin_contract_reviewed = true;
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1280.0, 900.0))),
            ..egui::RawInput::default()
        },
        |ctx| app.render_create_model_bound_symbol_dialog(ctx),
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("create-symbol access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
    }));
    for label in [PRIMARY, "Cancel", "Add pin", "Remove pin"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
}

#[test]
fn shipped_subcircuit_opens_an_exact_x_device_pin_contract() {
    let mut state = state_with_bound_mos_model();
    let source = std::env::current_dir()
        .expect("current directory")
        .join("test-fixtures/logic-family-models.lib");
    open_create_subcircuit_bound_symbol_dialog(
        &mut state,
        "pack-fixture-logic-family".to_owned(),
        "7400".to_owned(),
        source,
        vec![
            "A".to_owned(),
            "B".to_owned(),
            "Y".to_owned(),
            "VCC".to_owned(),
            "GND".to_owned(),
        ],
        Some("TT".to_owned()),
        std::collections::BTreeMap::from([
            ("GAIN".to_owned(), "100".to_owned()),
            ("MODE".to_owned(), "\"low noise\"".to_owned()),
            ("SCALE".to_owned(), "{GAIN * 2}".to_owned()),
        ]),
    )
    .expect("subcircuit draft opens");

    let draft = &mut state.dialogs.create_model_bound_symbol;
    assert!(draft.open);
    assert_eq!(
        draft
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["A", "B", "Y", "VCC", "GND"]
    );
    assert!(!draft.pin_contract_reviewed);
    draft.pin_contract_reviewed = true;
    let definition = build_create_symbol_definition(&state).expect("definition builds");
    assert_eq!(definition.netlist.device_prefix, "X");
    assert_eq!(
        definition
            .netlist
            .model
            .as_ref()
            .map(|model| model.model.as_str()),
        Some("7400")
    );
    assert_eq!(
        definition
            .netlist
            .model
            .as_ref()
            .and_then(|model| model.section.as_deref()),
        Some("TT")
    );
    let fields = &definition.parameter_form.sections[0].fields;
    assert_eq!(
        fields
            .iter()
            .map(|field| (field.key.as_str(), field.property_type))
            .collect::<Vec<_>>(),
        [
            ("GAIN", crate::state::PropertyType::Number),
            ("MODE", crate::state::PropertyType::String),
            ("SCALE", crate::state::PropertyType::Expression),
        ]
    );
    assert!(matches!(
        fields[2].default,
        crate::state::SymbolParameterDefault::Expression { ref value }
            if value == "GAIN * 2"
    ));
}

#[test]
fn mockup_width_keeps_pin_actions_and_generated_views_in_separate_tracks() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);

    // A warm-up frame lets the dialog settle its measured body height before
    // the production-sized frame is inspected.
    let size = vec2(772.0, 477.0);
    let _ = render_accessibility_frame(&mut app, &ctx, size);
    let nodes = render_accessibility_frame(&mut app, &ctx, size);
    let dialog = node_bounds(&nodes, egui::accesskit::Role::Dialog, TITLE);

    let earlier = matching_bounds(
        &nodes,
        egui::accesskit::Role::Button,
        "Move pin earlier in netlist order",
    );
    let later = matching_bounds(
        &nodes,
        egui::accesskit::Role::Button,
        "Move pin later in netlist order",
    );
    assert_eq!(
        earlier.len(),
        app.state.dialogs.create_model_bound_symbol.pins.len()
    );
    assert_eq!(later.len(), earlier.len());
    for (up, down) in earlier.into_iter().zip(later) {
        assert!((up.x1 - up.x0 - 24.0).abs() <= 0.5);
        assert!((down.x1 - down.x0 - 24.0).abs() <= 0.5);
        assert!(!rects_overlap(up, down));
        assert!(up.x0 >= dialog.x0 && down.x1 <= dialog.x1);
    }

    let generated = ["symbol", "parameter form", "simulation test fixture"]
        .map(|label| node_bounds(&nodes, egui::accesskit::Role::CheckBox, label));
    assert!(!rects_overlap(generated[0], generated[1]));
    assert!(!rects_overlap(generated[1], generated[2]));
    assert!(generated[0].x0 >= dialog.x0);
    assert!(generated[2].x1 <= dialog.x1);
}

#[test]
fn validation_status_reserves_height_and_desktop_generated_views_do_not_clip() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut app.state);

    let size = vec2(1440.0, 900.0);
    let _ = render_accessibility_frame(&mut app, &ctx, size);
    let invalid_nodes = render_accessibility_frame(&mut app, &ctx, size);
    let invalid_add = node_bounds(&invalid_nodes, egui::accesskit::Role::Button, "Add pin");

    app.state
        .dialogs
        .create_model_bound_symbol
        .pin_contract_reviewed = true;
    let valid_nodes = render_accessibility_frame(&mut app, &ctx, size);
    let valid_add = node_bounds(&valid_nodes, egui::accesskit::Role::Button, "Add pin");
    assert!(
        (invalid_add.y0 - valid_add.y0).abs() <= 1.0,
        "validation status changed body placement: invalid={invalid_add:?}, valid={valid_add:?}"
    );

    let dialog = node_bounds(&valid_nodes, egui::accesskit::Role::Dialog, TITLE);
    let generated = ["symbol", "parameter form", "simulation test fixture"]
        .map(|label| node_bounds(&valid_nodes, egui::accesskit::Role::CheckBox, label));
    assert!(!rects_overlap(generated[0], generated[1]));
    assert!(!rects_overlap(generated[1], generated[2]));
    assert!(generated[0].x0 >= dialog.x0);
    assert!(generated[2].x1 <= dialog.x1);
}
