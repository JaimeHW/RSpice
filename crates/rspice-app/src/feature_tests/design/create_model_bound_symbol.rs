//! Creating a model-bound symbol through the whole application: the dialog reads the
//! models slot's library, and its commit publishes into an open project session.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::controller::open_create_subcircuit_bound_symbol_dialog;
use rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::state::*;
use rspice_app_state::state::{
    CellViewRef, ComponentType, Library, Point, SymbolSourceContract, ViewType,
    model_library::{DeviceModel, ModelType},
};
use rspice_app_workbench::workbench::framework::requests::OpenInDesign;
use std::path::PathBuf;

/// Draw the dialog as the frame's step does, over the application's parts.
/// Open Create symbol over `state`'s slots, from the model the models
/// workspace has selected.
pub(crate) fn open_create_model_bound_symbol_dialog(state: &mut AppState) {
    rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::controller::open_create_model_bound_symbol_dialog(
        &mut state.design.create_model_bound_symbol,
        &state.model,
        &mut state.workbench,
        state.models.selected_model.as_deref(),
    );
}

/// Check the draft as the dialog does, over the whole state.
fn validate_create_symbol_draft(state: &AppState) -> Result<(), String> {
    rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::controller::validate_create_symbol_draft(
        &state.design.create_model_bound_symbol,
        &state.model,
    )
}

/// The definition the draft describes, over the whole state.
fn build_create_symbol_definition(
    state: &AppState,
) -> Result<rspice_app_state::state::ModelBoundSymbolDefinition, String> {
    rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::controller::build_symbol_definition(
        &state.model,
        &state.design.create_model_bound_symbol,
    )
}

/// Create the symbol as the dialog's primary does, then open what it created
/// as the shell answers the dialog's request.
fn commit_create_model_bound_symbol(state: &mut AppState) -> Result<(), String> {
    rspice_app_design::workbench::app::dialogs::create_model_bound_symbol::controller::commit_create_model_bound_symbol(
        &mut state.design.create_model_bound_symbol,
        &mut state.model,
        &mut state.workbench,
    )?;
    let OpenInDesign(reference) = state
        .workbench
        .outbox
        .take()
        .expect("the created view is asked for");
    state.open_workspace_view(reference);
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    Ok(())
}

pub(crate) fn state_with_bound_mos_model() -> AppState {
    let mut state = AppState::default();
    state.open_project_session();
    state.model.library_manager.clear();
    state
        .model
        .library_manager
        .add_library(Library::new("analog_blocks"));
    let mut models = rspice_app_state::state::model_library::ModelLibraryManager::new();
    let mut model_library =
        rspice_app_state::state::model_library::ModelLibrary::new("vendor_cmos");
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
    state.model.model_library_manager = models;
    state.models.selected_model = Some("nmos_core".to_owned());
    state
}

#[test]
fn opening_uses_real_writable_library_and_selected_model() {
    let mut state = AppState::default();
    state.model.library_manager.clear();
    state
        .model
        .library_manager
        .add_library(Library::new("analog_blocks"));
    let mut models = rspice_app_state::state::model_library::ModelLibraryManager::new();
    let mut model_library =
        rspice_app_state::state::model_library::ModelLibrary::new("vendor_analog");
    model_library.models.insert(
        "OPA189_A".to_owned(),
        DeviceModel::new("OPA189_A", ModelType::Other),
    );
    models.add_library(model_library);
    state.model.model_library_manager = models;
    state.models.selected_model = Some("OPA189_A".to_owned());

    open_create_model_bound_symbol_dialog(&mut state);

    let draft = &state.design.create_model_bound_symbol;
    assert!(draft.open);
    assert_eq!(draft.target, "analog_blocks / opa189_a");
    assert_eq!(draft.source_mode, CreateSymbolSourceMode::Model);
    // A model card is a primitive; its name never makes it an amplifier.
    assert_eq!(draft.template, CreateSymbolTemplate::RectangularIc);
    assert!(
        draft.pins.is_empty(),
        "unknown macro-model terminals are never fabricated"
    );
    assert!(!draft.pin_contract_reviewed);
}

#[test]
fn commit_publishes_one_typed_revision_and_opens_the_symbol_editor() {
    let mut state = state_with_bound_mos_model();
    let initial_project_revision = state.model.workspace.content.project.revision();
    open_create_model_bound_symbol_dialog(&mut state);
    state.design.create_model_bound_symbol.pin_contract_reviewed = true;

    commit_create_model_bound_symbol(&mut state).expect("model-bound symbol commit");

    assert!(!state.design.create_model_bound_symbol.open);
    assert_eq!(state.model.workspace.content.active_view.view, "symbol");
    assert_eq!(
        state.model.workspace.content.active_view.library,
        "analog_blocks"
    );
    assert_eq!(state.model.workspace.content.active_view.cell, "nmos_core");
    assert!(state.model.workspace.content.project.revision() > initial_project_revision);
    let cell = state
        .model
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
    let definition = rspice_app_state::state::load_model_bound_symbol(
        cell.get_view("symbol").expect("symbol view"),
    )
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
    let draft = &mut state.design.create_model_bound_symbol;
    draft.pin_contract_reviewed = true;
    draft.simulation_test_fixture = true;
    draft.pins[3].electrical_type = CreateSymbolPinType::Ground;

    commit_create_model_bound_symbol(&mut state).expect("fixture publication");

    let fixture_ref = CellViewRef::new("analog_blocks", "nmos_core", "testbench");
    let fixture = state
        .model
        .workspace
        .schematic_editor(&fixture_ref.key())
        .expect("published editable fixture");
    assert_eq!(
        fixture
            .document()
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
            .count(),
        1
    );
    assert_eq!(
        fixture
            .document()
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::Port)
            .count(),
        4
    );
    assert_eq!(
        fixture
            .document()
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::Ground)
            .count(),
        1,
        "ground is generated only for the explicitly electrical-ground pin"
    );
    assert_eq!(fixture.document().wires.len(), 5);
    assert!(fixture.is_dirty());
    let dut = fixture
        .document()
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
            .is_some_and(rspice_app_state::state::model_library::is_portable_absolute_path)
    );
}

#[test]
fn existing_schematic_source_is_identity_locked_and_preserves_the_master() {
    let mut state = AppState::default();
    state.open_project_session();
    let library = state.model.workspace.content.active_view.library.clone();
    let cell = state.model.workspace.content.active_view.cell.clone();
    let port_id = state
        .model
        .schematic
        .add_component(ComponentType::Port, Point::origin());
    let port = state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port_id)
        .expect("port");
    port.value = "IN".to_owned();
    state.sync_active_schematic_to_workspace();

    open_create_model_bound_symbol_dialog(&mut state);
    state
        .design
        .create_model_bound_symbol
        .select_source(CreateSymbolSourceMode::ExistingSchematicPins);
    assert_eq!(
        state.design.create_model_bound_symbol.target,
        format!("{library} / {cell}")
    );
    let definition = build_create_symbol_definition(&state).expect("schematic definition");
    assert!(matches!(
        definition.source,
        SymbolSourceContract::ExistingSchematicPins { .. }
    ));
    assert_eq!(definition.netlist.model, None);
    assert_eq!(definition.netlist.device_prefix, "X");

    state.design.create_model_bound_symbol.target = format!("{library} / different_cell");
    assert!(
        validate_create_symbol_draft(&state)
            .expect_err("cross-cell identity must fail")
            .contains("Use that cell as the target")
    );
}

#[test]
fn blank_contract_publishes_truthful_review_views_but_never_a_fixture() {
    let mut state = state_with_bound_mos_model();
    open_create_model_bound_symbol_dialog(&mut state);
    {
        let draft = &mut state.design.create_model_bound_symbol;
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
    assert_eq!(
        state.model.workspace.content.active_view.view,
        "parameter_form"
    );
    let cell = state
        .model
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
fn shipped_subcircuit_opens_an_exact_x_device_pin_contract() {
    let mut state = state_with_bound_mos_model();
    let source = std::env::current_dir()
        .expect("current directory")
        .join("test-fixtures/logic-family-models.lib");
    open_create_subcircuit_bound_symbol_dialog(
        &mut state.design.create_model_bound_symbol,
        &state.model,
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
        None,
        Some("TT".to_owned()),
        std::collections::BTreeMap::from([
            ("GAIN".to_owned(), "100".to_owned()),
            ("MODE".to_owned(), "\"low noise\"".to_owned()),
            ("SCALE".to_owned(), "{GAIN * 2}".to_owned()),
        ]),
    )
    .expect("subcircuit draft opens");

    let draft = &mut state.design.create_model_bound_symbol;
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
            ("GAIN", rspice_app_state::state::PropertyType::Number),
            ("MODE", rspice_app_state::state::PropertyType::String),
            ("SCALE", rspice_app_state::state::PropertyType::Expression),
        ]
    );
    assert!(matches!(
        fields[2].default,
        rspice_app_state::state::SymbolParameterDefault::Expression { ref value }
            if value == "GAIN * 2"
    ));
}

/// A vendor op-amp names its pins by number and lists them in the comment
/// above the definition; the draft reads the roles from that list.
#[test]
fn a_vendor_amplifier_subcircuit_opens_with_the_amplifier_template() {
    let mut state = state_with_bound_mos_model();
    open_create_subcircuit_bound_symbol_dialog(
        &mut state.design.create_model_bound_symbol,
        &state.model,
        "vendor_analog".to_owned(),
        "OPA_SYNTH".to_owned(),
        PathBuf::from("/models/opa_synth.lib"),
        ["1", "2", "3", "4", "5"].map(str::to_owned).to_vec(),
        Some("Synthetic precision amplifier. Pin order: IN+, IN-, V+, V-, OUT".to_owned()),
        None,
        std::collections::BTreeMap::new(),
    )
    .expect("subcircuit draft opens");

    let draft = &state.design.create_model_bound_symbol;
    assert_eq!(
        draft.template,
        CreateSymbolTemplate::OperationalAmplifier5Pin
    );
    assert_eq!(
        draft
            .pins
            .iter()
            .map(|pin| (pin.name.as_str(), pin.electrical_type, pin.side))
            .collect::<Vec<_>>(),
        [
            (
                "1",
                CreateSymbolPinType::AnalogInput,
                CreateSymbolPinSide::Left
            ),
            (
                "2",
                CreateSymbolPinType::AnalogInput,
                CreateSymbolPinSide::Left
            ),
            ("3", CreateSymbolPinType::Power, CreateSymbolPinSide::Top),
            ("4", CreateSymbolPinType::Power, CreateSymbolPinSide::Bottom),
            (
                "5",
                CreateSymbolPinType::AnalogOutput,
                CreateSymbolPinSide::Right
            ),
        ]
    );
}
