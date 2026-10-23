//! The inspector's account of a Verilog-A module instance: the parameter rows
//! its compile declares, its multiplicity and temperature, a value the module
//! no longer declares, and the identity it states instead of model-library
//! rows.

use super::*;

/// A compiled module parameter, as a clean compile states it.
fn module_parameter(
    name: &str,
    default: Option<&str>,
    units: Option<&str>,
    description: Option<&str>,
) -> rspice_design::library::metadata::CellParameter {
    use rspice_design::library::metadata::{
        CellParameter, ParameterDeclaration, ParameterScope, ParameterValueType,
    };
    CellParameter {
        name: name.to_owned(),
        aliases: Vec::new(),
        required: false,
        default_value: default.map(str::to_owned),
        declaration: Some(ParameterDeclaration {
            value_type: ParameterValueType::Real,
            scope: ParameterScope::Instance,
            range: None,
            units: units.map(str::to_owned),
            description: description.map(str::to_owned),
        }),
    }
}

/// An instance `X1` setting `params` of `behavioral/amp`, whose `veriloga`
/// view's compile stated `contract`.
fn app_with_compiled_veriloga_instance(
    contract: &[rspice_design::library::metadata::CellParameter],
    params: &str,
) -> (RSpiceApp, Component) {
    let mut app = RSpiceApp::test_instance();
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "amp".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    view.metadata.insert(
        rspice_design::library::metadata::VERILOGA_PARAMETER_CONTRACT_KEY.to_owned(),
        rspice_design::library::metadata::encode_veriloga_parameter_contract(contract)
            .expect("a compiled contract encodes"),
    );
    let mut cell = Cell::new("amp");
    cell.add_view(view);
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    let mut binding = LibraryCellInstance::new("behavioral", "amp", "veriloga");
    binding.terminal_order = vec!["p".to_owned(), "n".to_owned()];
    let mut component = Component::new(51, ComponentType::CellInstance, Point::origin())
        .with_library_cell(binding)
        .with_name_value("X1", "amp");
    component.params = params.to_owned();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component.clone());
    app.state.model.schematic.init_undo_history();
    (app, component)
}

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
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
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

    let evidence = super::super::component_model_evidence(&app.state, &component);
    let painted = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        painted.lines().any(|line| line == "S") && painted.lines().any(|line| line == "V"),
        "each unit is painted beside its field:\n{painted}"
    );
    let nodes = accesskit_nodes(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        nodes.iter().any(|(_, node)| {
            node.label() == Some("gain")
                && node
                    .description()
                    .is_some_and(|description| description.starts_with("Transconductance."))
        }),
        "the field is described by the module's own words"
    );

    let capacitor = Component::new(9, ComponentType::Capacitor, Point::origin());
    let rows = inline_parameter_contract(&app.state, &capacitor, &HashMap::new());
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

/// `m` is a module's `$mfactor`: a module instance keeps its multiplicity
/// row, and an edit there sets the typed multiplicity the deck emits, never
/// parameter text the netlister would refuse.
#[test]
fn m_on_a_module_instance_is_its_typed_multiplicity() {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    assert!(rows.iter().any(|row| row.key == "m"), "{rows:?}");

    let field = InlineEditField::Parameter("m".to_owned());
    assert!(begin_edit(&mut app, component.id, field.clone()));
    update_edit(&mut app, "2".to_owned());
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

/// A module reads `$temperature`, so an instance of one is offered `temp`
/// only as a parameter its module declares, as that parameter's own row.
#[test]
fn temp_is_offered_on_a_module_instance_only_as_a_declared_parameter() {
    let (app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    assert!(!supports_instance_temperature(&app.state, &component));
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    assert!(
        rows.iter().all(|row| row.key != TEMPERATURE_PARAM),
        "{rows:?}"
    );

    let (app, component) = app_with_compiled_veriloga_instance(
        &[
            module_parameter("gain", Some("2.5"), None, None),
            module_parameter("Temp", Some("27"), Some("C"), None),
        ],
        "",
    );
    assert!(!supports_instance_temperature(&app.state, &component));
    let rows = inline_parameter_contract(&app.state, &component, &HashMap::new());
    let temp = rows
        .iter()
        .find(|row| row.key == TEMPERATURE_PARAM)
        .expect("the module's own temp row");
    assert_eq!((temp.label.as_str(), temp.hint.as_str()), ("Temp", "27"));

    // A native device still takes the instance temperature.
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert!(supports_instance_temperature(&app.state, &resistor));
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
    let rows = inline_parameter_contract(&app.state, &component, &declared);
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

    let evidence = super::super::component_model_evidence(&app.state, &component);
    let painted = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(
        painted.contains(UNDECLARED_PARAMETER_NOTE)
            && painted.contains(REMOVE_UNDECLARED_PARAMETER),
        "{painted}"
    );

    assert!(
        app.state
            .remove_undeclared_instance_parameter(component.id, "gain")
            .is_err(),
        "a declared parameter is edited, not removed"
    );
    assert_eq!(
        app.state
            .remove_undeclared_instance_parameter(component.id, "offset"),
        Ok(true)
    );
    let removed = &app.state.model.schematic.document().components[0];
    assert_eq!(removed.params, "gain=3");
    assert!(app.state.model.schematic.can_undo());
}

/// [`app_with_compiled_veriloga_instance`] whose view owns its module's
/// source, compiled, and whose cell has the symbol a compile generates.
fn app_with_owned_veriloga_instance() -> (RSpiceApp, Component) {
    let (mut app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("10"), Some("V/V"), None)],
        "gain=5",
    );
    let reference = CellViewRef::new("behavioral", "amp", "veriloga");
    let ports = [
        crate::state::PortSpec {
            name: "p".to_owned(),
            direction: PortDirection::InOut,
        },
        crate::state::PortSpec {
            name: "n".to_owned(),
            direction: PortDirection::InOut,
        },
    ];
    let mut symbol = View::new("symbol", ViewType::Symbol);
    rspice_design::symbol::generated::store_generated_symbol(&mut symbol, &ports, |_| None)
        .expect("the symbol stores");
    app.state
        .model
        .library_manager
        .get_library_mut("behavioral")
        .and_then(|library| library.get_cell_mut("amp"))
        .expect("the cell")
        .add_view(symbol);
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(reference),
        crate::state::ProjectSourceLanguage::VerilogA,
        "amp.va",
        "module amp(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real gain = 10;\n  analog I(p, n) <+ gain * V(p, n);\nendmodule\n",
        [],
        [],
    )
    .expect("a cell-view bundle");
    let id = bundle.id();
    let sources = &mut app.state.model.workspace.content.project_sources;
    sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    sources
        .mark_bundle_validated(id)
        .expect("the module compiled");
    (app, component)
}

/// An instance of a module says what it is and nothing a model library
/// would: its view is a Verilog-A module, it runs that module, and no model
/// card or library section is offered. A current compile is not a warning.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_module_instance_states_its_view_and_module_and_no_model_library_rows() {
    let (mut app, component) = app_with_owned_veriloga_instance();

    let evidence = super::super::component_model_evidence(&app.state, &component);
    assert_eq!(evidence.model.as_deref(), Some("amp"));
    assert_eq!(evidence.problem, None, "a current compile says nothing");

    let identity = painted_panel(|ui| identity_section(ui, &mut app, &component));
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
    let parameters = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
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
    let evidence = super::super::component_model_evidence(&app.state, &component);
    assert_eq!(
        evidence.problem.as_ref().map(|(problem, _)| *problem),
        Some(Problem::Warning)
    );
    let parameters = painted_panel(|ui| parameters_section(ui, &mut app, &component, &evidence));
    assert!(parameters.contains("Recompile needed"), "{parameters}");
}

/// A component's title says what it is: a cell instance by its cell, a
/// primitive by its device kind.
#[test]
fn a_components_title_names_what_it_instantiates() {
    let (_, module) = app_with_owned_veriloga_instance();
    assert_eq!(component_title_kind(&module), "amp");
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert_eq!(component_title_kind(&resistor), "Resistor");
}
