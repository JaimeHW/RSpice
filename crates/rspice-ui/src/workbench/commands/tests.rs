//! Tests for command dispatch and menu availability.
//!
//! The cases here mostly assert absence: a command whose behaviour is not
//! implemented must not be offered, and one whose preconditions are unmet must
//! report unavailable rather than dispatch and fail.
//!
//! They are grouped by the surface whose commands they hold to account. This
//! file keeps only the fixtures more than one group builds on, so each group
//! can be read as the whole claim about its surface.

mod catalog;
mod code_pages;
mod design_menu;
mod drawing_sheets;
mod execution;
mod models;
mod project;
mod results;
mod schematic_editing;
mod schematic_placement;
mod sheets;
mod simulation_plan;
mod view_state;

use super::vocabulary::{CommandSpec, command_catalog};
use super::*;
use crate::state::Wire;
use crate::workbench::state::{ModelsPage, ProjectPage, SimulationPage};

fn app_with_selected_authored_symbol() -> RSpiceApp {
    use crate::state::{
        Cell, Component, Library, LibraryCellInstance, Point, PortDirection, PortSpec,
        SymbolDocument, SymbolPin, View, ViewType, Wire,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;

    let document = SymbolDocument {
        pins: vec![
            SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
            SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
        ],
        ..SymbolDocument::default()
    };
    let mut symbol_view = View::new("symbol", ViewType::Symbol);
    document
        .store_in_view(&mut symbol_view)
        .expect("authored symbol stores");
    let mut cell = Cell::new("amp");
    cell.add_view(symbol_view);
    let mut library = Library::new("command_test");
    library.add_cell(cell);
    app.state.library_manager.add_library(library);

    let interface = [
        PortSpec {
            name: "IN".to_owned(),
            direction: PortDirection::In,
        },
        PortSpec {
            name: "OUT".to_owned(),
            direction: PortDirection::Out,
        },
    ];
    let mut binding = LibraryCellInstance::new("command_test", "amp", "schematic");
    binding.bind_interface(&interface);
    app.state.schematic.components.push(
        Component::new(701, ComponentType::CellInstance, Point::new(100, 50))
            .with_library_cell(binding),
    );
    app.state
        .schematic
        .wires
        .push(Wire::segment(702, Point::new(60, 40), Point::new(60, 0)));
    app.state.schematic.selection.select_component(701);
    app
}
