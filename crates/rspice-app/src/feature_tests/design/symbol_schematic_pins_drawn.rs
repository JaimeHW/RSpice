//! Creating schematic pins from a symbol, drawn through a whole
//! application: the other direction of updating a symbol's pins from its
//! schematic.

use egui::accesskit::Role;

use super::app_state_of;
use crate::feature_tests::replay::Replay;
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::workspace::tests::state_with_amp_symbol_pin;
use crate::workbench::framework::command_registry::ids;
use crate::workbench::state::Workspace;
use rspice_app_state::state::{CellViewRef, ComponentType, Point, PortDirection, SymbolPin};

/// The amp whose schematic declares IN, and whose symbol has OUT and VDD
/// besides.
fn amp_whose_symbol_has_more_pins() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state = app_state_of(state_with_amp_symbol_pin("IN", Some(Point::new(-40, 0))));
    let mut document = app.state.model.load_active_symbol_document().unwrap();
    document.pins.push(SymbolPin::new(
        "OUT",
        PortDirection::Out,
        Some(Point::new(40, 0)),
    ));
    document.pins.push(SymbolPin::new(
        "VDD",
        PortDirection::Supply,
        Some(Point::new(0, -40)),
    ));
    app.state.store_active_symbol_document(&document).unwrap();
    app.state.workbench.activate(Workspace::Design);
    app
}

/// Every port of the schematic in front: its name, direction and place.
fn ports(app: &RSpiceApp) -> Vec<(String, PortDirection, Point)> {
    app.state
        .model
        .schematic
        .document()
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::Port)
        .map(|port| {
            let contract = port.port_contract().expect("a named port");
            (port.value.clone(), contract.direction, port.pos)
        })
        .collect()
}

/// The pins only the symbol has become ports of its schematic, named and
/// directed as the pins are, in one column clear of what the sheet holds,
/// as one undo step of the schematic; the port it had stays where and as it
/// was, and the offer goes once there is nothing left to create.
#[test]
fn the_symbols_extra_pins_become_schematic_ports_as_one_step() {
    let mut replay = Replay::of(amp_whose_symbol_has_more_pins());
    let offered = replay.click("Create schematic pins from symbol", Role::Button);
    assert!(
        !offered
            .iter()
            .any(|control| control.label == "Create schematic pins from symbol"),
        "the schematic declares every pin now, so nothing is offered"
    );

    replay
        .app
        .state
        .open_workspace_view(CellViewRef::new("work", "amp", "schematic"));
    replay.frames(Vec::new());
    let created = ports(&replay.app);
    let names = created
        .iter()
        .map(|(name, direction, _)| (name.as_str(), *direction))
        .collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            ("IN", PortDirection::InOut),
            ("OUT", PortDirection::Out),
            ("VDD", PortDirection::Supply),
        ]
    );
    assert_eq!(created[0].2, Point::new(0, 0), "the port it had stays");
    let (out, vdd) = (created[1].2, created[2].2);
    assert_eq!(out.x, vdd.x, "one column");
    assert!(vdd.y > out.y, "in the symbol's netlist order");
    assert!(out.x < 0, "clear of the sheet's content, to its left");

    replay.app.execute_command_key(ids::UNDO);
    replay.frames(Vec::new());
    let undone = ports(&replay.app);
    assert_eq!(
        undone.len(),
        1,
        "one undo takes both ports back: {undone:?}"
    );
    assert_eq!(undone[0].0, "IN");
}
