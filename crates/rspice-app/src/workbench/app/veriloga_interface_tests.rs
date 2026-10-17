//! A clean Verilog-A compile defines a schematic-less cell's ports, and the
//! cell's symbol, placed instances and runs follow.
//!
//! These pins compile real modules and publish them through the frame loop's
//! own poll, then read what the library, the schematic and the deck say.

use std::sync::{Arc, Mutex, mpsc};

use super::veriloga_design_tests::{DESKTOP, open_in_design, open_project_app};
use super::*;
use crate::state::{
    Cell, CellViewRef, Library, LibraryCellInstance, Point, PortDirection, ProjectSourceBundle,
    ProjectSourceLanguage, ProjectSourceOwner, SymbolDocument, SymbolElectricalType, View,
    ViewType,
};
use crate::workbench::documents::code_workspace::{
    self, PendingVerilogACompile, VerilogACellSymbol, VerilogACompileOutcome,
    VerilogAInterfaceOutcome,
};
use rspice_design::library::metadata::{PortDomain, SourceViewPort, decode_source_view_ports};
use rspice_design::symbol::generated::{
    GENERATED_PORTS_KEY, GENERATED_SYMBOL_VIEW, is_generated_symbol, store_generated_symbol,
};

const LIBRARY: &str = "behavioral";

/// A writable library cell `cell` whose `veriloga` view owns `source`, and
/// states the names-only interface New View used to write.
pub(super) fn add_module_cell(
    state: &mut AppState,
    cell: &str,
    module: &str,
    source: &str,
) -> CellViewRef {
    add_module_cell_in(state, LIBRARY, cell, module, source)
}

/// [`add_module_cell`] in `library`, which is created if it is missing.
fn add_module_cell_in(
    state: &mut AppState,
    library: &str,
    cell: &str,
    module: &str,
    source: &str,
) -> CellViewRef {
    if state.library_manager.get_library(library).is_none() {
        state.library_manager.add_library(Library::new(library));
    }
    let reference = CellViewRef::new(library, cell, "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module.to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut library_cell = Cell::new(cell);
    library_cell.add_view(view);
    state
        .library_manager
        .get_library_mut(library)
        .expect("the library")
        .add_cell(library_cell);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        format!("{cell}.va"),
        source,
        [],
        [],
    )
    .expect("a valid cell-view bundle");
    state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    reference
}

fn conductance(module: &str, header: &str, declarations: &str) -> String {
    format!(
        "module {module}({header});\n{declarations}  analog I(p, n) <+ 1m * V(p, n);\nendmodule\n"
    )
}

/// Compile the open view and publish the result through the frame loop's
/// poll, exactly as a worker's answer is published.
pub(super) fn compile_and_publish(app: &mut RSpiceApp) {
    let selected = code_workspace::selected_veriloga_source(app).expect("a Verilog-A view is open");
    let receipt = code_workspace::compile_project_bundle_receipt(
        app.state.workspace.content.project.id(),
        selected.bundle(),
        selected.selected_module(),
    )
    .unwrap_or_else(|diagnostics| panic!("the module compiles: {diagnostics:#?}"));
    let (sender, receiver) = mpsc::channel();
    sender
        .send(VerilogACompileOutcome::Success(Box::new(
            (**receipt.compilation.report()).clone(),
        )))
        .expect("the channel is open");
    app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token: receipt.compilation.token(),
        receiver: Arc::new(Mutex::new(receiver)),
    });
    code_workspace::poll_veriloga_compile(app);
    assert!(
        app.state.ui.code_workspace.veriloga.receipt.is_some(),
        "the compile was published"
    );
    code_workspace::refresh_symbol_standing(
        &mut app.state.ui.code_workspace.veriloga,
        &app.state.library_manager,
        &app.state.workspace.content.active_view,
        app.state.workspace.content.active_view_type(),
    );
}

/// Replace the open view's root source with `source`.
pub(super) fn edit_source(app: &mut RSpiceApp, source: &str) {
    let selected = code_workspace::selected_veriloga_source(app).expect("a Verilog-A view is open");
    let path = selected.document().logical_path().to_owned();
    assert!(
        code_workspace::replace_selected_veriloga_file(app, &selected, &path, source.to_owned())
            .expect("the edit applies")
    );
}

pub(super) fn view<'a>(
    app: &'a RSpiceApp,
    reference: &CellViewRef,
    name: &str,
) -> Option<&'a View> {
    app.state
        .library_manager
        .get_library(&reference.library)?
        .get_cell(&reference.cell)?
        .get_view(name)
}

fn stated_ports(app: &RSpiceApp, reference: &CellViewRef) -> Vec<SourceViewPort> {
    view(app, reference, &reference.view)
        .and_then(|view| view.metadata.get("veriloga.ports"))
        .and_then(|encoded| decode_source_view_ports(encoded))
        .expect("the view states a structured interface")
}

fn port_names(ports: &[SourceViewPort]) -> Vec<(&str, PortDirection)> {
    ports
        .iter()
        .map(|port| (port.name.as_str(), port.direction))
        .collect()
}

fn symbol(app: &RSpiceApp, reference: &CellViewRef) -> SymbolDocument {
    SymbolDocument::load_from_view(
        view(app, reference, GENERATED_SYMBOL_VIEW).expect("the cell has a symbol"),
    )
    .expect("the symbol loads")
}

fn pin_names(document: &SymbolDocument) -> Vec<(&str, PortDirection)> {
    let mut pins = document
        .pins
        .iter()
        .map(|pin| (pin.name.as_str(), pin.direction))
        .collect::<Vec<_>>();
    pins.sort_by_key(|(name, _)| *name);
    pins
}

/// `veriloga.ports` used to be written once, when the view was created, so a
/// port added to the module never reached the cell, its symbol or anything
/// placed from it.
#[test]
fn a_port_added_to_the_module_reaches_the_cell_and_its_generated_symbol() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "gain",
        "gain",
        &conductance("gain", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    open_in_design(&mut app.state, &reference);

    compile_and_publish(&mut app);
    assert_eq!(
        port_names(&stated_ports(&app, &reference)),
        [("p", PortDirection::InOut), ("n", PortDirection::InOut)]
    );
    let generated = view(&app, &reference, GENERATED_SYMBOL_VIEW).expect("a symbol is generated");
    assert!(is_generated_symbol(generated));
    assert!(app.state.workspace.content.project_metadata_dirty);

    edit_source(
        &mut app,
        &conductance(
            "gain",
            "p, n, ctl",
            "  inout p, n;\n  input ctl;\n  electrical p, n, ctl;\n",
        ),
    );
    compile_and_publish(&mut app);
    assert_eq!(
        port_names(&stated_ports(&app, &reference)),
        [
            ("p", PortDirection::InOut),
            ("n", PortDirection::InOut),
            ("ctl", PortDirection::In),
        ]
    );
    assert_eq!(
        view(&app, &reference, GENERATED_SYMBOL_VIEW)
            .and_then(|view| view.metadata.get(GENERATED_PORTS_KEY))
            .map(String::as_str),
        Some("p:inout n:inout ctl:in")
    );
    assert_eq!(
        pin_names(&symbol(&app, &reference)),
        [
            ("ctl", PortDirection::In),
            ("n", PortDirection::InOut),
            ("p", PortDirection::InOut),
        ]
    );

    edit_source(
        &mut app,
        &conductance(
            "gain",
            "p, ctl, n",
            "  inout p;\n  input ctl;\n  output n;\n  electrical p, n, ctl;\n",
        ),
    );
    compile_and_publish(&mut app);
    assert_eq!(
        port_names(&stated_ports(&app, &reference)),
        [
            ("p", PortDirection::InOut),
            ("ctl", PortDirection::In),
            ("n", PortDirection::Out),
        ],
        "the order and the directions are the module's"
    );

    edit_source(
        &mut app,
        &conductance("gain", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    compile_and_publish(&mut app);
    assert_eq!(
        pin_names(&symbol(&app, &reference)),
        [("n", PortDirection::InOut), ("p", PortDirection::InOut)],
        "a removed port's pin goes"
    );
}

const VECTOR_MODULE: &str = "module vec(d, p, n);\n  inout [3:0] d;\n  inout p, n;\n  electrical [3:0] d;\n  electrical p, n;\n  analog begin\n    I(p, n) <+ 1m * V(p, n);\n    I(d[3], d[0]) <+ 1m * V(d[3], d[0]);\n  end\nendmodule\n";

/// A configuration that executes `veriloga` views, and an instance of
/// `reference` placed in the top schematic with the interface its view
/// states, the way the library shelf places one.
fn place_configured_instance(state: &mut AppState, reference: &CellViewRef) -> u64 {
    place_configured_instance_at(state, reference, Point::new(20, 20))
}

/// [`place_configured_instance`] at `at`.
fn place_configured_instance_at(state: &mut AppState, reference: &CellViewRef, at: Point) -> u64 {
    let ports = rspice_design::library::metadata::metadata_ports(
        &state
            .library_manager
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .expect("the view")
            .metadata,
    )
    .expect("the view states its ports");
    state.open_workspace_view(CellViewRef::default_top());
    let mut binding =
        LibraryCellInstance::new(&reference.library, &reference.cell, &reference.view);
    binding.bind_interface(&ports);
    let id = state.schematic.add_library_cell_component(at, binding);
    ensure_veriloga_configuration(state);
    id
}

/// A configuration rooted at the top schematic that executes `veriloga`
/// views, unless the project already has one.
pub(super) fn ensure_veriloga_configuration(state: &mut AppState) {
    let top = CellViewRef::default_top();
    if state
        .workspace
        .content
        .configuration_sets
        .configurations()
        .is_empty()
    {
        state
            .workspace
            .content
            .configuration_sets
            .create(crate::state::ConfigurationSetDefinition {
                name: "Behavioral".to_owned(),
                root: top,
                dut_path: "/X1".to_owned(),
                executable_view_policy: vec!["veriloga".to_owned()],
                stop_views: vec!["veriloga".to_owned()],
                unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
                black_box_policy:
                    crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
                overrides: Vec::new(),
                model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
                owner: "Behavioral design".to_owned(),
            })
            .expect("create the executable configuration");
    }
}

/// The deck for the top schematic as configured, after its Verilog-A
/// runtimes prepared: what a run would emit, or why it refuses.
pub(super) fn configured_deck(app: &RSpiceApp) -> Result<String, String> {
    let state = &app.state;
    let projection = state
        .workspace
        .configuration_execution_projection(
            &state.library_manager,
            &state.workspace.content.active_view,
            &state.schematic,
        )
        .map_err(|error| error.to_string())?;
    rspice_simulation::project_veriloga::preparation::prepared_configuration_veriloga_runtimes(
        state.workspace.content.project.id(),
        &state.workspace.content.project_sources,
        &projection,
    )
    .map_err(|error| error.message().to_owned())?;
    let hierarchy = rspice_design::hierarchy::HierarchySource::from_execution_projection(
        state.library_manager.catalog(),
        &projection,
    );
    let generated = rspice_simulation::netlist_gen::generate_netlist_hierarchical(
        projection.root_schematic().expect("a materialized root"),
        &[],
        &hierarchy,
        &rspice_simulation::netlist_gen::NetlistSourceData::new(
            &crate::simulation::table_route::SourceFiles,
        ),
    );
    if generated.errors.is_empty() {
        Ok(generated.netlist)
    } else {
        Err(generated.errors.join("; "))
    }
}

/// A continuous vector is one bus pin whose conductors are the module's
/// terminals in the module's order: `d[3:0]` expands most significant first,
/// which is how the compiled module binds `d[3]` to `d[0]`.
#[test]
fn a_continuous_vector_is_one_bus_pin_whose_conductors_are_the_modules_terminals() {
    let mut app = open_project_app();
    let reference = add_module_cell(&mut app.state, "vec", "vec", VECTOR_MODULE);
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);

    let ports = stated_ports(&app, &reference);
    assert_eq!(
        port_names(&ports),
        [
            ("d[3:0]", PortDirection::InOut),
            ("p", PortDirection::InOut),
            ("n", PortDirection::InOut),
        ]
    );
    assert_eq!(ports[0].domain, PortDomain::Continuous);
    let abi = &app
        .state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .expect("the receipt")
        .compilation
        .report()
        .abi;
    let conductors = rspice_design::schematic::bus::declared_vector("d[3:0]")
        .expect("a bus pin")
        .members()
        .iter()
        .map(ToString::to_string)
        .chain(["p".to_owned(), "n".to_owned()])
        .collect::<Vec<_>>();
    let lanes = abi
        .analog_ports
        .iter()
        .map(|port| port.name.to_string())
        .collect::<Vec<_>>();
    assert_eq!(conductors, lanes, "the pin expands as the module binds");
    assert_eq!(
        symbol(&app, &reference).pins.len(),
        3,
        "one pin per port, a vector included"
    );

    place_configured_instance(&mut app.state, &reference);
    let deck = configured_deck(&app).expect("the instance netlists and its runtime prepares");
    let line = deck
        .lines()
        .find(|line| line.contains("#3"))
        .unwrap_or_else(|| panic!("the instance line carries the bus conductors:\n{deck}"));
    let bits = line
        .split_whitespace()
        .filter_map(|token| token.rsplit_once('#').map(|(_, bit)| bit))
        .collect::<Vec<_>>();
    assert_eq!(bits, ["3", "2", "1", "0"], "{line}");
}

/// A module is named after its cell, so two libraries can each hold a cell
/// `amp` whose module is `amp`. Placed in one design, each instance binds its
/// own view's module: the deck names a project module by the bundle that owns
/// it, never by the name the two share, and each bundle's source is read once
/// under its own key.
#[test]
fn same_named_cells_in_two_libraries_each_bind_their_own_module() {
    let mut app = open_project_app();
    let mut placed = Vec::new();
    for (library, gain) in [("analog_a", "1m"), ("analog_b", "2m")] {
        let reference = add_module_cell_in(
            &mut app.state,
            library,
            "amp",
            "amp",
            &format!(
                "module amp(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ {gain} * V(p, n);\nendmodule\n"
            ),
        );
        open_in_design(&mut app.state, &reference);
        compile_and_publish(&mut app);
        placed.push(reference);
    }
    let mut instances = Vec::new();
    for (reference, at) in placed.iter().zip([Point::new(20, 20), Point::new(200, 20)]) {
        let id = place_configured_instance_at(&mut app.state, reference, at);
        instances.push((reference.clone(), id));
    }

    let deck = configured_deck(&app).expect("both instances netlist and their runtimes prepare");

    let bindings = instances
        .iter()
        .map(|(reference, id)| {
            let binding = crate::state::workspace::project_veriloga_binding_for_view(
                &app.state.workspace,
                &app.state.library_manager,
                reference,
            )
            .expect("the view owns its module");
            let name = app
                .state
                .schematic
                .document()
                .components
                .iter()
                .find(|component| component.id == *id)
                .expect("the placed instance")
                .spice_instance_name();
            (binding, name)
        })
        .collect::<Vec<_>>();
    assert_eq!(bindings[0].0.selected_module(), "amp");
    assert_eq!(bindings[1].0.selected_module(), "amp");
    assert!(
        !bindings[0]
            .0
            .netlist_alias()
            .eq_ignore_ascii_case(bindings[1].0.netlist_alias()),
        "two bundles' modules are two names in the deck"
    );
    for (binding, name) in &bindings {
        let lines = deck
            .lines()
            .filter(|line| {
                !line.trim_start().starts_with(['.', '*'])
                    && line
                        .split_whitespace()
                        .any(|token| token.eq_ignore_ascii_case(binding.netlist_alias()))
            })
            .collect::<Vec<_>>();
        assert_eq!(
            lines.len(),
            1,
            "one instance binds {name}'s module:\n{deck}"
        );
        assert!(
            lines[0]
                .split_whitespace()
                .next()
                .is_some_and(|token| token.eq_ignore_ascii_case(name)),
            "{name} binds its own view's module:\n{deck}"
        );
        assert_eq!(
            deck.matches(binding.source_key()).count(),
            1,
            "each bundle's source is read once, under its own key:\n{deck}"
        );
    }
}

/// A discrete vector's bits travel as one digital signal: one pin, `Logic`,
/// one terminal, so the symbol's conductor count is the module's.
#[test]
fn a_discrete_vector_is_one_logic_pin_on_one_terminal() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "moded",
        "moded",
        "module moded(p, n, mode);\n  inout p, n;\n  input [1:0] mode;\n  electrical p, n;\n  logic mode;\n  analog I(p, n) <+ (mode[0] ? 1m : 2m) * V(p, n);\nendmodule\n",
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);

    let ports = stated_ports(&app, &reference);
    assert_eq!(
        port_names(&ports),
        [
            ("p", PortDirection::InOut),
            ("n", PortDirection::InOut),
            ("mode", PortDirection::In),
        ]
    );
    assert_eq!(ports[2].domain, PortDomain::Discrete);
    assert_eq!(ports[2].discipline, "logic");
    let symbol = symbol(&app, &reference);
    assert_eq!(
        symbol.pin("mode").and_then(|pin| pin.electrical_type),
        Some(SymbolElectricalType::Logic)
    );
    let terminals = symbol
        .pins
        .iter()
        .map(|pin| rspice_design::schematic::bus::declared_width(&pin.name))
        .sum::<usize>();
    let lanes = app
        .state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .expect("the receipt")
        .compilation
        .report()
        .abi
        .analog_ports
        .len();
    assert_eq!(terminals, lanes);
    assert_eq!(lanes, 3);
}

/// A cell's schematic defines its ports. A module that compiles to different
/// ones is a finding of that compile, the editor's and the design check's,
/// and never rewrites the interface the schematic owns.
#[test]
fn a_module_that_differs_from_its_cells_schematic_is_refused_and_writes_nothing() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "amp",
        "amp",
        &conductance(
            "amp",
            "p, n, ctl",
            "  inout p, n;\n  input ctl;\n  electrical p, n, ctl;\n",
        ),
    );
    let schematic = CellViewRef::new(LIBRARY, "amp", "schematic");
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
        master
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("the port")
            .value = name.to_owned();
    }
    app.state
        .workspace
        .insert_schematic_editor(schematic.key(), master);
    open_in_design(&mut app.state, &reference);
    let before = view(&app, &reference, "veriloga").map(|view| view.metadata.clone());

    compile_and_publish(&mut app);

    let receipt = app
        .state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .expect("the module compiled");
    assert!(matches!(
        receipt.interface,
        Some(VerilogAInterfaceOutcome::SchematicMismatch { .. })
    ));
    let finding = receipt
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "VA-INTERFACE-SCHEMATIC-MISMATCH")
        .expect("the editor shows the mismatch");
    assert!(
        finding.detail.contains("inout p, inout n"),
        "{}",
        finding.detail
    );
    assert!(finding.detail.contains("input ctl"), "{}", finding.detail);
    assert_eq!(
        view(&app, &reference, "veriloga").map(|view| view.metadata.clone()),
        before
    );
    assert!(view(&app, &reference, GENERATED_SYMBOL_VIEW).is_none());

    let report = super::dialogs::check_and_save_validation::CheckAndSaveValidationReport::validate(
        &app.state,
    )
    .expect("the design check runs");
    assert!(
        report.advisories().iter().any(|advisory| advisory
            .message
            .contains("differ from its cell's schematic")),
        "{:?}",
        report.advisories()
    );
}

/// A symbol its author edited is never rewritten by a compile. Its drift
/// from the module is reported, and Update symbol reconciles it as one
/// project undo step, keeping the pins no port declares.
#[test]
fn a_hand_edited_symbol_reports_its_drift_and_updates_as_one_undo_step() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "amp",
        "amp",
        &conductance("amp", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    // The author's first edit takes the symbol over.
    let drawn = symbol(&app, &reference);
    app.state
        .library_manager
        .edit_library(LIBRARY)
        .and_then(|mut library| library.store_symbol_document("amp", GENERATED_SYMBOL_VIEW, &drawn))
        .expect("the symbol view")
        .expect("the symbol stores");
    assert!(!is_generated_symbol(
        view(&app, &reference, GENERATED_SYMBOL_VIEW).expect("the symbol")
    ));

    edit_source(
        &mut app,
        &conductance(
            "amp",
            "p, ctl",
            "  inout p;\n  input ctl;\n  electrical p, ctl;\n  electrical n;\n",
        ),
    );
    compile_and_publish(&mut app);

    assert_eq!(symbol(&app, &reference), drawn, "a compile leaves it alone");
    let Some(VerilogACellSymbol::Drifted(drift)) = app
        .state
        .ui
        .code_workspace
        .veriloga
        .symbol_standing
        .of(&reference)
        .cloned()
    else {
        panic!("the drift is measured")
    };
    assert_eq!(drift.missing, ["ctl"]);
    assert_eq!(drift.extra, ["n"]);
    assert_eq!(
        code_workspace::symbol_drift_text(&drift, app.state.ui.messages()),
        "1 pin missing, 1 extra"
    );
    assert!(!app.state.can_undo_project_design());

    let summary = app
        .state
        .update_veriloga_symbol(&reference)
        .expect("the update applies");
    assert!(summary.contains("Pin n is not a module port"), "{summary}");
    assert_eq!(
        pin_names(&symbol(&app, &reference)),
        [
            ("ctl", PortDirection::In),
            ("n", PortDirection::InOut),
            ("p", PortDirection::InOut),
        ],
        "the missing pin is added and the extra one kept"
    );

    assert!(app.state.can_undo_project_design());
    assert!(app.state.undo_project_design().expect("undo").is_some());
    assert_eq!(symbol(&app, &reference), drawn, "undo restores the drawing");
    assert!(
        !app.state.can_undo_project_design(),
        "the update was exactly one undo step"
    );
}

/// New Cell stores an empty generated symbol. Read as an authored drawing
/// with no pins, every port of the module resolved unplaced and the netlister
/// refused the instance with "terminal mismatch".
#[test]
fn a_new_cells_empty_generated_symbol_netlists_the_modules_instance() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "gain",
        "gain",
        &conductance("gain", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    let mut empty = View::new(GENERATED_SYMBOL_VIEW, ViewType::Symbol);
    store_generated_symbol(&mut empty, &[], |_| None).expect("the empty symbol stores");
    app.state
        .library_manager
        .get_library_mut(LIBRARY)
        .and_then(|library| library.get_cell_mut("gain"))
        .expect("the cell")
        .add_view(empty);

    // As a project saved before the compile defined the cell left it.
    place_configured_instance(&mut app.state, &reference);
    let deck = configured_deck(&app);
    assert!(deck.is_ok(), "{deck:?}");

    // And after a clean compile has regenerated the symbol.
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    app.state.open_workspace_view(CellViewRef::default_top());
    let deck = configured_deck(&app);
    assert!(deck.is_ok(), "{deck:?}");
}

/// A generated symbol in a read-only library is not the compile's to write.
#[test]
fn a_read_only_library_is_not_written_and_the_compile_says_so() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "gain",
        "gain",
        &conductance(
            "gain",
            "p, n, ctl",
            "  inout p, n;\n  input ctl;\n  electrical p, n, ctl;\n",
        ),
    );
    app.state
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("the library")
        .read_only = true;
    open_in_design(&mut app.state, &reference);
    let before = view(&app, &reference, "veriloga").map(|view| view.metadata.clone());

    compile_and_publish(&mut app);

    assert_eq!(
        view(&app, &reference, "veriloga").map(|view| view.metadata.clone()),
        before
    );
    assert!(view(&app, &reference, GENERATED_SYMBOL_VIEW).is_none());
    assert!(matches!(
        app.state
            .ui
            .code_workspace
            .veriloga
            .receipt
            .as_ref()
            .and_then(|receipt| receipt.interface.as_ref()),
        Some(VerilogAInterfaceOutcome::Refused(reason)) if reason.contains("read-only")
    ));
    assert!(app.state.log_buffer.entries().any(|entry| {
        entry
            .message
            .contains("did not become the cell's interface")
    }));
}

/// An instance placed before the module gained a port is offered the same
/// repair a schematic master's gets; the run's refusal names it, and once it
/// is applied the run prepares and netlists.
#[test]
fn an_instance_placed_before_a_port_change_is_repaired_and_the_run_proceeds() {
    let mut app = open_project_app();
    let reference = add_module_cell(
        &mut app.state,
        "gain",
        "gain",
        &conductance("gain", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    let placed = place_configured_instance(&mut app.state, &reference);
    assert!(configured_deck(&app).is_ok());

    open_in_design(&mut app.state, &reference);
    edit_source(
        &mut app,
        &conductance(
            "gain",
            "p, n, ctl",
            "  inout p, n;\n  input ctl;\n  electrical p, n, ctl;\n",
        ),
    );
    compile_and_publish(&mut app);
    app.state.open_workspace_view(CellViewRef::default_top());

    let refusal = configured_deck(&app).expect_err("the stale instance is refused");
    assert!(refusal.contains("Update instance interface"), "{refusal}");
    let masters = std::collections::HashMap::<String, crate::state::SchematicState>::new();
    assert_eq!(
        app.state
            .schematic
            .stale_instance_interfaces(&app.state.library_manager, &masters)
            .len(),
        1
    );
    app.state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(placed);
    assert!(
        crate::workbench::commands::vocabulary::Command::UpdateInstanceInterface.is_enabled(&app)
    );
    crate::workbench::commands::vocabulary::Command::UpdateInstanceInterface.execute(&mut app);

    let deck = configured_deck(&app);
    assert!(deck.is_ok(), "{deck:?}");
}

/// Renders for review: the editor's interface strip and the inspector's
/// Symbol row with Update symbol, for a hand-edited symbol that drifted, and
/// the strip for a module that differs from its cell's schematic.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_veriloga_interface_findings_for_review() {
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

    // A hand-edited symbol the module has moved on from.
    let mut drifted = open_project_app();
    let reference = add_module_cell(
        &mut drifted.state,
        "gain_cell",
        "gain_cell",
        &conductance("gain_cell", "p, n", "  inout p, n;\n  electrical p, n;\n"),
    );
    open_in_design(&mut drifted.state, &reference);
    compile_and_publish(&mut drifted);
    let drawn = symbol(&drifted, &reference);
    drifted
        .state
        .library_manager
        .edit_library(LIBRARY)
        .and_then(|mut library| {
            library.store_symbol_document("gain_cell", GENERATED_SYMBOL_VIEW, &drawn)
        })
        .expect("the symbol view")
        .expect("the symbol stores");
    edit_source(
        &mut drifted,
        &conductance(
            "gain_cell",
            "p, ctl, en",
            "  inout p;\n  input ctl, en;\n  electrical p, ctl, en;\n  electrical n;\n",
        ),
    );
    compile_and_publish(&mut drifted);
    for (name, size) in [
        ("veriloga-symbol-drift-1440x900", DESKTOP),
        ("veriloga-symbol-drift-390x844", egui::vec2(390.0, 844.0)),
    ] {
        drifted.state.workbench.coarse_pointer = size.x < 500.0;
        let canvas = crate::ui::raster::render(size, |ui, _| drifted.render_frame_chrome(ui));
        write(name, &canvas, size.y as usize);
    }

    // A module that differs from its cell's schematic.
    let mut mismatched = open_project_app();
    let reference = add_module_cell(
        &mut mismatched.state,
        "amp",
        "amp",
        &conductance(
            "amp",
            "p, n, ctl",
            "  inout p, n;\n  input ctl;\n  electrical p, n, ctl;\n",
        ),
    );
    mismatched
        .state
        .library_manager
        .get_library_mut(LIBRARY)
        .and_then(|library| library.get_cell_mut("amp"))
        .expect("the cell")
        .add_view(View::new("schematic", ViewType::Schematic));
    open_in_design(&mut mismatched.state, &reference);
    compile_and_publish(&mut mismatched);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| mismatched.render_frame_chrome(ui));
    write(
        "veriloga-schematic-mismatch-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );
}
