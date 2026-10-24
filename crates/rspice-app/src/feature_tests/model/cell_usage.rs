//! `workbench/app_state/cell_usage` observed through a whole session.

use crate::state::{CellViewRef, LibraryCellInstance, Point, SchematicState};
use crate::workbench::app_state::AppState;

/// The open schematic is read from the editor, never twice, and every
/// other schematic from its buffer; a view filter keeps only that view.
#[test]
fn instances_come_from_the_open_editor_and_every_other_buffer_once() {
    let mut state = AppState::default();
    let active = state.model.workspace.content.active_view.clone();
    let symbol = LibraryCellInstance::new("behavioral", "gain", "symbol");
    let veriloga = LibraryCellInstance::new("behavioral", "gain", "veriloga");
    state
        .model
        .schematic
        .add_library_cell_component(Point::origin(), symbol.clone());
    // A stale copy of the open schematic must not be counted beside it.
    state
        .model
        .workspace
        .insert_schematic_editor(active.key(), SchematicState::default());
    let mut bench = SchematicState::default();
    bench.add_library_cell_component(Point::origin(), veriloga);
    state
        .model
        .workspace
        .insert_schematic_editor("work/bench/schematic".to_owned(), bench);

    let every_view = state.model.cell_instances("behavioral", "gain", None);
    assert_eq!(
        every_view
            .iter()
            .map(|usage| usage.schematic.clone())
            .collect::<Vec<_>>(),
        [
            active.clone(),
            CellViewRef::new("work", "bench", "schematic")
        ]
    );
    let symbol_only = state
        .model
        .cell_instances("behavioral", "gain", Some("symbol"));
    assert_eq!(symbol_only.len(), 1);
    assert_eq!(symbol_only[0].schematic, active);
}
