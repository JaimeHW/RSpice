//! The inspector's account of a Verilog-A module instance: the parameter rows
//! its compile declares, its multiplicity and temperature, a value the module
//! no longer declares, and the identity it states instead of model-library
//! rows.

use super::*;

/// A compiled module parameter, as a clean compile states it.
pub(crate) fn module_parameter(
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
            authored_default: None,
            is_array: false,
            array_length: None,
        }),
    }
}

/// An instance `X1` setting `params` of `behavioral/amp`, whose `veriloga`
/// view's compile stated `contract`.
pub(crate) fn app_with_compiled_veriloga_instance(
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

/// A module reads `$temperature`, so an instance of one is offered `temp`
/// only as a parameter its module declares, as that parameter's own row.
#[test]
fn temp_is_offered_on_a_module_instance_only_as_a_declared_parameter() {
    let (app, component) = app_with_compiled_veriloga_instance(
        &[module_parameter("gain", Some("2.5"), None, None)],
        "",
    );
    assert!(!supports_instance_temperature(&app.state.model, &component));
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &component,
        &HashMap::new(),
    );
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
    assert!(!supports_instance_temperature(&app.state.model, &component));
    let rows = inline_parameter_contract(
        &app.state.design,
        &app.state.model,
        &component,
        &HashMap::new(),
    );
    let temp = rows
        .iter()
        .find(|row| row.key == TEMPERATURE_PARAM)
        .expect("the module's own temp row");
    assert_eq!((temp.label.as_str(), temp.hint.as_str()), ("Temp", "27"));

    // A native device still takes the instance temperature.
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert!(supports_instance_temperature(&app.state.model, &resistor));
}

/// [`app_with_compiled_veriloga_instance`] whose view owns its module's
/// source, compiled, and whose cell has the symbol a compile generates.
pub(crate) fn app_with_owned_veriloga_instance() -> (RSpiceApp, Component) {
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

/// A component's title says what it is: a cell instance by its cell, a
/// primitive by its device kind.
#[test]
fn a_components_title_names_what_it_instantiates() {
    let (_, module) = app_with_owned_veriloga_instance();
    assert_eq!(component_title_kind(&module), "amp");
    let resistor = Component::new(9, ComponentType::Resistor, Point::origin());
    assert_eq!(component_title_kind(&resistor), "Resistor");
}
