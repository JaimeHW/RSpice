//! The document registry's dirty marks, against an edit that also fills the
//! design's clipboard.

use crate::state::{ComponentType, Point};
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::project_lifecycle::{ProjectDocumentId, snapshot};
use rspice_project::registry::{DocumentRegistry, document_fingerprints};

#[test]
fn presentation_and_interaction_state_never_marks_engineering_documents_dirty() {
    let mut state = AppState::default();
    let component = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(4, 6));
    let baseline = snapshot(&state).expect("baseline snapshot");
    let active = state.model.workspace.content.active_view.clone();

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(component);
    assert!(state.copy_active_schematic_selection());
    state.model.schematic.session.editor.pan = (125.0, -40.0);
    state.model.schematic.session.editor.zoom = 2.25;
    state.model.schematic.session.current_file =
        Some(std::path::PathBuf::from("presentation.rsch"));
    state.model.workspace.content.open_views[0].dirty = true;
    state.model.library_manager.filter_text = "presentation filter".to_owned();
    state.model.library_manager.show_read_only = !state.model.library_manager.show_read_only;
    state
        .model
        .library_manager
        .select_view(&active.library, &active.cell, &active.view);
    let view = state
        .model
        .library_manager
        .get_library_mut(&active.library)
        .and_then(|library| library.get_cell_mut(&active.cell))
        .and_then(|cell| cell.get_view_mut(&active.view))
        .expect("active view metadata");
    view.is_open = true;
    view.modified = true;
    view.file_path = Some(std::path::PathBuf::from("presentation-only.sch"));
    view.modified_time = Some(42);

    let current = snapshot(&state).expect("current snapshot");
    let mut registry = DocumentRegistry::default();
    registry.rebuild_from_fingerprints(
        &document_fingerprints(&current.file).unwrap(),
        Some(&document_fingerprints(&baseline.file).unwrap()),
    );
    assert!(
        registry.records().iter().all(|record| !record.dirty),
        "selection, clipboard, viewport, open-state, and browser presentation are not engineering edits"
    );

    state
        .model
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(12, 9));
    let edited = snapshot(&state).expect("edited snapshot");
    registry.rebuild_from_fingerprints(
        &document_fingerprints(&edited.file).unwrap(),
        Some(&document_fingerprints(&baseline.file).unwrap()),
    );
    assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
    assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
}
