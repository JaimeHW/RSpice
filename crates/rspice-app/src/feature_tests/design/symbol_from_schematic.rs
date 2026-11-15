//! A symbol made from a schematic with real pins, and the schematic's ports
//! and the symbol's pins kept in step, through whole frames of the
//! application on a project that has just been created.

use crate::state::{
    CellViewRef, ComponentType, PinFindingKind, PortDirection, SymbolDocument, SymbolPin, ViewType,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app::project_transactions::open_new_project_dialog;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::state::Workspace;

use super::super::replay::Replay;

/// A project just created under the defaults New project offers, its top
/// cell's schematic in front on the Design workspace.
fn new_project() -> Replay {
    let mut app = RSpiceApp::test_instance();
    open_new_project_dialog(&mut app.state);
    app.state.project.new_project.name = "untitled".to_owned();
    let params = app.state.project.new_project.params();
    app.state
        .create_new_project_with(&params)
        .expect("the default identity creates a project");
    app.state.project.new_project = Default::default();
    app.state.workbench.activate(Workspace::Design);
    let mut replay = Replay::of(app);
    replay.frames(Vec::new());
    replay
}

/// The schematic in front declares `ports`, as one step of it.
fn declare(replay: &mut Replay, ports: &[(&str, PortDirection)]) {
    let pins = ports
        .iter()
        .map(|(name, direction)| SymbolPin::new(*name, *direction, None))
        .collect::<Vec<_>>();
    let outcome = replay
        .app
        .state
        .model
        .schematic
        .declare_ports_for_symbol_pins(&pins, "Place pins")
        .expect("the schematic takes the ports");
    assert!(outcome.refused.is_empty(), "{:?}", outcome.refused);
    replay.frames(Vec::new());
}

/// The port of the schematic in front named `name`.
fn port_id(app: &RSpiceApp, name: &str) -> u64 {
    app.state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.kind == ComponentType::Port && component.value == name)
        .map(|component| component.id)
        .unwrap_or_else(|| panic!("the schematic declares {name}"))
}

/// The symbol of the cell `schematic` belongs to, as the library holds it.
fn symbol_of(schematic: &CellViewRef) -> CellViewRef {
    CellViewRef::new(&schematic.library, &schematic.cell, "symbol")
}

fn symbol_view_type(app: &RSpiceApp, symbol: &CellViewRef) -> Option<ViewType> {
    app.state
        .model
        .library_manager
        .get_library(&symbol.library)
        .and_then(|library| library.get_cell(&symbol.cell))
        .and_then(|cell| cell.get_view(&symbol.view))
        .map(|view| view.view_type)
}

fn document(app: &RSpiceApp, symbol: &CellViewRef) -> SymbolDocument {
    app.state
        .model
        .load_symbol_document(symbol)
        .expect("the symbol loads")
}

/// A symbol the schematic made is never removed by the schematic: deleting
/// its last port leaves the symbol as it was drawn, and its pin is then the
/// one the pin checks report as not in the schematic.
#[test]
fn deleting_the_last_port_keeps_the_symbol_and_orphans_its_pin() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(&mut replay, &[("IN", PortDirection::In)]);
    assert_eq!(
        symbol_view_type(&replay.app, &symbol),
        Some(ViewType::Symbol),
        "the schematic's first port made its symbol"
    );
    let drawn = document(&replay.app, &symbol);
    assert!(drawn.pin("IN").is_some());

    let port = port_id(&replay.app, "IN");
    replay
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(port);
    replay.app.execute_command_key(ids::DELETE);
    replay.frames(Vec::new());
    assert!(
        replay
            .app
            .state
            .model
            .schematic
            .interface_ports()
            .is_empty()
    );

    assert_eq!(
        symbol_view_type(&replay.app, &symbol),
        Some(ViewType::Symbol),
        "the symbol stays"
    );
    assert_eq!(document(&replay.app, &symbol), drawn, "as it was drawn");
    let interface = replay.app.state.model.symbol_interface(&symbol);
    let findings = drawn
        .pin_findings(&interface)
        .into_iter()
        .map(|finding| (finding.kind, finding.pin_name))
        .collect::<Vec<_>>();
    assert_eq!(findings, [(PinFindingKind::OrphanedPin, "IN".to_owned())]);

    replay.app.execute_command_key(ids::UNDO);
    replay.frames(Vec::new());
    assert_eq!(
        document(&replay.app, &symbol),
        drawn,
        "the port back, the symbol answers it again"
    );
}
