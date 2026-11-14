//! Creating a hierarchy block through the whole application: its commit is undone and
//! redone as one step of the project's design history.

use crate::state::CellViewRef;
use crate::state::{ComponentType, Point, SymbolDocument};
use crate::workbench::app::dialogs::hierarchy::create::*;
use crate::workbench::app_state::AppState;

#[test]
fn commit_publishes_synchronized_cell_and_exact_project_history() {
    let mut state = AppState::default();
    let r1 = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    let r2 = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(80, 0));
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(160, 0));
    state
        .model
        .schematic
        .add_wire(vec![Point::new(20, 0), Point::new(60, 0)]);
    state
        .model
        .schematic
        .add_wire(vec![Point::new(100, 0), Point::new(140, 0)]);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(r1);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(r2);
    let parent_ref = state.model.workspace.content.active_view.clone();
    let library_name = parent_ref.library.clone();

    open_create_hierarchy_dialog(
        &mut state.design.create_hierarchy,
        &state.model,
        &mut state.workbench,
    );
    assert!(state.design.create_hierarchy.open);
    let child = state.design.create_hierarchy.cell_name.clone();
    commit_create_hierarchy(
        &mut state.design.create_hierarchy,
        &mut state.design.symbol,
        &mut state.model,
        &state.workbench,
    )
    .expect("transaction commits");

    let target_ref = CellViewRef::new(&library_name, &child, "schematic");
    assert_eq!(state.model.workspace.content.active_view, target_ref);
    let target_cell = state
        .model
        .library_manager
        .get_library(&library_name)
        .and_then(|library| library.get_cell(&child))
        .expect("target cell");
    let symbol_view = target_cell.get_view("symbol").expect("symbol view");
    assert_eq!(
        symbol_view.metadata.get("generated").map(String::as_str),
        Some("ports")
    );
    let symbol = SymbolDocument::load_from_view(symbol_view).expect("generated symbol");
    assert_eq!(
        symbol.pins.len(),
        state.model.schematic.interface_ports().len()
    );
    assert!(
        state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&parent_ref.key())
            .expect("parent buffer")
            .document()
            .components
            .iter()
            .any(|component| component.kind == ComponentType::CellInstance)
    );

    state
        .model
        .library_manager
        .get_library_mut(&library_name)
        .and_then(|library| library.get_cell_mut(&child))
        .expect("target cell")
        .views
        .values_mut()
        .for_each(|view| view.modified = false);
    state.model.schematic.session.is_dirty = false;
    assert!(
        state.model.can_undo_project_design(),
        "saving must not disable undo"
    );
    assert_eq!(
        state.undo_project_design().expect("undo"),
        Some("create hierarchical cell".to_owned())
    );
    assert_eq!(state.model.workspace.content.active_view, parent_ref);
    assert!(
        state
            .model
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell(&child))
            .is_none()
    );
    assert!(state.model.can_redo_project_design());
    assert_eq!(
        state.redo_project_design().expect("redo"),
        Some("create hierarchical cell".to_owned())
    );
    assert_eq!(state.model.workspace.content.active_view, target_ref);
}
