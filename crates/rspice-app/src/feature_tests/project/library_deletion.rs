//! Deleting a library cell or view through the whole application: the
//! review, the staged deletion and what it leaves behind, answered as the
//! dialog walk takes it.

use crate::workbench::RSpiceApp;
use rspice_app_project::workbench::project::ProjectFeature;
use rspice_app_project::workbench::project::library_dialogs::deletion::library_deletion_impact;
use rspice_app_project::workbench::project::workspace_state::{
    DeletionInstanceResolution, LibraryDeletionTarget,
};
use rspice_app_state::state::{
    Cell, CellViewRef, ComponentType, LayoutEdit, LayoutInstance, LayoutObjectId,
    LayoutOrientation, LayoutPoint, LayoutTransform, Library, LibraryCellInstance, OpenCellView,
    Point, SchematicState, View, ViewType,
};

use super::with_cx;

fn app_with_state(state: crate::workbench::app_state::AppState) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.code.automation_runtime_project_id = state.model.workspace.content.project.id();
    app.state = state;
    app
}

/// The staged deletion's step: the deletion in the feature's context,
/// then what it raised answered, as the dialog walk takes it.
fn process_pending(app: &mut RSpiceApp) {
    with_cx(app, ProjectFeature::process_pending_library_deletions);
    app.after_contribution();
}

fn state_with_open_amp_cell() -> crate::workbench::app_state::AppState {
    let mut state = crate::workbench::app_state::AppState::default();
    let mut library = Library::new("work");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("schematic", ViewType::Schematic));
    amp.add_view(View::new("symbol", ViewType::Symbol));
    library.add_cell(amp);
    let mut keep = Cell::new("keep");
    keep.add_view(View::new("schematic", ViewType::Schematic));
    library.add_cell(keep);
    state.model.library_manager.add_library(library);

    let amp_ref = CellViewRef::new("work", "amp", "schematic");
    let keep_ref = CellViewRef::new("work", "keep", "schematic");
    let mut amp_schematic = SchematicState::default();
    amp_schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
    state
        .model
        .workspace
        .insert_schematic_editor(amp_ref.key(), amp_schematic.clone());
    state
        .model
        .workspace
        .insert_schematic_editor(keep_ref.key(), SchematicState::default());
    state.model.workspace.content.open_views = vec![
        OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
        OpenCellView::new(amp_ref.clone(), ViewType::Schematic),
    ];
    state.model.workspace.content.active_view = amp_ref.clone();
    state.model.workspace.content.hierarchy_stack = vec![keep_ref, amp_ref.clone()];
    state.model.workspace.content.hierarchy_instances = vec!["XAMP".to_string()];
    state.model.schematic = amp_schematic;
    state
}

fn insert_cell_source(
    state: &mut crate::workbench::app_state::AppState,
    cell: &str,
    view: &str,
) -> rspice_app_state::state::ProjectSourceId {
    state
        .model
        .library_manager
        .get_library_mut("work")
        .and_then(|library| library.get_cell_mut(cell))
        .expect("fixture cell")
        .add_view(View::new(view, ViewType::VerilogA));
    let bundle = rspice_app_state::state::ProjectSourceBundle::try_new(
        rspice_app_state::state::ProjectSourceOwner::cell_view(CellViewRef::new(
            "work", cell, view,
        )),
        rspice_app_state::state::ProjectSourceLanguage::VerilogA,
        format!("{view}.va"),
        format!("module {view}(p, n); inout p, n; endmodule"),
        std::iter::empty(),
        std::iter::empty(),
    )
    .expect("valid source bundle");
    let id = bundle.id();
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("unique source owner");
    id
}

fn add_configuration_root(
    state: &mut crate::workbench::app_state::AppState,
    name: &str,
    root: CellViewRef,
) -> rspice_app_state::state::ConfigurationSetId {
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(rspice_app_state::state::ConfigurationSetDefinition {
            name: name.to_owned(),
            root,
            dut_path: "/".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: rspice_app_state::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                rspice_app_state::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: rspice_app_state::state::ConfigurationModelProfile::ProjectRunSetSections,
            owner: "Lifecycle test".to_owned(),
        })
        .expect("valid configuration root")
}

fn state_with_active_leaf_under_amp() -> crate::workbench::app_state::AppState {
    let mut state = crate::workbench::app_state::AppState::default();
    let mut library = Library::new("work");
    for name in ["top", "amp", "leaf"] {
        let mut cell = Cell::new(name);
        cell.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(cell);
    }
    state.model.library_manager.add_library(library);

    let top = CellViewRef::new("work", "top", "schematic");
    let amp = CellViewRef::new("work", "amp", "schematic");
    let leaf = CellViewRef::new("work", "leaf", "schematic");
    state
        .model
        .workspace
        .insert_schematic_editor(top.key(), SchematicState::default());
    state
        .model
        .workspace
        .insert_schematic_editor(amp.key(), SchematicState::default());
    state
        .model
        .workspace
        .insert_schematic_editor(leaf.key(), SchematicState::default());
    state.model.workspace.content.open_views = vec![
        OpenCellView::new(top.clone(), ViewType::Schematic),
        OpenCellView::new(amp.clone(), ViewType::Schematic),
        OpenCellView::new(leaf.clone(), ViewType::Schematic),
    ];
    state.model.workspace.content.active_view = leaf.clone();
    state.model.workspace.content.hierarchy_stack = vec![top, amp, leaf.clone()];
    state.model.workspace.content.hierarchy_instances =
        vec!["XAMP".to_string(), "XLEAF".to_string()];
    state.model.schematic = SchematicState::default();
    state
}

fn default_project_with_active_keep_cell() -> crate::workbench::app_state::AppState {
    let mut state = crate::workbench::app_state::AppState::default();
    let keep_ref = CellViewRef::new("user", "keep", "schematic");
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        let mut keep = Cell::new("keep");
        keep.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(keep);
    }
    state
        .model
        .workspace
        .insert_schematic_editor(keep_ref.key(), SchematicState::default());
    state.model.workspace.content.open_views = vec![
        OpenCellView::new(
            CellViewRef::new("user", "top", "schematic"),
            ViewType::Schematic,
        ),
        OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
    ];
    state.model.workspace.content.active_view = keep_ref.clone();
    state.model.workspace.content.hierarchy_stack = vec![keep_ref];
    state.model.workspace.content.hierarchy_instances.clear();
    state.model.schematic = SchematicState::default();
    state
}

/// Two loaded drawings placing the same master, one of them in front.
fn default_project_with_two_drawings_instancing_amp() -> crate::workbench::app_state::AppState {
    let mut state = default_project_with_active_top_instancing_amp();
    let aux_ref = CellViewRef::new("user", "aux", "schematic");
    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        let mut aux = Cell::new("aux");
        aux.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(aux);
    }
    let mut aux = SchematicState::default();
    aux.add_library_cell_component(
        Point::new(40, 40),
        LibraryCellInstance::new("user", "amp", "schematic"),
    );
    state
        .model
        .workspace
        .insert_schematic_editor(aux_ref.key(), aux);
    state
        .model
        .workspace
        .content
        .open_views
        .push(OpenCellView::new(aux_ref, ViewType::Schematic));
    state
}

/// Stage a cell deletion the way a confirmed review does: the request and
/// the answer about its placements travel together.
fn stage_cell_deletion(
    state: &mut crate::workbench::app_state::AppState,
    cell: &str,
    resolution: DeletionInstanceResolution,
) {
    state.project.pending_delete_cell = Some(("user".to_owned(), cell.to_owned()));
    state.project.library_deletion_review.resolution = Some(resolution);
}

fn default_project_with_active_top_instancing_amp() -> crate::workbench::app_state::AppState {
    let mut state = crate::workbench::app_state::AppState::default();
    let amp_ref = CellViewRef::new("user", "amp", "schematic");
    let top_ref = CellViewRef::new("user", "top", "schematic");

    if let Some(library) = state.model.library_manager.get_library_mut("user") {
        let mut amp = Cell::new("amp");
        amp.add_view(View::new("schematic", ViewType::Schematic));
        library.add_cell(amp);
    }

    let mut top = SchematicState::default();
    top.add_library_cell_component(
        Point::new(10, 10),
        LibraryCellInstance::new("user", "amp", "schematic"),
    );

    state
        .model
        .workspace
        .insert_schematic_editor(top_ref.key(), top.clone());
    state
        .model
        .workspace
        .insert_schematic_editor(amp_ref.key(), SchematicState::default());
    state.model.workspace.content.open_views =
        vec![OpenCellView::new(top_ref.clone(), ViewType::Schematic)];
    state.model.workspace.content.active_view = top_ref.clone();
    state.model.workspace.content.hierarchy_stack = vec![top_ref];
    state.model.workspace.content.hierarchy_instances.clear();
    state.model.schematic = top;
    state
}

#[test]
fn deletion_review_is_exact_revision_bound_and_only_confirmation_stages_mutation() {
    let mut state = state_with_open_amp_cell();

    state
        .project
        .open_library_cell_deletion_review(&state.model, "work", "amp")
        .expect("writable exact cell should open a deletion review");

    assert_eq!(
        state.project.library_deletion_review.target,
        Some(LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "amp".to_owned(),
        })
    );
    assert!(state.project.pending_delete_cell.is_none());
    assert!(state.project.pending_delete_view.is_none());

    state
        .project
        .confirm_library_deletion_review(&state.model, DeletionInstanceResolution::KeepUnresolved)
        .expect("unchanged reviewed cell should stage deletion");

    assert_eq!(
        state.project.pending_delete_cell,
        Some(("work".to_owned(), "amp".to_owned()))
    );
    assert!(state.project.library_deletion_review.target.is_none());
    assert!(
        state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_some(),
        "confirmation stages the existing deletion transaction; it does not mutate inline"
    );
}

#[test]
fn deletion_review_rejects_stale_and_read_only_targets_without_staging() {
    let mut state = state_with_open_amp_cell();
    state
        .project
        .open_library_view_deletion_review(&state.model, "work", "amp", "symbol")
        .expect("writable exact view should open a deletion review");
    state
        .model
        .library_manager
        .add_library(Library::new("unrelated"));

    let stale_error = state
        .project
        .confirm_library_deletion_review(&state.model, DeletionInstanceResolution::KeepUnresolved)
        .expect_err("catalog revision changes must invalidate the review");

    assert!(stale_error.contains("changed after this review opened"));
    assert!(state.project.pending_delete_view.is_none());
    assert!(state.project.library_deletion_review.target.is_some());
    state.project.library_deletion_review.close();

    state
        .model
        .library_manager
        .get_library_mut("work")
        .expect("work library exists")
        .read_only = true;
    let read_only_error = state
        .project
        .open_library_cell_deletion_review(&state.model, "work", "amp")
        .expect_err("read-only library must fail closed");
    assert!(read_only_error.contains("read-only"));
    assert!(state.project.pending_delete_cell.is_none());
    assert!(state.project.library_deletion_review.target.is_none());
}

#[test]
fn deletion_review_reports_loaded_dependents_without_double_counting_active_buffer() {
    let mut state = state_with_open_amp_cell();
    state
        .model
        .workspace
        .content
        .open_views
        .iter_mut()
        .find(|open| open.reference == CellViewRef::new("work", "amp", "schematic"))
        .expect("amp schematic is open")
        .dirty = true;
    let mut stored_keep = state
        .model
        .workspace
        .schematic_editor_mut(&CellViewRef::new("work", "keep", "schematic").key())
        .expect("keep schematic buffer exists");
    let keep = &mut stored_keep.editor;
    keep.add_library_cell_component(
        Point::origin(),
        LibraryCellInstance::new("work", "amp", "schematic"),
    );
    drop(stored_keep);
    let target = LibraryDeletionTarget::Cell {
        library: "work".to_owned(),
        cell: "amp".to_owned(),
    };

    let impact = library_deletion_impact(&state.model, &target);

    assert_eq!(impact.views, 2);
    assert_eq!(impact.open_views, 1);
    assert_eq!(impact.dirty_open_views, 1);
    assert_eq!(impact.instance_references, 1);
}

#[test]
fn deleting_open_cell_prunes_workspace_references_and_restores_valid_focus() {
    let mut app = app_with_state(state_with_open_amp_cell());
    let source_id = insert_cell_source(&mut app.state, "amp", "behavior");
    let project_revision_before = app.state.model.workspace.content.project.revision().get();
    app.state.project.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

    process_pending(&mut app);

    assert_eq!(
        app.state.model.workspace.content.project.revision().get(),
        project_revision_before + 1
    );
    assert!(matches!(
        app.state
            .model.workspace
            .content.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::DeleteCell { library, cell })
            if library == "work" && cell == "amp"
    ));
    assert!(app.state.model.workspace.content.project_metadata_dirty);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_none()
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .all(|open| { open.reference.library != "work" || open.reference.cell != "amp" })
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .hierarchy_stack
            .iter()
            .all(|reference| reference.library != "work" || reference.cell != "amp")
    );
    assert!(
        !app.state
            .model
            .workspace
            .content
            .schematic_buffers
            .contains_key("work/amp/schematic")
    );
    assert_ne!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "amp", "schematic")
    );
    assert!(
        app.state
            .model
            .workspace
            .active_context_schematic()
            .is_some()
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(source_id)
            .is_none()
    );
    assert!(app.state.model.workspace.content.project_sources_dirty);
}

#[test]
fn deleting_active_view_prunes_workspace_references_and_falls_back_to_schematic() {
    let mut state = state_with_open_amp_cell();
    let symbol_ref = CellViewRef::new("work", "amp", "symbol");
    state
        .model
        .workspace
        .content
        .open_views
        .push(OpenCellView::new(symbol_ref.clone(), ViewType::Symbol));
    state.model.workspace.content.active_view = symbol_ref.clone();
    state.model.workspace.content.hierarchy_stack = vec![symbol_ref.clone()];
    state.model.workspace.content.hierarchy_instances.clear();
    let mut app = app_with_state(state);
    app.state.project.pending_delete_view =
        Some(("work".to_string(), "amp".to_string(), "symbol".to_string()));

    process_pending(&mut app);

    assert!(matches!(
        app.state
            .model.workspace
            .content.project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(rspice_app_state::state::ProjectLibraryMutation::DeleteView {
            library,
            cell,
            view,
        }) if library == "work" && cell == "amp" && view == "symbol"
    ));
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("symbol"))
            .is_none()
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .open_views
            .iter()
            .all(|open| open.reference != symbol_ref)
    );
    assert!(
        !app.state
            .model
            .workspace
            .content
            .hierarchy_stack
            .iter()
            .any(|reference| reference == &symbol_ref)
    );
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "amp", "schematic")
    );
    assert!(
        app.state
            .model
            .workspace
            .active_context_schematic()
            .is_some()
    );
}

#[test]
fn deleting_veriloga_view_removes_only_its_owned_source_bundle() {
    let mut state = state_with_open_amp_cell();
    let removed_id = insert_cell_source(&mut state, "amp", "behavior");
    let retained_id = insert_cell_source(&mut state, "keep", "behavior");
    let mut app = app_with_state(state);
    app.state.project.pending_delete_view =
        Some(("work".to_owned(), "amp".to_owned(), "behavior".to_owned()));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(removed_id)
            .is_none()
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(retained_id)
            .is_some()
    );
    assert!(app.state.model.workspace.content.project_sources_dirty);
}

#[test]
fn deleting_cell_or_view_is_blocked_while_any_configuration_owns_the_root() {
    let mut state = state_with_open_amp_cell();
    let root = CellViewRef::new("work", "amp", "schematic");
    let active = add_configuration_root(&mut state, "Release", root.clone());
    let inactive = state
        .model
        .workspace
        .content
        .configuration_sets
        .clone_configuration(active, 1, "Characterization")
        .expect("inactive configuration");
    assert_ne!(active, inactive);
    let before_catalog = state.model.workspace.content.configuration_sets.clone();
    let mut app = app_with_state(state);

    app.state.project.pending_delete_view =
        Some(("work".to_owned(), "amp".to_owned(), "schematic".to_owned()));
    process_pending(&mut app);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .and_then(|cell| cell.get_view("schematic"))
            .is_some()
    );

    app.state.project.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
    process_pending(&mut app);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_some()
    );
    assert_eq!(
        app.state.model.workspace.content.configuration_sets,
        before_catalog
    );
}

#[test]
fn deleting_hierarchy_ancestor_resets_invalid_breadcrumb_path() {
    let leaf = CellViewRef::new("work", "leaf", "schematic");
    let mut app = app_with_state(state_with_active_leaf_under_amp());
    app.state.project.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

    process_pending(&mut app);

    assert_eq!(app.state.model.workspace.content.active_view, leaf);
    assert_eq!(
        app.state.model.workspace.content.hierarchy_stack,
        vec![leaf]
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .hierarchy_instances
            .is_empty()
    );
}

/// The project top is chosen, never guessed: deleting the top cell, or the
/// schematic it runs from, is refused with a pointer to Set as project top,
/// and nothing is removed.
#[test]
fn deleting_the_project_top_or_its_schematic_is_refused_with_a_pointer_to_set_as_project_top() {
    for view in [None, Some("schematic")] {
        let mut app = app_with_state(default_project_with_active_keep_cell());
        let target = match view {
            None => LibraryDeletionTarget::Cell {
                library: "user".to_owned(),
                cell: "top".to_owned(),
            },
            Some(view) => LibraryDeletionTarget::View {
                library: "user".to_owned(),
                cell: "top".to_owned(),
                view: view.to_owned(),
            },
        };
        app.state
            .project
            .open_library_deletions_review(&app.state.model, vec![target.clone()])
            .expect("the review opens and states the refusal");
        let refused = app
            .state
            .project
            .confirm_library_deletion_review(
                &app.state.model,
                DeletionInstanceResolution::KeepUnresolved,
            )
            .expect_err("the review does not confirm a deletion of the top");
        assert!(
            refused.contains("Set as project top"),
            "{target:?}: {refused}"
        );
        app.state.project.library_deletion_review.close();

        match view {
            None => app.state.project.pending_delete_cell = Some(("user".into(), "top".into())),
            Some(view) => {
                app.state.project.pending_delete_view =
                    Some(("user".into(), "top".into(), view.into()));
            }
        }
        process_pending(&mut app);

        assert!(
            app.state
                .model
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_some(),
            "{target:?}: the top's schematic stays"
        );
        let project = &app.state.model.workspace.content.project;
        assert_eq!(
            (project.root_library.as_str(), project.top_cell.as_str()),
            ("user", "top")
        );
        let prompt = app
            .state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.as_str());
        assert!(
            prompt.is_some_and(
                |text| text.contains("project top") && text.contains("Set as project top")
            ),
            "{target:?}: {prompt:?}"
        );
    }
}

/// Once another cell is the top, the former top deletes like any other cell:
/// the top stays the cell the reader chose, and the editor comes to rest on
/// the top's schematic rather than on a cell made up for it.
#[test]
fn a_former_top_deletes_once_another_is_chosen_and_the_chosen_top_stays() {
    let mut state = default_project_with_active_keep_cell();
    let top_ref = CellViewRef::new("user", "top", "schematic");
    state.model.workspace.content.open_views =
        vec![OpenCellView::new(top_ref.clone(), ViewType::Schematic)];
    state.model.workspace.content.active_view = top_ref.clone();
    state.model.workspace.content.hierarchy_stack = vec![top_ref];
    let mut app = app_with_state(state);
    app.state
        .model
        .set_project_top("user", "keep")
        .expect("keep has a drawing to run from");
    let original_epoch = app.state.model.design_execution_epoch;
    let cells_before = app.state.model.library_manager.total_cell_count();
    app.state.project.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("top"))
            .is_none(),
        "the former top is deleted"
    );
    assert_eq!(
        app.state.model.library_manager.total_cell_count(),
        cells_before - 1,
        "no cell is made up to rest on"
    );
    let project = &app.state.model.workspace.content.project;
    assert_eq!(
        (project.root_library.as_str(), project.top_cell.as_str()),
        ("user", "keep")
    );
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("user", "keep", "schematic")
    );
    assert_ne!(app.state.model.design_execution_epoch, original_epoch);
}

#[test]
fn deleting_instanced_non_active_cell_invalidates_runs() {
    let mut app = app_with_state(default_project_with_active_top_instancing_amp());
    let original_epoch = app.state.model.design_execution_epoch;
    stage_cell_deletion(
        &mut app.state,
        "amp",
        DeletionInstanceResolution::KeepUnresolved,
    );

    process_pending(&mut app);

    assert_eq!(app.state.model.workspace.content.active_view.cell, "top");
    assert_eq!(
        app.state.model.workspace.content.project.root_library,
        "user"
    );
    assert_eq!(app.state.model.workspace.content.project.top_cell, "top");
    assert_ne!(app.state.model.design_execution_epoch, original_epoch);
}

#[test]
fn deleting_instanced_non_active_schematic_invalidates_runs() {
    let mut app = app_with_state(default_project_with_active_top_instancing_amp());
    let original_epoch = app.state.model.design_execution_epoch;
    app.state.project.pending_delete_view = Some((
        "user".to_string(),
        "amp".to_string(),
        "schematic".to_string(),
    ));
    app.state.project.library_deletion_review.resolution =
        Some(DeletionInstanceResolution::KeepUnresolved);

    process_pending(&mut app);

    assert_eq!(app.state.model.workspace.content.active_view.cell, "top");
    assert_eq!(
        app.state.model.workspace.content.project.root_library,
        "user"
    );
    assert_eq!(app.state.model.workspace.content.project.top_cell, "top");
    assert_ne!(app.state.model.design_execution_epoch, original_epoch);
}

#[test]
fn deleting_cell_blocks_live_variant_then_removes_and_tombstones_design_management_ownership() {
    let mut state = state_with_open_amp_cell();
    let object_id = state.model.schematic.document().components[0].id;
    let owner = CellViewRef::new("work", "amp", "schematic").key();
    let sheet_id = state
        .model
        .workspace
        .content
        .design_management
        .bootstrap_for_cell_view(&owner, "Main", [object_id])
        .expect("owned sheet catalog");
    let object = rspice_app_state::state::SchematicObjectKey::new(&owner, object_id)
        .expect("scoped schematic object");
    let variant_id = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(rspice_app_state::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: rspice_app_state::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan:
                rspice_app_state::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                object.clone(),
                rspice_app_state::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-23".to_owned(),
                },
            )]),
        })
        .expect("live variant");
    let renumber_request = rspice_app_state::state::RenumberRequest {
        scope: rspice_app_state::state::RenumberScope::WholeProject,
        order: rspice_app_state::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references:
            rspice_app_state::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![rspice_app_state::state::AnnotationObject {
            object: object.clone(),
            current_reference: "R8".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(sheet_id),
            hierarchy_path: "/top".to_owned(),
            position: rspice_app_state::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = state
        .model
        .workspace
        .content
        .design_management
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    state
        .model
        .workspace
        .content
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");
    let catalog_before_blocked_delete = state.model.workspace.content.design_management.clone();
    let mut app = app_with_state(state);

    app.state.project.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_some(),
        "a live scoped variant must block the library mutation"
    );
    assert_eq!(
        app.state.model.workspace.content.design_management, catalog_before_blocked_delete,
        "blocked deletion must not partially mutate governed design state"
    );
    assert!(app.state.workbench.prompt.as_ref().is_some_and(|prompt| {
        prompt.text.contains("Cannot delete cell 'work/amp'")
            && prompt
                .text
                .contains("Design Management still references it")
    }));

    let variant_revision = app
        .state
        .model
        .workspace
        .content
        .design_management
        .variants()
        .find(variant_id)
        .expect("variant remains after blocked delete")
        .revision();
    app.state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .update(
            variant_id,
            variant_revision,
            rspice_app_state::state::AssemblyVariantDraft {
                name: "Industrial".to_owned(),
                parent_id: None,
                inheritance:
                    rspice_app_state::state::VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan:
                    rspice_app_state::state::VariantQualificationPlan::InvalidateAffectedTests,
                overrides: std::collections::BTreeMap::new(),
            },
        )
        .expect("variant reference is reviewed away");
    let annotation_journal_len = app
        .state
        .model
        .workspace
        .content
        .design_management
        .annotation()
        .journal()
        .len();

    app.state.project.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_none()
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&owner)
            .is_none()
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .journal()
            .len(),
        annotation_journal_len,
        "deletion must retain immutable reviewed annotation history"
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .effective_mapping_for(&owner, object_id)
            .expect("deleted annotation lookup")
            .is_none()
    );
    assert!(matches!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .object_authorities()
            .get(&object),
        Some(rspice_app_state::state::AnnotationObjectAuthority::Tombstone)
    ));
}

#[test]
fn deleting_cell_publishes_annotation_tombstones_without_a_sheet_catalog() {
    let mut state = state_with_open_amp_cell();
    let object_id = state.model.schematic.document().components[0].id;
    let owner = CellViewRef::new("work", "amp", "schematic").key();
    let object = rspice_app_state::state::SchematicObjectKey::new(&owner, object_id)
        .expect("scoped schematic object");
    let renumber_request = rspice_app_state::state::RenumberRequest {
        scope: rspice_app_state::state::RenumberScope::WholeProject,
        order: rspice_app_state::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references:
            rspice_app_state::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![rspice_app_state::state::AnnotationObject {
            object: object.clone(),
            current_reference: "R28".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: rspice_app_state::state::AnnotationPosition::default(),
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = state
        .model
        .workspace
        .content
        .design_management
        .annotation()
        .preview_renumbering(&renumber_request)
        .expect("renumber preview");
    state
        .model
        .workspace
        .content
        .design_management
        .annotation_mut()
        .commit_renumbering(&preview, &renumber_request)
        .expect("reviewed annotation");
    assert!(
        state
            .model
            .workspace
            .content
            .design_management
            .sheet_catalog(&owner)
            .is_none()
    );
    let journal_len = state
        .model
        .workspace
        .content
        .design_management
        .annotation()
        .journal()
        .len();
    let mut app = app_with_state(state);

    app.state.project.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_none()
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .journal()
            .len(),
        journal_len,
        "deletion must keep immutable annotation evidence"
    );
    assert!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .effective_mapping_for(&owner, object_id)
            .expect("deleted annotation lookup")
            .is_none()
    );
    assert!(matches!(
        app.state
            .model
            .workspace
            .content
            .design_management
            .annotation()
            .object_authorities()
            .get(&object),
        Some(rspice_app_state::state::AnnotationObjectAuthority::Tombstone)
    ));
}

#[test]
fn delete_cell_with_instances_requires_a_resolution() {
    let mut app = app_with_state(default_project_with_active_top_instancing_amp());
    app.state.project.pending_delete_cell = Some(("user".to_owned(), "amp".to_owned()));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .is_some(),
        "a placed master is not deleted until the reader has answered for its placements"
    );
    assert!(
        app.state.workbench.prompt.as_ref().is_some_and(|prompt| {
            prompt.text.contains("1 placement of 'user/amp' is drawn")
                && prompt.text.contains("Choose what happens to it")
        }),
        "the refusal has to say what is missing"
    );

    stage_cell_deletion(
        &mut app.state,
        "amp",
        DeletionInstanceResolution::KeepUnresolved,
    );
    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .is_none(),
        "the same request goes through once the choice travels with it"
    );
}

#[test]
fn keeping_placements_leaves_them_drawn_and_unresolved() {
    let mut state = default_project_with_active_top_instancing_amp();
    let binding = state.model.schematic.document_mut_for_test().components[0]
        .library_cell
        .as_mut()
        .expect("the fixture places amp");
    binding.module_name = Some("amp".to_owned());
    let mut app = app_with_state(state);
    stage_cell_deletion(
        &mut app.state,
        "amp",
        DeletionInstanceResolution::KeepUnresolved,
    );

    process_pending(&mut app);

    let top = app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .get(&CellViewRef::new("user", "top", "schematic").key())
        .expect("the parent drawing survives");
    assert_eq!(
        top.document().components.len(),
        1,
        "the placement stays drawn"
    );
    let binding = top.document().components[0]
        .library_cell
        .as_ref()
        .expect("it keeps naming the master it wants");
    assert_eq!(binding.cell, "amp");
    assert!(
        binding.module_name.is_none(),
        "nothing may netlist it from the identity it copied when it was placed"
    );
    assert_eq!(
        app.state
            .model
            .workspace
            .resolve_hierarchy(&app.state.model.library_manager)
            .unresolved_instances(),
        1,
        "the navigator has an unresolved placement to show"
    );
}

#[test]
fn removing_placements_across_two_drawings_undoes_as_one_step() {
    let mut app = app_with_state(default_project_with_two_drawings_instancing_amp());
    app.state.open_project_session();
    let top_key = CellViewRef::new("user", "top", "schematic").key();
    let aux_key = CellViewRef::new("user", "aux", "schematic").key();
    stage_cell_deletion(
        &mut app.state,
        "amp",
        DeletionInstanceResolution::RemoveInstances,
    );

    process_pending(&mut app);

    for key in [&top_key, &aux_key] {
        assert!(
            app.state.model.workspace.content.schematic_buffers[key]
                .document()
                .components
                .is_empty(),
            "{key} still places the deleted master"
        );
    }
    assert!(
        app.state.model.can_undo_project_design(),
        "the reader's one answer is one undoable step"
    );

    let description = app
        .state
        .undo_project_design()
        .expect("the recorded removal undoes")
        .expect("a description");

    assert!(description.contains("user/amp"));
    for key in [&top_key, &aux_key] {
        assert_eq!(
            app.state.model.workspace.content.schematic_buffers[key]
                .document()
                .components
                .len(),
            1,
            "{key} did not get its placement back"
        );
    }
    assert!(
        app.state.model.project_undo_sequence().is_none(),
        "both drawings came back on one step, not two"
    );
}

/// Each drawing the project holds, with how many objects it places.
fn drawings(app: &RSpiceApp) -> Vec<(String, usize)> {
    let mut drawings = app
        .state
        .model
        .workspace
        .content
        .schematic_buffers
        .iter()
        .map(|(key, schematic)| (key.clone(), schematic.document().components.len()))
        .collect::<Vec<_>>();
    drawings.sort();
    drawings
}

/// A staged deletion carried out through the shell ends with the object
/// and what it owned gone: a cell whose placements the reader removed from
/// two drawings, and a view that owned a Verilog-A source. Each publishes
/// one revision and says so in the Console.
#[test]
fn a_staged_deletion_removes_the_object_and_what_it_owned() {
    let mut app = app_with_state(default_project_with_two_drawings_instancing_amp());
    app.state.open_project_session();
    let receipts = app
        .state
        .model
        .workspace
        .content
        .project
        .library_mutation_audit()
        .len();
    stage_cell_deletion(
        &mut app.state,
        "amp",
        DeletionInstanceResolution::RemoveInstances,
    );
    process_pending(&mut app);
    let model = &app.state.model;
    assert!(
        model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .is_none(),
        "the cell is gone"
    );
    assert!(
        model.cell_instances("user", "amp", None).is_empty(),
        "no drawing places it"
    );
    assert_eq!(
        drawings(&app).len(),
        2,
        "both drawings stay, without the placement"
    );
    assert_eq!(
        model
            .workspace
            .content
            .project
            .library_mutation_audit()
            .len(),
        receipts + 1,
        "one revision is published"
    );
    assert_ne!(model.workspace.content.active_view.cell, "amp");
    assert!(
        model
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains("amp")),
        "the Console says what was deleted"
    );

    let mut state = state_with_open_amp_cell();
    let source = insert_cell_source(&mut state, "amp", "behavior");
    let mut app = app_with_state(state);
    app.state.project.pending_delete_view =
        Some(("work".to_owned(), "amp".to_owned(), "behavior".to_owned()));
    process_pending(&mut app);
    let content = &app.state.model.workspace.content;
    assert!(
        app.state
            .model
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("amp"))
            .is_some_and(|cell| cell.get_view("behavior").is_none()),
        "the view is gone and its cell stays"
    );
    assert!(
        content.project_sources.get_bundle(source).is_none(),
        "the view's Verilog-A source goes with it"
    );
    assert_eq!(content.project.library_mutation_audit().len(), 1);
}

/// `count` instances of `master` placed in the layout `owner`, through the
/// one way an edit reaches a layout.
fn place_in_layout(
    state: &mut crate::workbench::app_state::AppState,
    owner: &CellViewRef,
    master: &CellViewRef,
    count: i64,
) {
    let revision = state
        .model
        .workspace
        .content
        .physical_layout_document(owner)
        .expect("the layout exists")
        .revision();
    let edits = (0..count)
        .map(|index| LayoutEdit::InsertInstance {
            id: LayoutObjectId::new(),
            value: LayoutInstance {
                master: master.clone(),
                transform: LayoutTransform {
                    origin: LayoutPoint::new(index * 10_000, 0),
                    orientation: LayoutOrientation::R0,
                },
                array: None,
                terminal_bindings: Default::default(),
                properties: Default::default(),
            },
        })
        .collect::<Vec<_>>();
    state
        .model
        .apply_physical_layout_transaction(owner, revision, &edits)
        .expect("the placements commit");
}

/// user/amp with a layout, placed twice in the layout of soc/chip.
fn project_with_amp_placed_in_a_layout() -> crate::workbench::app_state::AppState {
    let mut state = default_project_with_active_keep_cell();
    let user = state
        .model
        .library_manager
        .get_library_mut("user")
        .expect("the default project's library");
    let mut amp = Cell::new("amp");
    amp.add_view(View::new("layout", ViewType::Layout));
    user.add_cell(amp);
    let mut soc = Library::new("soc");
    let mut chip = Cell::new("chip");
    chip.add_view(View::new("layout", ViewType::Layout));
    soc.add_cell(chip);
    state.model.library_manager.add_library(soc);
    state.provision_test_project_technology_contract();
    let amp_layout = CellViewRef::new("user", "amp", "layout");
    let chip_layout = CellViewRef::new("soc", "chip", "layout");
    for owner in [&amp_layout, &chip_layout] {
        state
            .model
            .initialize_physical_layout_document(owner.clone())
            .expect("the layout initializes from the project's PDK");
    }
    place_in_layout(&mut state, &chip_layout, &amp_layout, 2);
    state
}

/// A layout that places a cell counts where the cell is used, for the cell
/// and for its library, and the cell's deletion names the placements and
/// refuses until they are gone: deleting it would leave the layout placing
/// a master that no longer exists.
#[test]
fn a_layout_placement_counts_where_used_and_blocks_the_cells_deletion() {
    let state = project_with_amp_placed_in_a_layout();
    let chip_layout = CellViewRef::new("soc", "chip", "layout");

    let placed = state.model.cell_layout_placements("user", "amp", None);
    assert_eq!(
        placed
            .iter()
            .map(|usage| (usage.layout.clone(), usage.placements))
            .collect::<Vec<_>>(),
        vec![(chip_layout.clone(), 2)]
    );
    assert!(
        state
            .model
            .cell_view_consumers(&CellViewRef::new("user", "amp", "layout"))
            .contains(&chip_layout.display_path()),
        "the inspector's Used by names the layout"
    );
    let (_, outside) = state.model.library_external_placements("user");
    assert_eq!(
        outside
            .iter()
            .map(|usage| (usage.layout.clone(), usage.placements))
            .collect::<Vec<_>>(),
        vec![(chip_layout.clone(), 2)],
        "the library's Where used counts the placement made from outside it"
    );
    let (_, inside) = state.model.library_external_placements("soc");
    assert!(inside.is_empty(), "soc's own cells are placed nowhere");

    let target = LibraryDeletionTarget::Cell {
        library: "user".to_owned(),
        cell: "amp".to_owned(),
    };
    assert_eq!(
        library_deletion_impact(&state.model, &target).layout_placements,
        2
    );
    let mut app = app_with_state(state);
    app.state
        .project
        .open_library_deletions_review(&app.state.model, vec![target])
        .expect("the review opens and states the refusal");
    let refused = app
        .state
        .project
        .confirm_library_deletion_review(
            &app.state.model,
            DeletionInstanceResolution::RemoveInstances,
        )
        .expect_err("a cell a layout places is not deleted");
    assert!(
        refused.contains("2 layout placements of 'user/amp'"),
        "{refused}"
    );
    app.state.project.library_deletion_review.close();

    app.state.project.pending_delete_cell = Some(("user".into(), "amp".into()));
    app.state.project.library_deletion_review.resolution =
        Some(DeletionInstanceResolution::RemoveInstances);
    process_pending(&mut app);
    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("amp"))
            .is_some(),
        "the staged deletion is refused alike"
    );
}
