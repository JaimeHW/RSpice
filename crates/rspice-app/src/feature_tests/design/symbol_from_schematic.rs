//! A symbol made from a schematic with real pins, and the schematic's ports
//! and the symbol's pins kept in step, through whole frames of the
//! application on a project that has just been created.

use crate::workbench::RSpiceApp;
use crate::workbench::app::project_transactions::open_new_project_dialog;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{
    CellViewRef, Component, ComponentType, PinFindingKind, Point, PortDirection, SymbolDocument,
    SymbolInterface, SymbolPin, SymbolPinSide, ViewType,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;

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

/// Edit the port `name` of the schematic in front as the property dialog
/// commits it: one step of the schematic.
fn edit_port(replay: &mut Replay, name: &str, edit: impl FnOnce(&mut Component)) {
    let id = port_id(&replay.app, name);
    let expected = replay
        .app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the port is placed")
        .clone();
    let mut candidate = expected.clone();
    edit(&mut candidate);
    replay
        .app
        .state
        .model
        .edit_component_transaction(&expected, candidate, "edit properties")
        .expect("the port edit commits");
    replay.frames(Vec::new());
}

/// Delete the port `name` of the schematic in front, as Delete does.
fn delete_port(replay: &mut Replay, name: &str) {
    let port = port_id(&replay.app, name);
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
}

/// Bring `view` to the front of the Design workspace.
fn open(replay: &mut Replay, view: &CellViewRef) {
    replay.app.state.open_workspace_view(view.clone());
    replay.app.state.workbench.activate(Workspace::Design);
    replay.frames(Vec::new());
}

/// Make `edit` to the symbol in front as one hand edit, called `step`.
fn hand_edit(replay: &mut Replay, step: &str, edit: impl FnOnce(&mut SymbolDocument)) {
    let before = replay
        .app
        .state
        .model
        .load_active_symbol_document()
        .expect("the symbol in front loads");
    let metadata = replay
        .app
        .state
        .model
        .load_symbol_editor_metadata(
            &replay.app.state.model.workspace.content.active_view,
            &before,
        )
        .expect("its metadata loads");
    let mut edited = before.clone();
    edit(&mut edited);
    rspice_app_design::workbench::app::actions::workspace::symbol_document::record_symbol_edit(
        &mut replay.app.state.design.symbol,
        &replay.app.state.model,
        &before,
        step,
    );
    replay
        .app
        .state
        .store_active_symbol_editor_bundle(&edited, &metadata)
        .expect("the edit stores");
    replay.frames(Vec::new());
}

/// Where each pin's terminal is, by name.
fn tips(document: &SymbolDocument) -> Vec<(String, Point)> {
    document
        .pins
        .iter()
        .map(|pin| (pin.name.clone(), pin.position.expect("every pin is placed")))
        .collect()
}

/// Every pin of `document` stands on the terminal grid, at a terminal of
/// its own, with its lead reaching the body on its side.
fn assert_tidy(document: &SymbolDocument) {
    let tips = tips(document);
    for (index, (name, tip)) in tips.iter().enumerate() {
        assert!(
            tips[..index].iter().all(|(_, other)| other != tip),
            "{name} has a terminal of its own"
        );
    }
    let (min, max) = document.body_bounds();
    for pin in &document.pins {
        assert!(
            pin.terminal_on_grid(),
            "{} is on the terminal grid",
            pin.name
        );
        let inner = pin.lead_inner().expect("a placed pin has a lead");
        let on_edge = match pin.side {
            SymbolPinSide::Left => inner.x == min.x,
            SymbolPinSide::Right => inner.x == max.x,
            SymbolPinSide::Top => inner.y == min.y,
            SymbolPinSide::Bottom => inner.y == max.y,
        };
        assert!(on_edge, "{}'s lead reaches the body", pin.name);
    }
}

/// A schematic with ports of every kind makes a symbol with each pin on the
/// edge its role takes, inputs left, outputs and bidirectional pins right,
/// positive rails on top and ground rails below, every terminal on the grid
/// and every lead reaching the body. A bus port is one pin; a name no port
/// may take is refused, with why; the symbol opens ready to edit.
#[test]
fn ports_of_every_kind_make_a_tidy_symbol() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    let refused = replay
        .app
        .state
        .model
        .schematic
        .declare_ports_for_symbol_pins(
            &[SymbolPin::new("GND", PortDirection::InOut, None)],
            "Place pins",
        )
        .expect("the schematic answers")
        .refused;
    let refused = refused
        .iter()
        .map(|(name, reason)| (name.as_str(), reason.to_string()))
        .collect::<Vec<_>>();
    assert!(
        matches!(refused.as_slice(), [("GND", reason)] if reason.starts_with("port name `GND` is ")),
        "{refused:?}"
    );
    declare(
        &mut replay,
        &[
            ("IN", PortDirection::In),
            ("CLK", PortDirection::In),
            ("A[7:0]", PortDirection::In),
            ("vref_trim_1v8", PortDirection::In),
            ("OUT", PortDirection::Out),
            ("IO", PortDirection::InOut),
            ("VDD", PortDirection::Supply),
            ("VSS", PortDirection::Supply),
            ("AVSS", PortDirection::InOut),
        ],
    );
    let drawn = document(&replay.app, &symbol);
    let sides = drawn
        .pins
        .iter()
        .map(|pin| (pin.name.as_str(), pin.side))
        .collect::<Vec<_>>();
    assert_eq!(
        sides,
        [
            ("IN", SymbolPinSide::Left),
            ("CLK", SymbolPinSide::Left),
            ("A[7:0]", SymbolPinSide::Left),
            ("vref_trim_1v8", SymbolPinSide::Left),
            ("OUT", SymbolPinSide::Right),
            ("IO", SymbolPinSide::Right),
            ("VDD", SymbolPinSide::Top),
            ("VSS", SymbolPinSide::Bottom),
            ("AVSS", SymbolPinSide::Bottom),
        ]
    );
    assert_tidy(&drawn);
    let interface = replay.app.state.model.symbol_interface(&symbol);
    assert!(drawn.pin_findings(&interface).is_empty());

    open(&mut replay, &symbol);
    assert_eq!(
        replay.app.state.model.workspace.content.active_view_type(),
        Some(ViewType::Symbol)
    );
    assert!(
        !replay.app.state.model.active_view_read_only(),
        "the symbol is in front, ready to edit"
    );
    hand_edit(&mut replay, "Move pin", |document| {
        let pin = document.pins.iter_mut().find(|pin| pin.name == "IO");
        let pin = pin.expect("IO is drawn");
        pin.position = pin.position.map(|at| at + Point::new(0, 20));
    });
    assert_ne!(document(&replay.app, &symbol), drawn, "and takes an edit");
}

/// Forty ports make forty pins, each column in the order the schematic
/// declares, a terminal-grid pitch apart, none sharing a terminal.
#[test]
fn forty_ports_make_forty_pins_in_their_order() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    let names = (0..20)
        .map(|bit| format!("D{bit}"))
        .chain((0..20).map(|bit| format!("Q{bit}")))
        .collect::<Vec<_>>();
    let ports = names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let direction = if index < 20 {
                PortDirection::In
            } else {
                PortDirection::Out
            };
            (name.as_str(), direction)
        })
        .collect::<Vec<_>>();
    declare(&mut replay, &ports);
    let drawn = document(&replay.app, &symbol);
    assert_eq!(
        drawn
            .pins
            .iter()
            .map(|pin| pin.name.clone())
            .collect::<Vec<_>>(),
        names
    );
    assert_tidy(&drawn);
    for (side, column) in [
        (SymbolPinSide::Left, &drawn.pins[..20]),
        (SymbolPinSide::Right, &drawn.pins[20..]),
    ] {
        let ys = column
            .iter()
            .map(|pin| {
                assert_eq!(pin.side, side, "{}", pin.name);
                pin.position.expect("placed").y
            })
            .collect::<Vec<_>>();
        assert!(
            ys.windows(2).all(|pair| pair[1] - pair[0] == 20),
            "{side:?} pins stand one pitch apart in order: {ys:?}"
        );
    }
}

/// Each change to the schematic's ports is named by the symbol's pin check,
/// and Update pins brings a symbol drawn by hand back in step as one undo
/// step of the symbol: new pins on the edge their role takes, a changed
/// direction taken, and the drawing, the moved pin and the pins no port
/// declares left where they are.
#[test]
fn the_pin_check_names_each_port_change_and_update_pins_answers_it() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(
        &mut replay,
        &[
            ("IN", PortDirection::In),
            ("EN", PortDirection::In),
            ("OUT", PortDirection::Out),
        ],
    );
    open(&mut replay, &symbol);
    hand_edit(&mut replay, "Move pin", |document| {
        let pin = document.pins.iter_mut().find(|pin| pin.name == "OUT");
        let pin = pin.expect("OUT is drawn");
        pin.position = pin.position.map(|at| at + Point::new(0, 20));
    });
    let drawn = document(&replay.app, &symbol);

    open(&mut replay, &schematic);
    declare(&mut replay, &[("B", PortDirection::In)]);
    edit_port(&mut replay, "OUT", |port| port.value = "Y".to_owned());
    delete_port(&mut replay, "IN");
    edit_port(&mut replay, "EN", |port| {
        let mut contract = port.port_contract().expect("a typed port");
        contract.direction = PortDirection::Out;
        port.params = contract.encoded_params();
    });
    assert_eq!(
        document(&replay.app, &symbol),
        drawn,
        "a symbol drawn by hand is the author's: the schematic never redraws it"
    );

    open(&mut replay, &symbol);
    let inputs = replay
        .app
        .state
        .model
        .symbol_pin_check_inputs(&symbol)
        .expect("the check reads the symbol");
    let mut findings = rspice_design::symbol::publication::check_symbol_pins(
        &inputs.document,
        &inputs.interface,
        None,
        &symbol,
        replay.app.state.model.symbol_contract_source(&symbol),
    )
    .violations()
    .iter()
    .map(|violation| {
        let (_, name) = violation
            .message
            .rsplit_once(": ")
            .expect("each finding names its pin");
        (format!("{:?}", violation.violation_type), name.to_owned())
    })
    .collect::<Vec<_>>();
    findings.sort();
    let mut expected = [
        ("SymbolUnplacedPin", "Y"),
        ("SymbolUnplacedPin", "B"),
        ("SymbolOrphanedPin", "IN"),
        ("SymbolOrphanedPin", "OUT"),
        ("SymbolPinDirection", "EN"),
    ]
    .map(|(kind, name)| (kind.to_owned(), name.to_owned()));
    expected.sort();
    assert_eq!(findings, expected);

    replay
        .app
        .execute_command_key(DesignCommand::SymbolUpdatePinsFromContract.key());
    replay.frames(Vec::new());
    let updated = document(&replay.app, &symbol);
    assert_eq!(updated.body, drawn.body, "the drawing is untouched");
    for kept in ["IN", "OUT", "EN"] {
        assert_eq!(
            updated.pin(kept).and_then(|pin| pin.position),
            drawn.pin(kept).and_then(|pin| pin.position),
            "{kept} stays where it was drawn"
        );
    }
    assert_eq!(
        updated.pin("EN").map(|pin| pin.direction),
        Some(PortDirection::Out)
    );
    assert_eq!(
        updated.pin("B").map(|pin| pin.side),
        Some(SymbolPinSide::Left)
    );
    assert_eq!(
        updated.pin("Y").map(|pin| pin.side),
        Some(SymbolPinSide::Right)
    );
    assert_tidy(&updated);

    replay.app.execute_command_key(ids::UNDO);
    replay.frames(Vec::new());
    assert_eq!(document(&replay.app, &symbol), drawn, "one undo step");
    replay.app.execute_command_key(ids::REDO);
    replay.frames(Vec::new());
    assert_eq!(document(&replay.app, &symbol), updated, "and one redo");
}

/// Update pins keeps a pin no port declares, and the pin check's finding on
/// it says how to take it off in one step: following the finding, as a click
/// on its row does, selects the pin, and Delete removes it, which answers the
/// finding.
#[test]
fn an_orphaned_pin_goes_in_one_step_from_its_finding() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticInputs;

    let orphan = |replay: &Replay, symbol: &CellViewRef| {
        replay
            .app
            .state
            .model
            .diagnostics
            .batches()
            .find(|batch| {
                batch.inputs
                    == DiagnosticInputs::SymbolPins {
                        subject: symbol.clone(),
                    }
            })
            .and_then(|batch| {
                batch
                    .records
                    .iter()
                    .find(|record| record.canonical.code.as_ref() == "SYMBOL-ORPHANED-PIN")
                    .cloned()
            })
    };
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(
        &mut replay,
        &[("IN", PortDirection::In), ("OUT", PortDirection::Out)],
    );
    open(&mut replay, &symbol);
    hand_edit(&mut replay, "Move pin", |document| {
        let pin = document.pins.iter_mut().find(|pin| pin.name == "OUT");
        let pin = pin.expect("OUT is drawn");
        pin.position = pin.position.map(|at| at + Point::new(0, 20));
    });
    open(&mut replay, &schematic);
    delete_port(&mut replay, "OUT");
    open(&mut replay, &symbol);
    replay
        .app
        .execute_command_key(DesignCommand::SymbolUpdatePinsFromContract.key());
    replay.frames(Vec::new());
    assert!(
        document(&replay.app, &symbol).pin("OUT").is_some(),
        "Update pins keeps the pin"
    );

    replay
        .app
        .execute_command_key(DesignCommand::CheckCellview.key());
    replay.frames(Vec::new());
    let finding = orphan(&replay, &symbol).expect("the kept pin is a finding");
    assert!(
        finding.details.contains("select the pin and press Delete"),
        "the finding says how: {:?}",
        finding.details
    );
    // What a click on the finding's row does: follow its route, and give
    // the canvas it lands on the keyboard.
    let route = finding.route.clone().expect("the finding is routed");
    replay.app.state.model.diagnostics.request_route(route);
    rspice_app_workbench::workbench::app_state::session::shortcuts::request_engineering_canvas_focus(replay.ctx());
    replay.frames(Vec::new());
    assert_eq!(replay.app.state.model.workspace.content.active_view, symbol);
    assert_eq!(
        replay
            .app
            .state
            .design
            .symbol
            .editor
            .selection
            .pins
            .iter()
            .collect::<Vec<_>>(),
        ["OUT"],
        "following the finding selects the pin"
    );

    replay.key(egui::Key::Delete, egui::Modifiers::NONE);
    assert!(
        document(&replay.app, &symbol).pin("OUT").is_none(),
        "Delete removes it"
    );
    replay
        .app
        .execute_command_key(DesignCommand::CheckCellview.key());
    replay.frames(Vec::new());
    assert!(
        orphan(&replay, &symbol).is_none(),
        "the finding is answered"
    );
}

/// Update pins is named for the drawing it takes the pins from wherever it
/// shows: the toolbar button and the navigator's, the Design menu row, the
/// palette row that runs it, and the step Undo then takes back.
#[test]
fn update_pins_is_named_for_the_schematic_wherever_it_shows() {
    use egui::accesskit::Role;
    use rspice_app_workbench::workbench::framework::host::Host;

    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(&mut replay, &[("IN", PortDirection::In)]);
    open(&mut replay, &symbol);
    hand_edit(&mut replay, "Move pin", |document| {
        let pin = document.pins.iter_mut().find(|pin| pin.name == "IN");
        let pin = pin.expect("IN is drawn");
        pin.position = pin.position.map(|at| at + Point::new(0, 20));
    });
    open(&mut replay, &schematic);
    declare(&mut replay, &[("EN", PortDirection::In)]);
    open(&mut replay, &symbol);

    let label = "Update pins from schematic";
    let controls = replay.frames(Vec::new());
    let buttons = controls
        .iter()
        .filter(|control| control.role == Role::Button && control.label == label)
        .count();
    assert_eq!(buttons, 2, "the toolbar's button and the navigator's");
    assert!(
        controls
            .iter()
            .all(|control| !control.label.contains("contract")),
        "no control says contract"
    );
    let key = DesignCommand::SymbolUpdatePinsFromContract.key();
    assert_eq!(Host::menu_label(&replay.app, key), label, "the menu row");

    replay.choose_command(label);
    assert!(
        document(&replay.app, &symbol).pin("EN").is_some(),
        "the palette row ran it"
    );
    assert_eq!(
        Host::menu_label(&replay.app, ids::UNDO),
        "Undo update pins from schematic"
    );
}

/// A cell its compiled module defines takes its pins from the module, and
/// Update pins says so, from the menu row to the step Undo takes back.
#[test]
fn update_pins_is_named_for_the_module_of_a_module_defined_cell() {
    use rspice_app_state::state::{Cell, View};
    use rspice_app_workbench::workbench::framework::host::Host;

    let mut replay = new_project();
    let library = replay
        .app
        .state
        .model
        .workspace
        .content
        .active_view
        .library
        .clone();
    let symbol = CellViewRef::new(&library, "gain", "symbol");
    let mut module = View::new("veriloga", ViewType::VerilogA);
    module
        .metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    module
        .metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut drawn = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![SymbolPin::new(
            "p",
            PortDirection::InOut,
            Some(Point::new(-20, 0)),
        )],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut drawn)
    .expect("the symbol stores");
    let mut cell = Cell::new("gain");
    cell.add_view(module);
    cell.add_view(drawn);
    replay
        .app
        .state
        .model
        .library_manager
        .get_library_mut(&library)
        .expect("the project library")
        .add_cell(cell);
    open(&mut replay, &symbol);

    let key = DesignCommand::SymbolUpdatePinsFromContract.key();
    assert_eq!(
        Host::menu_label(&replay.app, key),
        "Update pins from module"
    );
    replay.app.execute_command_key(key);
    replay.frames(Vec::new());
    assert!(document(&replay.app, &symbol).pin("n").is_some());
    assert_eq!(
        Host::menu_label(&replay.app, ids::UNDO),
        "Undo update pins from module"
    );
}

/// Create schematic pins from symbol declares a port for each pin the
/// schematic lacks, with its direction, as one undo step of the schematic,
/// and leaves the symbol as it is.
#[test]
fn create_schematic_pins_declares_the_missing_ports_as_one_schematic_step() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(
        &mut replay,
        &[("IN", PortDirection::In), ("OUT", PortDirection::Out)],
    );
    open(&mut replay, &symbol);
    hand_edit(&mut replay, "Add pin", |document| {
        let (min, _) = document.body_bounds();
        document.pins.push(SymbolPin::new(
            "VDD",
            PortDirection::Supply,
            Some(Point::new(0, min.y - 10)),
        ));
    });
    let drawn = document(&replay.app, &symbol);

    replay
        .app
        .execute_command_key(DesignCommand::SymbolCreateSchematicPins.key());
    replay.frames(Vec::new());
    assert_eq!(
        document(&replay.app, &symbol),
        drawn,
        "the symbol is as drawn"
    );
    let declared = |replay: &Replay| match replay.app.state.model.symbol_interface(&symbol) {
        SymbolInterface::Declared(ports) => ports
            .into_iter()
            .map(|port| (port.name, port.direction))
            .collect::<Vec<_>>(),
        SymbolInterface::SymbolDefined => Vec::new(),
    };
    assert_eq!(
        declared(&replay),
        [
            ("IN".to_owned(), PortDirection::In),
            ("OUT".to_owned(), PortDirection::Out),
            ("VDD".to_owned(), PortDirection::Supply),
        ]
    );

    open(&mut replay, &schematic);
    replay.app.execute_command_key(ids::UNDO);
    replay.frames(Vec::new());
    assert_eq!(
        declared(&replay),
        [
            ("IN".to_owned(), PortDirection::In),
            ("OUT".to_owned(), PortDirection::Out),
        ],
        "one undo step of the schematic"
    );
}

/// Where `document` puts the terminal of its pin `name`, from its origin:
/// where that terminal is on an instance placed unturned at (0, 0).
fn terminal(document: &SymbolDocument, name: &str) -> Point {
    document
        .pin(name)
        .and_then(|pin| pin.position)
        .expect("the pin is placed")
        - document.origin
}

/// A sheet `parent` of its own, in the library of `schematic`, placing the
/// cell `schematic` draws once at (0, 0) as instance 900, with wire 950 run
/// 40 to the left from its pin `pin`. The sheet is held as the project holds
/// a sheet that is not open.
fn place_wired_instance(replay: &mut Replay, schematic: &CellViewRef, pin: &str) -> CellViewRef {
    use rspice_app_state::state::{Cell, LibraryCellInstance, View, Wire, WireConnection};

    let symbol = symbol_of(schematic);
    let at = terminal(&document(&replay.app, &symbol), pin);
    let parent = CellViewRef::new(&schematic.library, "parent", "schematic");
    let mut cell = Cell::new("parent");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    replay
        .app
        .state
        .model
        .library_manager
        .get_library_mut(&schematic.library)
        .expect("the project library")
        .add_cell(cell);
    let mut sheet = rspice_app_state::state::SchematicState::default();
    let mut binding = LibraryCellInstance::new(&schematic.library, &schematic.cell, "schematic");
    binding.bind_interface(&replay.app.state.model.symbol_ports(&symbol));
    let drawing = sheet.document_mut_for_test();
    drawing.components.push(
        Component::new(900, ComponentType::CellInstance, Point::new(0, 0))
            .with_library_cell(binding),
    );
    drawing
        .wires
        .push(Wire::segment(950, at, at - Point::new(40, 0)));
    drawing.connections.push(WireConnection {
        wire_id: 950,
        point_index: 0,
        component_id: 900,
        terminal_name: pin.to_owned(),
    });
    replay
        .app
        .state
        .model
        .workspace
        .insert_schematic_editor(parent.key(), sheet);
    parent
}

/// A placed instance of a cell whose symbol the schematic draws keeps its
/// wires on its pins as the schematic's ports change and the symbol is
/// redrawn: a pin that moves takes its wire with it.
#[test]
fn an_instance_keeps_its_wires_as_the_schematic_redraws_its_symbol() {
    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(
        &mut replay,
        &[("IN", PortDirection::In), ("OUT", PortDirection::Out)],
    );
    let at_in = terminal(&document(&replay.app, &symbol), "IN");
    let parent = place_wired_instance(&mut replay, &schematic, "IN");

    let held = |replay: &Replay| {
        let sheet = replay
            .app
            .state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&parent.key())
            .expect("the parent sheet is held")
            .document();
        (
            sheet.wires[0].points[0],
            sheet
                .connections
                .first()
                .map(|connection| connection.terminal_name.clone()),
        )
    };

    declare(&mut replay, &[("IN2", PortDirection::In)]);
    let redrawn = document(&replay.app, &symbol);
    assert_ne!(
        terminal(&redrawn, "IN"),
        at_in,
        "the second input moved IN along its side"
    );
    assert_eq!(
        held(&replay),
        (terminal(&redrawn, "IN"), Some("IN".to_owned())),
        "the wire followed IN to where it is drawn now"
    );

    edit_port(&mut replay, "IN", |port| port.value = "A_IN".to_owned());
    let renamed = document(&replay.app, &symbol);
    assert_eq!(
        held(&replay),
        (terminal(&renamed, "A_IN"), Some("A_IN".to_owned())),
        "the renamed pin keeps its wire"
    );

    let (end, _) = held(&replay);
    delete_port(&mut replay, "A_IN");
    assert!(document(&replay.app, &symbol).pin("A_IN").is_none());
    assert_eq!(
        held(&replay),
        (end, None),
        "the wire of a pin that went is left where it was, attached to nothing"
    );
}

/// A port deleted from the schematic takes its pin off the symbol the
/// schematic draws. The wire an instance held on that pin stays where it
/// was, open: no other pin takes it, and checking the sheet reports it.
#[test]
fn a_wire_a_deleted_port_let_go_is_reported_on_its_sheet() {
    use rspice_app_state::workbench::app_state::diagnostic_store::DiagnosticInputs;

    let mut replay = new_project();
    let schematic = replay.app.state.model.workspace.content.active_view.clone();
    let symbol = symbol_of(&schematic);
    declare(
        &mut replay,
        &[
            ("IN", PortDirection::In),
            ("IN2", PortDirection::In),
            ("OUT", PortDirection::Out),
        ],
    );
    let parent = place_wired_instance(&mut replay, &schematic, "IN");
    let findings = |replay: &mut Replay| {
        open(replay, &parent);
        replay
            .app
            .execute_command_key(DesignCommand::CheckCellview.key());
        replay.frames(Vec::new());
        let subject = DiagnosticInputs::DesignCheck {
            subject: parent.clone(),
        };
        let found = replay
            .app
            .state
            .model
            .diagnostics
            .batches()
            .filter(|batch| batch.inputs == subject)
            .flat_map(|batch| batch.records.iter())
            .map(|record| (record.canonical.code.to_string(), record.message.clone()))
            .collect::<Vec<_>>();
        open(replay, &schematic);
        found
    };
    let before = findings(&mut replay);

    delete_port(&mut replay, "IN");
    let redrawn = document(&replay.app, &symbol);
    assert!(redrawn.pin("IN").is_none(), "the pin went with its port");
    let sheet = replay
        .app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&parent.key())
        .expect("the sheet is held")
        .document()
        .clone();
    let end = sheet.wires[0].points[0];
    for pin in &redrawn.pins {
        assert_ne!(
            terminal(&redrawn, &pin.name),
            end,
            "{} is not drawn onto the wire IN let go",
            pin.name
        );
    }
    assert!(
        sheet
            .connections
            .iter()
            .filter(|connection| connection.wire_id == 950)
            .all(|connection| redrawn.pin(&connection.terminal_name).is_none()),
        "no pin the symbol still draws holds the wire: {:?}",
        sheet.connections
    );
    let after = findings(&mut replay);
    let reported = after
        .iter()
        .filter(|finding| !before.contains(finding))
        .collect::<Vec<_>>();
    assert!(
        !reported.is_empty(),
        "checking the sheet reports the wire end IN let go; before {before:?}, after {after:?}"
    );
}
