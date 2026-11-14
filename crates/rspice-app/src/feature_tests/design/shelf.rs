//! The parts shelf drawn and armed through a whole application.

use crate::state::{ComponentType, Tool};
use crate::workbench::RSpiceApp;
use crate::workbench::design::navigator::shelf::*;

#[test]
fn palette_placement_cancels_every_unfinished_conductor_route() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .start_wire(crate::state::Point::origin());
    app.state
        .model
        .schematic
        .start_bus(crate::state::Point::new(2, 3), None)
        .unwrap();

    arm_primitive(
        &mut app.design_cx(),
        ComponentType::Resistor,
        &egui::Context::default(),
    );

    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::Resistor)
    );
    assert!(!app.state.model.schematic.session.editor.wire_drawing.active);
    assert!(!app.state.model.schematic.session.editor.bus_drawing.active);
}

#[test]
fn port_shelf_entry_uses_the_typed_place_pin_transaction() {
    let mut app = RSpiceApp::test_instance();

    arm_primitive(
        &mut app.design_cx(),
        ComponentType::Port,
        &egui::Context::default(),
    );
    // The shell runs what the shelf raised once the navigator has drawn.
    app.after_contribution();

    assert!(app.state.design.pin_port.open);
    assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Select);
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .pending_port_sequence
            .is_none()
    );
    assert!(app.state.model.schematic.document().components.is_empty());
}

/// The pin set and the placement history are per-user, cross-project state,
/// and they survive an application restart.
///
/// The boundary proved here is the real one: the session store
/// `RSpiceApp::save` writes and `RSpiceApp::new` reads back. Nothing about
/// that store is per-project: it is the user's own application-data file,
/// and the browser's local storage on the web build.
#[test]
fn the_shelf_set_survives_the_restart_boundary_the_workbench_actually_crosses() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    let mut app = RSpiceApp::test_instance();
    toggle_pin(
        &mut app.state.model,
        &ShelfEntry::Primitive(ComponentType::Diode),
    );
    toggle_pin(
        &mut app.state.model,
        &ShelfEntry::Primitive(ComponentType::Resistor),
    );
    record_placement(
        &mut app.state.model,
        &ShelfEntry::LibraryPart("1N4148".to_owned()),
    );
    record_placement(
        &mut app.state.model,
        &ShelfEntry::Cell {
            library: "work".to_owned(),
            cell: "opamp".to_owned(),
            view: "schematic".to_owned(),
        },
    );
    let saved = app.state.model.preferences.component_shelf();

    let mut storage = MemoryStorage::default();
    app.state.store_session(&mut storage);
    let restored = crate::workbench::app_state::AppState::restore_eframe_session(&storage)
        .expect("RSpice must be able to restore a session it just saved");

    assert_eq!(
        restored.model.preferences.component_shelf(),
        saved,
        "the shelf's personal set must cross the restart boundary intact"
    );
    assert_eq!(
        restored.model.preferences.component_shelf().pinned,
        Some(vec![
            ShelfEntry::Primitive(ComponentType::Capacitor).storage_key(),
            ShelfEntry::Primitive(ComponentType::Ground).storage_key(),
            ShelfEntry::Primitive(ComponentType::Diode).storage_key(),
        ]),
        "including the order the reader pinned in"
    );
}
