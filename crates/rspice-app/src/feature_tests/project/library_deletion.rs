//! Deleting a library cell or view through the whole application: the
//! review, the staged deletion and what it leaves behind, answered as the
//! dialog walk takes it.

use crate::state::{
    Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
    SchematicState, View, ViewType,
};
use crate::workbench::RSpiceApp;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::project::library_dialogs::deletion::library_deletion_impact;
use crate::workbench::project::library_dialogs::{in_front, published, shown};
use crate::workbench::project::tests::with_cx;
use crate::workbench::project::workspace_state::{
    DeletionInstanceResolution, LibraryDeletionTarget,
};
use crate::workbench::session::project_transactions::{
    apply_workspace_pruned, void_veriloga_evidence,
};

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
) -> crate::state::ProjectSourceId {
    state
        .model
        .library_manager
        .get_library_mut("work")
        .and_then(|library| library.get_cell_mut(cell))
        .expect("fixture cell")
        .add_view(View::new(view, ViewType::VerilogA));
    let bundle = crate::state::ProjectSourceBundle::try_new(
        crate::state::ProjectSourceOwner::cell_view(CellViewRef::new("work", cell, view)),
        crate::state::ProjectSourceLanguage::VerilogA,
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
) -> crate::state::ConfigurationSetId {
    state
        .model
        .workspace
        .content
        .configuration_sets
        .create(crate::state::ConfigurationSetDefinition {
            name: name.to_owned(),
            root,
            dut_path: "/top".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: crate::state::UnresolvedBindingPolicy::BlockNetlist,
            black_box_policy:
                crate::state::ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
            overrides: Vec::new(),
            cell_bindings: Vec::new(),
            model_profile: crate::state::ConfigurationModelProfile::ProjectRunSetSections,
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
        Some(crate::state::ProjectLibraryMutation::DeleteCell { library, cell })
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
        Some(crate::state::ProjectLibraryMutation::DeleteView {
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

#[test]
fn deleting_default_top_cell_does_not_recreate_deleted_design() {
    let mut app = app_with_state(crate::workbench::app_state::AppState::default());
    app.state.project.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("top"))
            .is_none(),
        "deleted default top cell should not be recreated as fallback"
    );
    assert_ne!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("user", "top", "schematic")
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
fn deleting_default_top_schematic_view_does_not_recreate_deleted_view() {
    let mut app = app_with_state(crate::workbench::app_state::AppState::default());
    app.state.project.pending_delete_view = Some((
        "user".to_string(),
        "top".to_string(),
        "schematic".to_string(),
    ));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("top"))
            .and_then(|cell| cell.get_view("schematic"))
            .is_none(),
        "deleted default top schematic view should not be recreated as fallback"
    );
    assert_ne!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("user", "top", "schematic")
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
fn deleting_non_active_project_top_cell_repoints_root_and_invalidates_runs() {
    let mut app = app_with_state(default_project_with_active_keep_cell());
    let original_epoch = app.state.model.design_execution_epoch;
    app.state.project.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("top"))
            .is_none(),
        "deleted project top cell should stay deleted"
    );
    assert_eq!(app.state.model.workspace.content.active_view.cell, "keep");
    assert_eq!(
        app.state.model.workspace.content.project.root_library,
        "user"
    );
    assert_eq!(app.state.model.workspace.content.project.top_cell, "keep");
    assert_ne!(app.state.model.design_execution_epoch, original_epoch);
}

#[test]
fn deleting_non_active_project_top_schematic_repoints_root_and_invalidates_runs() {
    let mut app = app_with_state(default_project_with_active_keep_cell());
    let original_epoch = app.state.model.design_execution_epoch;
    app.state.project.pending_delete_view = Some((
        "user".to_string(),
        "top".to_string(),
        "schematic".to_string(),
    ));

    process_pending(&mut app);

    assert!(
        app.state
            .model
            .library_manager
            .get_library("user")
            .and_then(|library| library.get_cell("top"))
            .and_then(|cell| cell.get_view("schematic"))
            .is_none(),
        "deleted project top schematic should stay deleted"
    );
    assert_eq!(app.state.model.workspace.content.active_view.cell, "keep");
    assert_eq!(
        app.state.model.workspace.content.project.root_library,
        "user"
    );
    assert_eq!(app.state.model.workspace.content.project.top_cell, "keep");
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
    let object =
        crate::state::SchematicObjectKey::new(&owner, object_id).expect("scoped schematic object");
    let variant_id = state
        .model
        .workspace
        .content
        .design_management
        .variants_mut()
        .create(crate::state::AssemblyVariantDraft {
            name: "Industrial".to_owned(),
            parent_id: None,
            inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
            qualification_plan: crate::state::VariantQualificationPlan::InvalidateAffectedTests,
            overrides: std::collections::BTreeMap::from([(
                object.clone(),
                crate::state::VariantObjectOverride::DoNotPopulate {
                    approval_reference: "ECO-23".to_owned(),
                },
            )]),
        })
        .expect("live variant");
    let renumber_request = crate::state::RenumberRequest {
        scope: crate::state::RenumberScope::WholeProject,
        order: crate::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references: crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![crate::state::AnnotationObject {
            object: object.clone(),
            current_reference: "R8".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: Some(sheet_id),
            hierarchy_path: "/top".to_owned(),
            position: crate::state::AnnotationPosition::default(),
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
            crate::state::AssemblyVariantDraft {
                name: "Industrial".to_owned(),
                parent_id: None,
                inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
                qualification_plan: crate::state::VariantQualificationPlan::InvalidateAffectedTests,
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
        Some(crate::state::AnnotationObjectAuthority::Tombstone)
    ));
}

#[test]
fn deleting_cell_publishes_annotation_tombstones_without_a_sheet_catalog() {
    let mut state = state_with_open_amp_cell();
    let object_id = state.model.schematic.document().components[0].id;
    let owner = CellViewRef::new("work", "amp", "schematic").key();
    let object =
        crate::state::SchematicObjectKey::new(&owner, object_id).expect("scoped schematic object");
    let renumber_request = crate::state::RenumberRequest {
        scope: crate::state::RenumberScope::WholeProject,
        order: crate::state::RenumberOrder::HierarchyThenCoordinates,
        protected_references: crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![crate::state::AnnotationObject {
            object: object.clone(),
            current_reference: "R28".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: crate::state::AnnotationPosition::default(),
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
        Some(crate::state::AnnotationObjectAuthority::Tombstone)
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

/// Carry out what the deletion handed over in the order the deletion
/// once did it in place.
fn carry_out_in_place(app: &mut RSpiceApp) {
    let Some(ProjectRequest::CellOrViewDeleted {
        removed_sources,
        pruned,
        mutation,
        remove_placements,
        notice,
        persist_global_veriloga,
    }) = app.state.workbench.outbox.take::<ProjectRequest>()
    else {
        panic!("the deletion hands the shell what it left");
    };
    assert!(
        !persist_global_veriloga,
        "the object is not the Verilog-A library's"
    );
    let state = &mut app.state;
    void_veriloga_evidence(state, &removed_sources);
    apply_workspace_pruned(state, pruned);
    state.model.publish_project_library_mutation(
        mutation
            .take()
            .expect("the reserved revision is published once"),
    );
    if let Some(target) = remove_placements {
        state.model.remove_deleted_master_placements(
            target.library(),
            target.cell(),
            target.view(),
            &target.display_path(),
        );
    }
    state.record(
        crate::diagnostics::LogSeverity::Info,
        crate::diagnostics::LogSource::Studio,
        notice,
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

/// Who owns each source the project keeps.
fn source_owners(app: &RSpiceApp) -> Vec<String> {
    let mut owners = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .iter_bundles()
        .map(|bundle| format!("{:?}", bundle.owner()))
        .collect::<Vec<_>>();
    owners.sort();
    owners
}

/// A staged deletion carried out in the feature's context, with what it
/// left handed to the shell, ends where the deletion carried out in place
/// did: a cell whose placements the reader removed from two drawings, and
/// a view that owned a Verilog-A source.
#[test]
fn a_staged_deletion_ends_where_the_deletion_carried_out_in_place_did() {
    let staged: [fn() -> RSpiceApp; 2] = [
        || {
            let mut app = app_with_state(default_project_with_two_drawings_instancing_amp());
            app.state.open_project_session();
            stage_cell_deletion(
                &mut app.state,
                "amp",
                DeletionInstanceResolution::RemoveInstances,
            );
            app
        },
        || {
            let mut state = state_with_open_amp_cell();
            insert_cell_source(&mut state, "amp", "behavior");
            let mut app = app_with_state(state);
            app.state.project.pending_delete_view =
                Some(("work".to_owned(), "amp".to_owned(), "behavior".to_owned()));
            app
        },
    ];
    for (fixture, staged) in staged.into_iter().enumerate() {
        let mut in_place = staged();
        with_cx(
            &mut in_place,
            ProjectFeature::process_pending_library_deletions,
        );
        carry_out_in_place(&mut in_place);

        let mut handed = staged();
        process_pending(&mut handed);

        assert_eq!(shown(&handed), shown(&in_place), "fixture {fixture}");
        assert_eq!(
            published(&handed),
            published(&in_place),
            "fixture {fixture}"
        );
        assert_eq!(in_front(&handed), in_front(&in_place), "fixture {fixture}");
        assert_eq!(drawings(&handed), drawings(&in_place), "fixture {fixture}");
        assert_eq!(
            source_owners(&handed),
            source_owners(&in_place),
            "fixture {fixture}"
        );
    }
}
