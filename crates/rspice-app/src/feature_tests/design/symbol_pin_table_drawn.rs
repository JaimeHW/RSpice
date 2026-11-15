//! The pin table drawn through a whole application: the symbol's pins as
//! the engineering table's rows, edited in place, several at once.

use std::collections::{BTreeSet, HashMap};

use egui::accesskit::{Affine, NodeId, Point as AccessPoint, Role, TreeUpdate};
use egui::{Key, Modifiers, Pos2, pos2};
use rspice_schematic_editor::symbol_editor::session::SymbolSelection;

use crate::feature_tests::replay::{Control, Replay, only};
use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{
    Cell, CellViewRef, Library, Point, PortDirection, SymbolDocument, SymbolPin, SymbolPinSide,
    SymbolShape, View, ViewType,
};

/// An amplifier symbol of its own, four pins in netlist order.
fn amplifier() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut library = Library::new("work");
    let mut cell = Cell::new("amp");
    let mut view = View::new("symbol", ViewType::Symbol);
    SymbolDocument {
        pins: vec![
            SymbolPin::new("INP", PortDirection::In, Some(Point::new(-40, -10))),
            SymbolPin::new("INN", PortDirection::In, Some(Point::new(-40, 10))),
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(40, 0))),
            SymbolPin::new("VDD", PortDirection::Supply, Some(Point::new(0, -40)))
                .with_side(SymbolPinSide::Top),
        ],
        body: vec![SymbolShape::Polyline {
            points: vec![Point::new(-20, -20), Point::new(20, 0), Point::new(-20, 20)],
            closed: true,
            style: Default::default(),
        }],
        ..SymbolDocument::default()
    }
    .store_in_view(&mut view)
    .unwrap();
    cell.add_view(view);
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .open_workspace_view(CellViewRef::new("work", "amp", "symbol"));
    app.state.workbench.activate(Workspace::Design);
    app
}

fn select_pins(replay: &mut Replay, pins: &[&str]) {
    replay
        .app
        .state
        .design
        .symbol
        .editor
        .set_selection(SymbolSelection {
            pins: pins.iter().map(|pin| (*pin).to_owned()).collect(),
            ..SymbolSelection::default()
        });
}

fn selected_pins(replay: &Replay) -> BTreeSet<String> {
    replay
        .app
        .state
        .design
        .symbol
        .editor
        .effective_selection()
        .pins
}

fn document(replay: &Replay) -> SymbolDocument {
    replay
        .app
        .state
        .model
        .load_active_symbol_document()
        .unwrap()
}

/// Every control with bounds inside the dialog titled `title`, as the
/// browser observer reads them.
fn dialog_controls(tree: &TreeUpdate, title: &str) -> Vec<Control> {
    let nodes: HashMap<NodeId, &egui::accesskit::Node> =
        tree.nodes.iter().map(|(id, node)| (*id, node)).collect();
    let root = tree.tree.as_ref().expect("the whole tree").root;
    let mut found = Vec::new();
    let mut pending = vec![(root, Affine::IDENTITY, false)];
    while let Some((id, parent, inside)) = pending.pop() {
        let node = nodes[&id];
        let transform = node.transform().map_or(parent, |own| parent * *own);
        let inside = inside || (node.role() == Role::Dialog && node.label() == Some(title));
        if inside && let Some(bounds) = node.bounds() {
            let center = transform
                * AccessPoint::new((bounds.x0 + bounds.x1) / 2.0, (bounds.y0 + bounds.y1) / 2.0);
            found.push(Control {
                role: node.role(),
                label: node.label().unwrap_or_default().to_owned(),
                description: node.description().unwrap_or_default().to_owned(),
                value: node.value().unwrap_or_default().to_owned(),
                center: pos2(center.x as f32, center.y as f32),
            });
        }
        pending.extend(
            node.children()
                .iter()
                .rev()
                .map(|child| (*child, transform, inside)),
        );
    }
    found
}

/// The pin table's controls after three frames.
fn table(replay: &mut Replay) -> Vec<Control> {
    let mut tree = None;
    for _ in 0..3 {
        tree = replay.frame(Vec::new()).platform_output.accesskit_update;
    }
    let controls = dialog_controls(&tree.expect("AccessKit is on"), "Pin table");
    assert!(!controls.is_empty(), "the pin table is drawn");
    controls
}

/// The cell of the row whose name cell reads `pin`, under `heading`.
fn cell(controls: &[Control], pin: &str, heading: &str) -> Pos2 {
    let row = only(controls, pin, Role::Button).center.y;
    let headings = controls
        .iter()
        .filter(|control| control.role == Role::Label && control.value == heading)
        .collect::<Vec<_>>();
    let [heading_label] = headings.as_slice() else {
        panic!("one column is headed {heading:?}, found {}", headings.len());
    };
    let column = heading_label.center.x;
    controls
        .iter()
        .filter(|control| control.role == Role::Button && (control.center.y - row).abs() < 1.0)
        .min_by(|a, b| {
            (a.center.x - column)
                .abs()
                .total_cmp(&(b.center.x - column).abs())
        })
        .unwrap_or_else(|| panic!("{pin} has a {heading} cell"))
        .center
}

/// The check box of the row whose name cell reads `pin`: the row's own,
/// nearer its cells than any other row's.
fn check_box(controls: &[Control], pin: &str) -> Pos2 {
    let row = only(controls, pin, Role::Button).center.y;
    controls
        .iter()
        .filter(|control| control.role == Role::CheckBox && control.label.is_empty())
        .min_by(|a, b| {
            (a.center.y - row)
                .abs()
                .total_cmp(&(b.center.y - row).abs())
        })
        .filter(|check| (check.center.y - row).abs() < 8.0)
        .unwrap_or_else(|| panic!("{pin}'s row has a check box"))
        .center
}

fn press(replay: &mut Replay, at: Pos2) {
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    replay.frames(vec![egui::Event::PointerMoved(at), button(true)]);
    replay.frames(vec![button(false)]);
}

/// Rows selected on the canvas or by their check boxes take one value from
/// one cell: the side of three pins set as one undo step that says so, the
/// fourth pin untouched, and the table and the canvas selecting the same
/// pins throughout.
#[test]
fn a_cell_edited_in_the_pin_table_sets_every_selected_pin_as_one_step() {
    let mut replay = Replay::of(amplifier());
    replay.frames(Vec::new());
    let key = replay.app.state.model.workspace.content.active_key();
    select_pins(&mut replay, &["INP", "INN"]);
    replay
        .app
        .execute_command_key(DesignCommand::EngineeringTableView.key());

    let controls = table(&mut replay);
    press(&mut replay, check_box(&controls, "OUT"));
    assert_eq!(
        selected_pins(&replay),
        BTreeSet::from(["INN".to_owned(), "INP".to_owned(), "OUT".to_owned()]),
        "the row checked in the table is selected on the canvas"
    );

    // The Side cell of a selected row, edited by its list.
    let controls = table(&mut replay);
    press(&mut replay, cell(&controls, "INN", "Side"));
    let opened = replay.key(Key::F2, Modifiers::NONE);
    let bottom = only(&opened, "bottom", Role::ListBoxOption).center;
    press(&mut replay, bottom);

    let sides = document(&replay)
        .pins
        .iter()
        .map(|pin| (pin.name.clone(), pin.side))
        .collect::<Vec<_>>();
    assert_eq!(
        sides,
        [
            ("INP".to_owned(), SymbolPinSide::Bottom),
            ("INN".to_owned(), SymbolPinSide::Bottom),
            ("OUT".to_owned(), SymbolPinSide::Bottom),
            ("VDD".to_owned(), SymbolPinSide::Top),
        ]
    );
    let history = &replay.app.state.design.symbol.history;
    assert_eq!(history.undo_depth(&key), 1, "one edit, one step");
    assert_eq!(history.undo_step(&key), Some("Set side of 3 pins"));
    assert!(replay.app.state.design.engineering_table.open);
    assert_eq!(
        selected_pins(&replay).len(),
        3,
        "the edited pins stay selected"
    );

    assert!(replay.app.state.undo_active_symbol_document().unwrap());
    assert!(
        document(&replay).pins[..3]
            .iter()
            .all(|pin| pin.side != SymbolPinSide::Bottom),
        "one undo puts every pin back"
    );
}

/// A name is typed into its cell and kept with Enter, the instances
/// following the rename; Escape drops a cell's edit and leaves the table
/// open.
#[test]
fn a_name_cell_takes_typed_text_and_escape_drops_it() {
    let mut replay = Replay::of(amplifier());
    replay.frames(Vec::new());
    let key = replay.app.state.model.workspace.content.active_key();
    replay
        .app
        .execute_command_key(DesignCommand::EngineeringTableView.key());

    let controls = table(&mut replay);
    press(&mut replay, cell(&controls, "OUT", "Name"));
    replay.key(Key::F2, Modifiers::NONE);
    replay.type_text("DROPPED");
    replay.key(Key::Escape, Modifiers::NONE);
    assert!(
        replay.app.state.design.engineering_table.open,
        "Escape left the table open"
    );
    assert!(replay.app.state.design.engineering_table.pin_edit.is_none());
    assert_eq!(document(&replay).pins[2].name, "OUT");

    let controls = table(&mut replay);
    press(&mut replay, cell(&controls, "OUT", "Name"));
    replay.key(Key::F2, Modifiers::NONE);
    replay.type_text("VOUT\n");
    assert_eq!(document(&replay).pins[2].name, "VOUT");
    assert_eq!(
        replay.app.state.design.symbol.history.undo_step(&key),
        Some("Rename pin OUT to VOUT")
    );
    assert_eq!(
        selected_pins(&replay),
        BTreeSet::from(["VOUT".to_owned()]),
        "the renamed pin stays selected"
    );
}

/// The selected pins move one netlist place a press, which is the rows'
/// order: Move earlier, then Alt+Down from a cell.
#[test]
fn reordering_rows_changes_netlist_order() {
    let mut replay = Replay::of(amplifier());
    replay.frames(Vec::new());
    let key = replay.app.state.model.workspace.content.active_key();
    select_pins(&mut replay, &["VDD"]);
    replay
        .app
        .execute_command_key(DesignCommand::EngineeringTableView.key());
    table(&mut replay);

    replay.click("Move earlier", Role::Button);
    let order = |replay: &Replay| {
        document(replay)
            .pins
            .iter()
            .map(|pin| pin.name.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(order(&replay), ["INP", "INN", "VDD", "OUT"]);
    assert_eq!(
        replay.app.state.design.symbol.history.undo_step(&key),
        Some("Move pin VDD earlier in netlist order")
    );

    let controls = table(&mut replay);
    press(&mut replay, cell(&controls, "INP", "Netlist order"));
    replay.key(Key::ArrowDown, Modifiers::ALT);
    assert_eq!(order(&replay), ["INN", "INP", "VDD", "OUT"]);
}
