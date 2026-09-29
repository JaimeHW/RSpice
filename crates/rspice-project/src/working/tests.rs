//! Working captures preserve live edits and their exact content authority.

use super::*;
use rspice_design::library::{Cell, Library, View};
use rspice_design::schematic::{component_edit::ComponentPlacement, component_type::ComponentType};
use rspice_design_model::{Point, cell_view::CellViewRef};
use rspice_simulation_contract::{plan_model::SimulationPlan, setup_state::SimulationSetup};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Sessions {
    active: Option<String>,
    cancelled: BTreeMap<String, (u64, usize)>,
    clean: bool,
    stripped: BTreeSet<String>,
}

impl SnapshotSessions for Sessions {
    fn replace_active(&mut self, key: &str) {
        self.active = Some(key.to_owned());
    }
    fn reconcile_cancelled_operation(
        &mut self,
        key: &str,
        design: &mut Schematic,
        cancelled: Option<CancelledOperation>,
    ) {
        assert!(!self.clean);
        if let Some(cancelled) = cancelled {
            assert!(design.pending_operation_id().is_none());
            self.cancelled.insert(
                key.to_owned(),
                (cancelled.operation_id, design.document().components.len()),
            );
        }
    }
    fn mark_all_clean(&mut self) {
        self.clean = true;
    }
    fn strip_schematic_runtime(&mut self, key: &str) {
        assert!(self.clean);
        self.stripped.insert(key.to_owned());
    }
}

fn libraries(workspace: &ProjectWorkspace) -> ProjectLibraries {
    let mut library = Library::new(workspace.project.root_library.clone());
    for name in [&workspace.project.top_cell, "other"] {
        let mut cell = Cell::new(name);
        let mut view = View::new(workspace.active_view.view.clone(), ViewType::Schematic);
        view.modified = true;
        view.is_open = true;
        cell.add_view(view);
        cell.add_view(View::new("symbol", ViewType::Symbol));
        library.add_cell(cell);
    }
    let mut libraries = ProjectLibraries::default();
    libraries.add_library(library);
    libraries
}

fn execution_context() -> Result<ProjectExecutionContext, ProjectLifecycleError> {
    let mut setup = SimulationSetup::default();
    setup.analysis_plan = Some(SimulationPlan::new());
    ProjectExecutionContext::from_setup(setup, vec![], vec![], None)
        .map_err(ProjectLifecycleError::InvalidState)
}

fn pending_schematic(x: i32) -> Schematic {
    let mut design = Schematic::default();
    design.add_component(
        ComponentType::Resistor,
        ComponentPlacement {
            position: Point::new(x, 100),
            rotation: Default::default(),
            mirror_h: false,
        },
        None,
    );
    design.initialize_history();
    design.begin_operation("placement preview");
    design.add_component(
        ComponentType::Capacitor,
        ComponentPlacement {
            position: Point::new(x, 200),
            rotation: Default::default(),
            mirror_h: false,
        },
        None,
    );
    design
}

#[test]
fn committed_and_current_captures_use_active_authority_without_consuming_live_edits() {
    let mut workspace = ProjectWorkspace::default();
    workspace.project_metadata_dirty = true;
    workspace.open_views[0].dirty = true;
    let active_key = workspace.active_key();
    let other_key = CellViewRef::new(
        &workspace.project.root_library,
        "other",
        &workspace.active_view.view,
    )
    .key();
    workspace
        .schematic_buffers
        .insert(other_key.clone(), pending_schematic(200));
    let active = pending_schematic(100);
    let libraries = libraries(&workspace);
    let original = serde_json::to_value(&workspace).unwrap();
    for mode in [SnapshotContent::Current, SnapshotContent::Committed] {
        let mut sessions = Sessions::default();
        let stage = std::cell::Cell::new(0);
        let capture = ProjectWorkingSet {
            workspace: &workspace,
            active_schematic: &active,
            libraries: &libraries,
        }
        .capture(
            mode,
            &mut sessions,
            || {
                assert_eq!(stage.replace(1), 0);
                ProjectSimulationResults::default()
            },
            || {
                assert_eq!(stage.replace(2), 1);
                execution_context()
            },
            || {
                assert_eq!(stage.replace(3), 2);
                ResultPresentation::default()
            },
        )
        .unwrap();
        assert_eq!(stage.get(), 3);
        assert_eq!(sessions.active.as_deref(), Some(active_key.as_str()));
        assert_eq!(
            sessions.stripped,
            BTreeSet::from([active_key.clone(), other_key.clone()])
        );
        for (key, source) in [
            (&active_key, &active),
            (&other_key, &workspace.schematic_buffers[&other_key]),
        ] {
            let captured = &capture.workspace.schematic_buffers[key];
            match mode {
                SnapshotContent::Current => {
                    assert_eq!(
                        serde_json::to_value(captured.document()).unwrap(),
                        serde_json::to_value(source.document()).unwrap(),
                    );
                    assert_eq!(
                        captured.pending_operation_id(),
                        source.pending_operation_id()
                    );
                    assert!(sessions.cancelled.is_empty());
                }
                SnapshotContent::Committed => {
                    assert_eq!(captured.document().components.len(), 1);
                    assert_eq!(
                        captured.document().components[0],
                        source.document().components[0]
                    );
                    assert_eq!(
                        sessions.cancelled[key],
                        (source.pending_operation_id().unwrap(), 1)
                    );
                }
            }
        }
        assert!(!capture.workspace.any_dirty());
        let view = capture
            .libraries
            .get_library(&workspace.project.root_library)
            .unwrap()
            .get_cell(&workspace.project.top_cell)
            .unwrap()
            .get_view(&workspace.active_view.view)
            .unwrap();
        assert!(!view.modified && !view.is_open);
    }
    assert_eq!(serde_json::to_value(&workspace).unwrap(), original);
    assert_eq!(active.document().components.len(), 2);
    assert!(active.pending_operation_id().is_some());
    assert!(
        workspace.schematic_buffers[&other_key]
            .pending_operation_id()
            .is_some()
    );
    let view = libraries
        .get_library(&workspace.project.root_library)
        .unwrap()
        .get_cell(&workspace.project.top_cell)
        .unwrap()
        .get_view(&workspace.active_view.view)
        .unwrap();
    assert!(view.modified && view.is_open);
}

#[test]
fn symbol_capture_keeps_buffered_designs_and_stops_at_invalid_execution_inputs() {
    let mut workspace = ProjectWorkspace::default();
    let libraries = libraries(&workspace);
    let schematic_key = workspace.active_key();
    workspace.active_view.view = "symbol".to_owned();
    workspace.open_views.push(crate::OpenCellView::new(
        workspace.active_view.clone(),
        ViewType::Symbol,
    ));
    let active = pending_schematic(300);
    let mut sessions = Sessions::default();
    let captured_results = std::cell::Cell::new(false);
    let error = ProjectWorkingSet {
        workspace: &workspace,
        active_schematic: &active,
        libraries: &libraries,
    }
    .capture(
        SnapshotContent::Committed,
        &mut sessions,
        || {
            captured_results.set(true);
            ProjectSimulationResults::default()
        },
        || {
            assert!(captured_results.get());
            Err(ProjectLifecycleError::InvalidState(
                "invalid execution inputs".to_owned(),
            ))
        },
        || panic!("presentation must not be captured after invalid execution inputs"),
    )
    .unwrap_err();
    assert!(
        matches!(error, ProjectLifecycleError::InvalidState(message) if message == "invalid execution inputs")
    );
    assert!(sessions.active.is_none());
    let capture = ProjectWorkingSet {
        workspace: &workspace,
        active_schematic: &active,
        libraries: &libraries,
    }
    .capture(
        SnapshotContent::Committed,
        &mut Sessions::default(),
        ProjectSimulationResults::default,
        execution_context,
        ResultPresentation::default,
    )
    .unwrap();
    assert_eq!(capture.workspace.schematic_buffers.len(), 1);
    assert!(
        capture.workspace.schematic_buffers[&schematic_key]
            .document()
            .components
            .is_empty()
    );
    assert_eq!(active.document().components.len(), 2);
    assert!(active.pending_operation_id().is_some());
}
