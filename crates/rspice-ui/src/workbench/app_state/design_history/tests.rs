//! What design history refuses to undo.
//!
//! Each case is a guard that must fail closed rather than resolve: a modified
//! child, a read-only library, project safe mode, a dangling external master,
//! or an active focus the operator moved since the step was recorded. Undo
//! that "fixes up" any of these silently discards work the operator can still
//! see on screen.

use super::*;
use crate::state::{Cell, ComponentType, LibraryCellInstance, Point, View, ViewType};
use crate::workbench::state::LocalSafeModeOptions;

#[test]
fn library_transaction_cannot_smuggle_a_provider_ledger_change() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let approved = state
        .model_library_manager
        .load_library_bytes(
            "approved.lib",
            b".model shared NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
            None,
        )
        .expect("approved source imports");
    state
        .model_library_manager
        .load_library_bytes(
            "alternate.lib",
            b".model shared NMOS (LEVEL=1 KP=2e-3)\n".to_vec(),
            None,
        )
        .expect("alternate source imports");
    state
        .model_library_manager
        .resolve_definition_provider(
            crate::state::model_library::ModelConsumerScope::PrimitiveModel,
            "shared",
            &approved,
            "Test setup selects the approved provider.",
        )
        .expect("provider decision records");

    let mut candidate = state.model_library_manager.clone();
    assert!(candidate.clear_definition_provider(
        crate::state::model_library::ModelConsumerScope::PrimitiveModel,
        "shared"
    ));
    candidate
        .get_library_mut(&approved)
        .expect("approved library exists")
        .corners
        .insert(
            "draft".to_owned(),
            crate::state::model_library::ProcessCorner::new("draft"),
        );

    let error = publish_model_library_candidate(
        &mut state,
        candidate,
        &approved,
        "attempt mixed publication",
    )
    .expect_err("library publication must own only library state");
    assert!(
        error.contains("cannot change model provider decisions"),
        "{error}"
    );
    assert!(
        state
            .model_library_manager
            .model_resolution_record(
                crate::state::model_library::ModelConsumerScope::PrimitiveModel,
                "shared"
            )
            .is_some()
    );
}

/// The same extraction, recorded from a session that *descended* into the new
/// child rather than merely opening it, so the step spans two occurrences.
fn state_with_descended_hierarchy_record() -> (AppState, CellViewRef, CellViewRef) {
    let mut state = AppState::default();
    let parent_ref = state.workspace.active_view.clone();
    let before_parent = state.schematic.clone();
    let mut after_parent = before_parent.clone();
    after_parent.add_component(ComponentType::Resistor, Point::origin());
    let target = CellViewRef::new(&parent_ref.library, "child", "schematic");
    let mut child = SchematicState::default();
    child.add_component(ComponentType::Capacitor, Point::origin());
    let mut cell = Cell::new("child");
    cell.add_view(View::new("schematic", ViewType::Schematic));

    let open_views_before = state.workspace.open_views.clone();
    let hierarchy_stack_before = state.workspace.hierarchy_stack.clone();
    let hierarchy_instances_before = state.workspace.hierarchy_instances.clone();

    state
        .library_manager
        .get_library_mut(&target.library)
        .expect("target library")
        .add_cell(cell.clone());
    state
        .workspace
        .schematic_buffers
        .insert(parent_ref.key(), after_parent.clone());
    state
        .workspace
        .schematic_buffers
        .insert(target.key(), child.clone());
    state.schematic = child.clone();
    state
        .workspace
        .descend_into("X1".to_owned(), target.clone(), ViewType::Schematic);

    state.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
        parent_ref: parent_ref.clone(),
        target_schematic_ref: target.clone(),
        target_open_ref: target.clone(),
        before_parent,
        after_parent,
        child,
        target_cell: cell,
        open_views_before,
        hierarchy_stack_before,
        hierarchy_instances_before,
        open_views_after: state.workspace.open_views.clone(),
        hierarchy_stack_after: state.workspace.hierarchy_stack.clone(),
        hierarchy_instances_after: state.workspace.hierarchy_instances.clone(),
    });
    (state, parent_ref, target)
}

/// Undo has to restore the occurrence each document was being edited at, not
/// only which tab was in front: a breadcrumb left pointing through a cell the
/// undo removed addresses an instance that no longer exists.
#[test]
fn undo_and_redo_restore_the_occurrence_each_document_was_edited_at() {
    let (mut state, parent_ref, target) = state_with_descended_hierarchy_record();
    assert_eq!(state.workspace.occurrence_path().to_string(), "/X1");

    assert!(
        state
            .undo_project_design()
            .expect("the recorded extraction undoes")
            .is_some()
    );
    assert_eq!(state.workspace.active_view, parent_ref);
    assert!(
        state.workspace.occurrence_path().is_root(),
        "the parent is a design root again once the child it was reached through is gone"
    );

    assert!(
        state
            .redo_project_design()
            .expect("the extraction redoes")
            .is_some()
    );
    assert_eq!(state.workspace.active_view, target);
    assert_eq!(
        state.workspace.occurrence_path().to_string(),
        "/X1",
        "redo returns to the occurrence the child was being edited at"
    );
}

fn state_with_hierarchy_record() -> (AppState, CellViewRef, CellViewRef) {
    let mut state = AppState::default();
    let parent_ref = state.workspace.active_view.clone();
    let before_parent = state.schematic.clone();
    let mut after_parent = before_parent.clone();
    after_parent.add_component(ComponentType::Resistor, Point::origin());
    let target = CellViewRef::new(&parent_ref.library, "child", "schematic");
    let mut child = SchematicState::default();
    child.add_component(ComponentType::Capacitor, Point::origin());
    let mut cell = Cell::new("child");
    cell.add_view(View::new("schematic", ViewType::Schematic));

    state
        .library_manager
        .get_library_mut(&target.library)
        .expect("target library")
        .add_cell(cell.clone());
    state
        .workspace
        .schematic_buffers
        .insert(parent_ref.key(), after_parent.clone());
    state
        .workspace
        .schematic_buffers
        .insert(target.key(), child.clone());
    state.schematic = child.clone();
    state.workspace.active_view = target.clone();
    let open_views_after = state.workspace.open_views.clone();
    let hierarchy_stack_after = state.workspace.hierarchy_stack.clone();
    let hierarchy_instances_after = state.workspace.hierarchy_instances.clone();
    state.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
        parent_ref: parent_ref.clone(),
        target_schematic_ref: target.clone(),
        target_open_ref: target.clone(),
        before_parent,
        after_parent,
        child,
        target_cell: cell,
        open_views_before: state.workspace.open_views.clone(),
        hierarchy_stack_before: state.workspace.hierarchy_stack.clone(),
        hierarchy_instances_before: state.workspace.hierarchy_instances.clone(),
        open_views_after,
        hierarchy_stack_after,
        hierarchy_instances_after,
    });
    (state, parent_ref, target)
}

#[test]
fn guarded_history_refuses_to_overwrite_a_modified_child() {
    let (mut state, _, _) = state_with_hierarchy_record();
    state.schematic.components[0].value = "changed".to_owned();
    assert!(!state.can_undo_project_design());
    assert_eq!(state.undo_project_design().expect("guarded"), None);
}

#[test]
fn hierarchy_history_refuses_read_only_library_without_mutation() {
    let (mut state, _, target) = state_with_hierarchy_record();
    state
        .library_manager
        .get_library_mut(&target.library)
        .expect("target library")
        .read_only = true;

    assert!(!state.can_undo_project_design());
    assert!(state.undo_project_design().is_err());
    assert!(
        state
            .library_manager
            .get_library(&target.library)
            .and_then(|library| library.get_cell(&target.cell))
            .is_some()
    );
}

#[test]
fn hierarchy_history_refuses_project_read_only_safe_mode() {
    let (mut state, _, target) = state_with_hierarchy_record();
    state.workbench.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..LocalSafeModeOptions::default()
        },
        "retained session".to_owned(),
    );

    assert!(!state.can_undo_project_design());
    assert!(state.undo_project_design().is_err());
    assert!(
        state
            .library_manager
            .get_library(&target.library)
            .and_then(|library| library.get_cell(&target.cell))
            .is_some()
    );
}

#[test]
fn hierarchy_history_refuses_dangling_external_master_reference() {
    let (mut state, _, target) = state_with_hierarchy_record();
    let unrelated = CellViewRef::new("work", "other", "schematic");
    let mut schematic = SchematicState::default();
    let id = schematic.add_component(ComponentType::CellInstance, Point::origin());
    let component = schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("instance");
    component.library_cell = Some(LibraryCellInstance::new(
        &target.library,
        &target.cell,
        "symbol",
    ));
    state
        .workspace
        .schematic_buffers
        .insert(unrelated.key(), schematic);

    assert!(!state.can_undo_project_design());
    assert!(state.undo_project_design().is_err());
    assert!(
        state
            .library_manager
            .get_library(&target.library)
            .and_then(|library| library.get_cell(&target.cell))
            .is_some()
    );
}

#[test]
fn new_hierarchy_target_refuses_to_resolve_an_existing_dangling_instance() {
    let mut state = AppState::default();
    let target = CellViewRef::new("work", "child", "schematic");
    let id = state
        .schematic
        .add_component(ComponentType::CellInstance, Point::origin());
    state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("instance")
        .library_cell = Some(LibraryCellInstance::new("work", "child", "symbol"));

    assert!(validate_hierarchy_target_unreferenced(&state, &target).is_err());
    assert!(
        state
            .library_manager
            .get_library("work")
            .and_then(|library| library.get_cell("child"))
            .is_none()
    );
}

#[test]
fn project_undo_and_redo_preserve_parent_view_runtime() {
    let (mut state, parent, _) = state_with_hierarchy_record();
    let parent_buffer = state
        .workspace
        .schematic_buffers
        .get_mut(&parent.key())
        .expect("parent");
    parent_buffer.zoom = 2.75;
    parent_buffer.pan = (140.0, -35.0);

    assert!(state.undo_project_design().expect("undo").is_some());
    assert_eq!(state.schematic.zoom, 2.75);
    assert_eq!(state.schematic.pan, (140.0, -35.0));

    state.schematic.zoom = 1.5;
    state.schematic.pan = (-20.0, 85.0);
    state.sync_active_schematic_to_workspace();
    assert!(state.redo_project_design().expect("redo").is_some());
    let parent_buffer = state
        .workspace
        .schematic_buffers
        .get(&parent.key())
        .expect("parent");
    assert_eq!(parent_buffer.zoom, 1.5);
    assert_eq!(parent_buffer.pan, (-20.0, 85.0));
}

#[test]
fn project_undo_and_redo_preserve_child_view_runtime() {
    let (mut state, _, _) = state_with_hierarchy_record();
    state.schematic.zoom = 3.25;
    state.schematic.pan = (75.0, -120.0);

    assert!(state.undo_project_design().expect("undo").is_some());
    assert!(state.redo_project_design().expect("redo").is_some());
    assert_eq!(state.schematic.zoom, 3.25);
    assert_eq!(state.schematic.pan, (75.0, -120.0));
}

#[test]
fn project_history_preserves_unrelated_tab_dirty_state() {
    let (mut state, _, _) = state_with_hierarchy_record();
    let unrelated = OpenCellView::new(
        CellViewRef::new("work", "other", "schematic"),
        ViewType::Schematic,
    );
    state.workspace.open_views.push(unrelated.clone());
    let record = state
        .project_design_history
        .undo
        .last_mut()
        .expect("record");
    let ProjectDesignBody::HierarchyExtraction(record) = &mut record.body else {
        panic!("expected hierarchy extraction record");
    };
    record.open_views_before.push(unrelated.clone());
    record.open_views_after.push(unrelated);
    state
        .workspace
        .open_views
        .last_mut()
        .expect("unrelated tab")
        .dirty = true;

    assert!(state.undo_project_design().expect("undo").is_some());
    assert!(
        state
            .workspace
            .open_views
            .iter()
            .find(|view| view.reference.cell == "other")
            .expect("restored unrelated tab")
            .dirty
    );
}

/// A record names the document it restores, so undo brings that tab forward
/// instead of refusing because the operator moved on — and says so, because a
/// step that rewrites a background document without a word is a step the
/// operator cannot review.
#[test]
fn undo_activates_the_document_its_compensation_names_and_says_so() {
    let (mut state, parent, _) = state_with_hierarchy_record();
    assert_ne!(state.workspace.active_view, parent);

    assert!(state.can_undo_project_design());
    assert!(state.undo_project_design().expect("undo").is_some());

    assert_eq!(state.workspace.active_view, parent);
    let announcement = format!("Undo switched to {}", parent.display_path());
    assert!(
        state
            .log_buffer
            .entries()
            .any(|entry| entry.message.contains(&announcement)),
        "an undo that moves the operator has to say where it went"
    );
}

/// A record states which documents it restores, so navigation, sheet
/// restoration and review all read the same list instead of re-deriving it
/// from whichever variant the record happens to be.
#[test]
fn an_extraction_names_both_documents_it_restores() {
    let (state, parent, target) = state_with_hierarchy_record();
    let record = state
        .project_design_history
        .undo
        .last()
        .expect("the recorded extraction");
    let named = record
        .header
        .documents()
        .iter()
        .map(|document| document.reference().clone())
        .collect::<Vec<_>>();
    assert_eq!(named, vec![parent, target]);
}

/// Every project transaction takes a position in the one undo order when it
/// commits, and takes a fresh one each time it crosses between the stacks —
/// Undo asks which step moved last, not which was authored first.
#[test]
fn project_records_are_stamped_and_restamped_as_they_cross_the_stacks() {
    let (mut state, _, _) = state_with_hierarchy_record();
    let committed = state.project_undo_sequence().expect("a committed record");
    assert_ne!(committed, 0, "a committed record is never unstamped");

    assert!(state.undo_project_design().expect("undo").is_some());
    let undone = state
        .project_redo_sequence()
        .expect("the record moved to the redo stack");
    assert!(undone > committed);

    assert!(state.redo_project_design().expect("redo").is_some());
    assert!(
        state
            .project_undo_sequence()
            .expect("the record moved back")
            > undone
    );
}

/// Undo puts a restored object back on the sheet it was drawn on. Landing it
/// on whichever sheet happens to be active would redraw the design.
#[test]
fn undo_restores_objects_to_the_sheet_they_were_recorded_on() {
    use crate::state::{SheetDefinition, SheetPortPolicy, SheetTemplate};

    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let parent_ref = state.workspace.active_view.clone();
    let kept = state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    let extracted = state
        .schematic
        .add_component(ComponentType::Capacitor, Point::new(40, 0));
    let key = parent_ref.key();
    let first = state
        .workspace
        .design_management
        .bootstrap_for_cell_view(&key, "Sheet 1", [kept, extracted])
        .expect("first sheet");
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    let second = catalog
        .create_sheet(
            SheetDefinition {
                name: "Sheet 2".to_owned(),
                template: SheetTemplate::AnalogSchematic,
                port_policy: SheetPortPolicy::TypedOffSheetPorts,
                explicit_page_number: Some(2),
            },
            Some(first),
        )
        .expect("second sheet");
    catalog
        .assign_objects(catalog.revision(), second, [extracted])
        .expect("the extracted object is drawn on the second sheet");

    let before_parent = state.schematic.clone();
    let mut after_parent = before_parent.clone();
    after_parent
        .components
        .retain(|component| component.id != extracted);
    let target = CellViewRef::new(&parent_ref.library, "child", "schematic");
    let mut child = SchematicState::default();
    child.add_component(ComponentType::Capacitor, Point::origin());
    let mut cell = Cell::new("child");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    state
        .library_manager
        .get_library_mut(&target.library)
        .expect("target library")
        .add_cell(cell.clone());
    state
        .workspace
        .schematic_buffers
        .insert(parent_ref.key(), after_parent.clone());
    state
        .workspace
        .schematic_buffers
        .insert(target.key(), child.clone());
    state.schematic = child.clone();
    state.workspace.active_view = target.clone();
    state.record_hierarchy_extraction(HierarchyExtractionHistoryEntry {
        parent_ref: parent_ref.clone(),
        target_schematic_ref: target.clone(),
        target_open_ref: target.clone(),
        before_parent,
        after_parent,
        child,
        target_cell: cell,
        open_views_before: state.workspace.open_views.clone(),
        hierarchy_stack_before: state.workspace.hierarchy_stack.clone(),
        hierarchy_instances_before: state.workspace.hierarchy_instances.clone(),
        open_views_after: state.workspace.open_views.clone(),
        hierarchy_stack_after: state.workspace.hierarchy_stack.clone(),
        hierarchy_instances_after: state.workspace.hierarchy_instances.clone(),
    });

    // Extraction leaves the catalog holding only what the parent still draws.
    let catalog = state
        .workspace
        .design_management
        .sheet_catalog_mut(&key)
        .expect("sheet catalog");
    catalog
        .reconcile_object_assignments(catalog.revision(), [kept], Some(first))
        .expect("the extracted object leaves the catalog with the design");
    assert_eq!(
        state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .expect("sheet catalog")
            .sheet_for_object(extracted),
        None
    );

    assert!(state.undo_project_design().expect("undo").is_some());

    assert_eq!(
        state
            .workspace
            .design_management
            .sheet_catalog(&key)
            .expect("sheet catalog")
            .sheet_for_object(extracted),
        Some(second),
        "the restored object returns to its own sheet, not the active one"
    );
}

fn state_with_design_management_record()
-> (AppState, DesignManagementCatalog, DesignManagementCatalog) {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let owner = state.workspace.active_schematic_reference();
    let before = state.workspace.design_management.clone();
    let mut candidate = before.clone();
    candidate
        .bootstrap_for_cell_view(&owner.key(), "Main", [1])
        .expect("bootstrap reviewed sheet catalog");
    let committed_revision = state
        .workspace
        .replace_design_management(candidate)
        .expect("publish reviewed catalog");
    let after = state.workspace.design_management.clone();
    state.record_design_management_transaction(DesignManagementHistoryEntry {
        description: "apply reviewed design-management changes".to_owned(),
        owner,
        before: before.clone(),
        after: after.clone(),
        before_schematics: BTreeMap::new(),
        after_schematics: BTreeMap::new(),
        committed_revision,
    });
    (state, before, after)
}

#[test]
fn design_management_history_round_trips_semantics_with_monotonic_revisions() {
    let (mut state, before, after) = state_with_design_management_record();
    let committed_revision = state.workspace.project.revision();
    let initial_epoch = state.design_execution_epoch;
    assert!(state.can_undo_project_design());

    assert_eq!(
        state.undo_project_design().expect("undo"),
        Some("apply reviewed design-management changes".to_owned())
    );
    assert!(design_management_semantics_match(
        &state.workspace.design_management,
        &before
    ));
    assert!(state.workspace.project.revision() > committed_revision);
    let undo_revision = state.workspace.project.revision();
    assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(1));
    assert!(state.can_redo_project_design());

    assert!(state.redo_project_design().expect("redo").is_some());
    assert!(design_management_semantics_match(
        &state.workspace.design_management,
        &after
    ));
    assert!(state.workspace.project.revision() > undo_revision);
    assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(2));
}

#[test]
fn design_management_history_fails_closed_after_external_project_revision_change() {
    let (mut state, _before, after) = state_with_design_management_record();
    state
        .workspace
        .project
        .advance_revision()
        .expect("external project edit revision");

    assert!(!state.can_undo_project_design());
    assert_eq!(state.undo_project_design().expect("guarded"), None);
    assert!(design_management_semantics_match(
        &state.workspace.design_management,
        &after
    ));
}

#[test]
fn design_management_history_refuses_read_only_project_without_mutation() {
    let (mut state, _before, after) = state_with_design_management_record();
    state.workbench.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..LocalSafeModeOptions::default()
        },
        "retained session".to_owned(),
    );

    assert!(!state.can_undo_project_design());
    assert!(state.undo_project_design().is_err());
    assert!(design_management_semantics_match(
        &state.workspace.design_management,
        &after
    ));
}

#[test]
fn annotation_publish_and_history_update_the_scoped_schematic_atomically() {
    use crate::state::{
        AnnotationObject, AnnotationPosition, ProtectedReferencePolicy, RenumberOrder,
        RenumberRequest, RenumberScope, SchematicObjectKey,
    };

    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let owner = state.workspace.active_schematic_reference();
    let object_id = state
        .schematic
        .add_component(ComponentType::Resistor, Point::origin());
    state
        .schematic
        .components
        .iter_mut()
        .find(|component| component.id == object_id)
        .expect("component")
        .name = "R42".to_owned();
    state.sync_active_schematic_to_workspace();

    let before_catalog = state.workspace.design_management.clone();
    let mut draft = before_catalog.clone();
    let request = RenumberRequest {
        scope: RenumberScope::WholeProject,
        order: RenumberOrder::HierarchyThenCoordinates,
        protected_references: ProtectedReferencePolicy::RetainLockedAndExternalIds,
        protected_reviewed: false,
        objects: vec![AnnotationObject {
            object: SchematicObjectKey::new(&owner.key(), object_id).expect("scoped object"),
            current_reference: "R42".to_owned(),
            device_family: "R".to_owned(),
            sheet_id: None,
            hierarchy_path: "/top".to_owned(),
            position: AnnotationPosition { x: 0, y: 0 },
            connectivity_order: Some(1),
            locked: false,
            external: false,
            imported: false,
        }],
    };
    let preview = draft
        .annotation()
        .preview_renumbering(&request)
        .expect("preview");
    let expected_reference = preview
        .mappings
        .values()
        .next()
        .expect("annotation mapping")
        .new_reference
        .clone();
    draft
        .annotation_mut()
        .commit_renumbering(&preview, &request)
        .expect("journal commit");
    let schematic_tx = state
        .prepare_design_management_schematic_transaction(&draft)
        .expect("preflight schematic annotation");
    let committed_revision = state
        .workspace
        .replace_design_management(draft)
        .expect("publish catalog");
    state.apply_design_management_schematic_transaction(&schematic_tx);
    let after_catalog = state.workspace.design_management.clone();
    state.record_design_management_transaction(DesignManagementHistoryEntry {
        description: "renumber schematic references".to_owned(),
        owner,
        before: before_catalog,
        after: after_catalog,
        before_schematics: schematic_tx.before,
        after_schematics: schematic_tx.after,
        committed_revision,
    });

    let reference = |state: &AppState| {
        state
            .schematic
            .components
            .iter()
            .find(|component| component.id == object_id)
            .expect("annotated component")
            .name
            .clone()
    };
    assert_eq!(reference(&state), expected_reference);
    assert!(state.undo_project_design().expect("undo").is_some());
    assert_eq!(reference(&state), "R42");
    assert!(state.redo_project_design().expect("redo").is_some());
    assert_eq!(reference(&state), expected_reference);
}

#[test]
fn symbol_definition_candidate_is_atomic_and_globally_undoable() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let mut candidate = state.library_manager.clone();
    let mut cell = Cell::new("imported_symbol");
    let mut view = View::new("symbol", ViewType::Symbol);
    view.metadata
        .insert("rspice.symbol.test".to_owned(), "revision-1".to_owned());
    cell.add_view(view);
    candidate
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(cell);

    let committed = publish_symbol_definition_candidate(
        &mut state,
        candidate,
        &library_name,
        "imported_symbol",
        "import symbol definition",
    )
    .expect("publish");

    assert_eq!(state.workspace.project.revision(), committed);
    assert!(state.workspace.project_metadata_dirty);
    assert!(state.can_undo_project_design());
    assert!(state.undo_project_design().expect("undo").is_some());
    assert!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("imported_symbol"))
            .is_none()
    );
    assert!(state.redo_project_design().expect("redo").is_some());
    assert!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("imported_symbol"))
            .is_some()
    );
}

/// The binding one placement of a project cell currently carries.
fn placed_binding<'a>(
    state: &'a AppState,
    document: &CellViewRef,
    object: u64,
) -> &'a LibraryCellInstance {
    state
        .workspace
        .schematic_buffers
        .get(&document.key())
        .expect("the buffer that placed the master")
        .components
        .iter()
        .find(|component| component.id == object)
        .expect("the placed instance")
        .library_cell
        .as_ref()
        .expect("a placement keeps its binding even when its master is gone")
}

/// A published cell is undone from a document that never placed it, while a
/// second buffer holds an instance of it. That buffer is not named by the
/// record, so nothing in the transaction re-checks it: without the sweep it
/// keeps the copy of the master's netlist identity it was placed with and goes
/// on netlisting a cell the project no longer holds.
#[test]
fn undoing_a_publish_unresolves_the_placements_it_leaves_behind() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let mut candidate = state.library_manager.clone();
    let mut cell = Cell::new("published_symbol");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    candidate
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(cell);
    publish_symbol_definition_candidate(
        &mut state,
        candidate,
        &library_name,
        "published_symbol",
        "publish symbol definition",
    )
    .expect("publish");

    let elsewhere = CellViewRef::new(&library_name, "elsewhere", "schematic");
    let mut buffer = SchematicState::default();
    let mut binding = LibraryCellInstance::new(&library_name, "published_symbol", "symbol");
    binding.module_name = Some("published_symbol".to_owned());
    binding.source_path = Some(std::path::PathBuf::from("published_symbol.va"));
    binding.netlist_template = Some("X{name} {nodes} {model}".to_owned());
    binding.parameter_order = vec!["w".to_owned()];
    let placed = buffer.add_library_cell_component(Point::origin(), binding);
    state
        .workspace
        .schematic_buffers
        .insert(elsewhere.key(), buffer);
    assert_ne!(state.workspace.active_schematic_reference(), elsewhere);

    assert!(state.undo_project_design().expect("undo").is_some());

    let stranded = placed_binding(&state, &elsewhere, placed);
    assert_eq!(stranded.module_name, None);
    assert_eq!(stranded.source_path, None);
    assert_eq!(stranded.netlist_template, None);
    assert!(stranded.parameter_order.is_empty());
    // Undoing a publish is not a licence to edit a drawing the step never
    // named: the placement stays, and stays pointed at the master it wants.
    assert_eq!(stranded.library, library_name);
    assert_eq!(stranded.cell, "published_symbol");

    assert!(state.redo_project_design().expect("redo").is_some());

    let resolved = placed_binding(&state, &elsewhere, placed);
    assert_eq!(resolved.module_name.as_deref(), Some("published_symbol"));
    assert_eq!(
        resolved.source_path,
        Some(std::path::PathBuf::from("published_symbol.va"))
    );
    assert_eq!(
        resolved.netlist_template.as_deref(),
        Some("X{name} {nodes} {model}")
    );
    assert_eq!(resolved.parameter_order, vec!["w".to_owned()]);
}

fn owned_model_definition(vth0: f64) -> ProjectModelDefinition {
    ProjectModelDefinition {
        name: "history_nch".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "History integration model".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), vth0),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    }
}

#[test]
fn project_model_publication_is_atomic_dirty_and_globally_undoable() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    state.model_library_manager.filter_text = "nch".to_owned();
    let initial_revision = state.workspace.project.revision();
    let initial_epoch = state.design_execution_epoch;
    let mut candidate = state.model_library_manager.clone();
    let commit = candidate
        .create_project_model("history-models", &owned_model_definition(0.48))
        .expect("candidate model validates");

    let committed_revision = state
        .publish_project_model_candidate(candidate, commit, "create project model history_nch")
        .expect("candidate publishes");
    assert!(committed_revision > initial_revision);
    assert!(state.workspace.project_metadata_dirty);
    assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(1));
    assert_eq!(state.model_library_manager.filter_text, "nch");
    assert!(
        state
            .model_library_manager
            .get_library("history-models")
            .is_some()
    );
    assert!(state.can_undo_project_design());

    assert_eq!(
        state.undo_project_design().expect("undo"),
        Some("create project model history_nch".to_owned())
    );
    assert!(
        state
            .model_library_manager
            .get_library("history-models")
            .is_none()
    );
    assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(2));
    let undo_revision = state.workspace.project.revision();
    assert!(undo_revision > committed_revision);
    assert!(state.can_redo_project_design());

    assert_eq!(
        state.redo_project_design().expect("redo"),
        Some("create project model history_nch".to_owned())
    );
    assert!(
        state
            .model_library_manager
            .get_library("history-models")
            .is_some()
    );
    assert_eq!(state.design_execution_epoch, initial_epoch.wrapping_add(3));
    assert!(state.workspace.project.revision() > undo_revision);
    assert_eq!(state.model_library_manager.filter_text, "nch");
}

#[test]
fn project_model_correlation_publication_is_history_guarded() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let mut model_candidate = state.model_library_manager.clone();
    let model_commit = model_candidate
        .create_project_model("history-models", &owned_model_definition(0.48))
        .expect("candidate model validates");
    state
        .publish_project_model_candidate(
            model_candidate,
            model_commit,
            "create project model history_nch",
        )
        .expect("model publishes");

    let library = state
        .model_library_manager
        .get_library("history-models")
        .expect("project model library");
    let ModelSourceAuthority::ProjectOwned {
        source_id,
        revision: library_revision,
        ..
    } = library.source_authority
    else {
        panic!("history model must be project-owned");
    };
    let definition = ProjectModelRevisionDefinition::new(
        ProjectModelDefinition::from_device_model(&library.models["history_nch"]),
        library.model_definition_metadata["history_nch"].clone(),
    );
    let model_identity = definition
        .project_source_identity()
        .expect("valid project source identity")
        .expect("bound project source identity");
    let source_binding = ModelSourceEvidenceBinding::try_new_project_bound(
        "history_nch",
        source_id,
        model_identity.content_digest,
        model_identity.revision,
    )
    .expect("valid correlation source binding");
    let dataset = CorrelationDatasetRevision::try_from_csv(
        "bench-reference",
        ObjectRevision::INITIAL,
        "Bench reference",
        CorrelationDatasetClass::BenchMeasurement,
        "test authority",
        "lot-1",
        "fixture-1",
        "calibration-1",
        "reference.csv",
        b"id,quantity,value,unit\nr1,gain,1,V\n".to_vec(),
        None,
    )
    .expect("valid retained correlation dataset");
    let suite = CorrelationSuite::try_new(
        "history-correlation",
        ObjectRevision::INITIAL,
        "History correlation",
        "model-owner",
        source_binding,
        vec![dataset],
        Vec::new(),
        Vec::new(),
    )
    .expect("valid correlation suite");
    let correlation =
        ModelCorrelationState::try_new(vec![suite], Vec::new()).expect("valid correlation");
    let mut correlation_candidate = state.model_library_manager.clone();
    let correlation_commit = correlation_candidate
        .replace_project_model_correlation(
            "history-models",
            source_id,
            library_revision,
            model_identity.revision,
            model_identity.content_digest,
            "history_nch",
            &correlation,
        )
        .expect("correlation candidate validates");
    assert!(!correlation_commit.affects_execution);

    state
        .publish_project_model_candidate(
            correlation_candidate,
            correlation_commit,
            "publish history_nch correlation",
        )
        .expect("correlation-only publication must not be rejected as a no-op");
    assert!(state.can_undo_project_design());
    assert_eq!(
        state.undo_project_design().expect("undo correlation"),
        Some("publish history_nch correlation".to_owned())
    );
    assert!(
        !state
            .model_library_manager
            .get_library("history-models")
            .expect("library remains after correlation undo")
            .model_correlation
            .contains_key("history_nch")
    );
    assert_eq!(
        state.redo_project_design().expect("redo correlation"),
        Some("publish history_nch correlation".to_owned())
    );

    state
        .model_library_manager
        .get_library_mut("history-models")
        .expect("project model library")
        .model_correlation
        .remove("history_nch");
    assert!(!state.can_undo_project_design());
    assert_eq!(state.undo_project_design().expect("guarded undo"), None);
}

#[test]
fn project_model_publication_rejects_closed_or_read_only_projects_without_mutation() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = false;
    let mut candidate = state.model_library_manager.clone();
    let commit = candidate
        .create_project_model("history-models", &owned_model_definition(0.48))
        .expect("candidate model validates");
    let revision = state.workspace.project.revision();
    let error = state
        .publish_project_model_candidate(candidate.clone(), commit.clone(), "create project model")
        .expect_err("closed project must reject publication");
    assert!(error.contains("open project"));
    assert_eq!(state.workspace.project.revision(), revision);
    assert!(
        state
            .model_library_manager
            .get_library("history-models")
            .is_none()
    );

    state.project_lifecycle.project_open = true;
    state.workbench.safe_mode.activate(
        LocalSafeModeOptions {
            open_project_read_only: true,
            ..LocalSafeModeOptions::default()
        },
        "retained session".to_owned(),
    );
    let error = state
        .publish_project_model_candidate(candidate, commit, "create project model")
        .expect_err("read-only project must reject publication");
    assert!(error.contains("read-only"));
    assert_eq!(state.workspace.project.revision(), revision);
    assert!(
        state
            .model_library_manager
            .get_library("history-models")
            .is_none()
    );
}

#[test]
fn symbol_definition_history_fails_closed_after_external_cell_edit() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let mut candidate = state.library_manager.clone();
    let mut cell = Cell::new("imported_symbol");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    candidate
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(cell);
    publish_symbol_definition_candidate(
        &mut state,
        candidate,
        &library_name,
        "imported_symbol",
        "import symbol definition",
    )
    .expect("publish");

    state
        .library_manager
        .get_library_mut(&library_name)
        .and_then(|library| library.get_cell_mut("imported_symbol"))
        .expect("imported cell")
        .description = "external edit".to_owned();

    assert!(!state.can_undo_project_design());
    assert_eq!(state.undo_project_design().expect("guarded"), None);
    assert_eq!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("imported_symbol"))
            .expect("cell retained")
            .description,
        "external edit"
    );
}

#[test]
fn symbol_definition_and_generated_fixture_share_one_history_record() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let fixture_ref = CellViewRef::new(&library_name, "fixture_symbol", "testbench");
    let mut fixture = SchematicState::default();
    fixture.add_component(ComponentType::Resistor, Point::origin());
    let mut candidate = state.library_manager.clone();
    let mut cell = Cell::new("fixture_symbol");
    cell.add_view(View::new("symbol", ViewType::Symbol));
    cell.add_view(View::new("testbench", ViewType::Testbench));
    candidate
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(cell);

    publish_symbol_definition_candidate_with_fixture(
        &mut state,
        candidate,
        &library_name,
        "fixture_symbol",
        "create symbol with fixture",
        Some(SymbolDefinitionFixtureDelta {
            reference: fixture_ref.clone(),
            before: None,
            after: Some(fixture.clone()),
        }),
    )
    .expect("publish symbol and fixture");
    assert!(
        state
            .workspace
            .schematic_buffers
            .get(&fixture_ref.key())
            .is_some_and(|stored| SchematicSnapshot::capture(&fixture).is_equal_state(stored))
    );

    assert!(state.undo_project_design().expect("undo").is_some());
    assert!(
        !state
            .workspace
            .schematic_buffers
            .contains_key(&fixture_ref.key())
    );
    assert!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("fixture_symbol"))
            .is_none()
    );

    assert!(state.redo_project_design().expect("redo").is_some());
    assert!(
        state
            .workspace
            .schematic_buffers
            .contains_key(&fixture_ref.key())
    );
    assert!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("fixture_symbol"))
            .is_some()
    );
}

#[test]
fn symbol_history_refuses_to_remove_an_open_generated_fixture() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let fixture_ref = CellViewRef::new(&library_name, "existing_symbol", "testbench");
    state
        .library_manager
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(Cell::new("existing_symbol"));

    let mut candidate = state.library_manager.clone();
    candidate
        .get_library_mut(&library_name)
        .and_then(|library| library.get_cell_mut("existing_symbol"))
        .expect("existing cell")
        .add_view(View::new("testbench", ViewType::Testbench));
    let mut fixture = SchematicState::default();
    fixture.add_component(ComponentType::Capacitor, Point::origin());
    publish_symbol_definition_candidate_with_fixture(
        &mut state,
        candidate,
        &library_name,
        "existing_symbol",
        "add generated fixture",
        Some(SymbolDefinitionFixtureDelta {
            reference: fixture_ref.clone(),
            before: None,
            after: Some(fixture),
        }),
    )
    .expect("publish fixture");
    state
        .workspace
        .open_views
        .push(OpenCellView::new(fixture_ref.clone(), ViewType::Testbench));

    assert!(!state.can_undo_project_design());
    let error = state
        .undo_project_design()
        .expect_err("open generated fixture must block undo with an actionable reason");
    assert!(error.contains("cannot be undone while it is open"));
    assert!(
        state
            .workspace
            .schematic_buffers
            .contains_key(&fixture_ref.key())
    );
    assert!(
        state
            .library_manager
            .get_library(&library_name)
            .and_then(|library| library.get_cell("existing_symbol"))
            .and_then(|cell| cell.get_view("testbench"))
            .is_some()
    );
}

#[test]
fn symbol_history_refuses_to_remove_the_active_generated_fixture() {
    let mut state = AppState::default();
    state.project_lifecycle.project_open = true;
    let library_name = state.workspace.active_view.library.clone();
    let fixture_ref = CellViewRef::new(&library_name, "active_fixture", "testbench");
    state
        .library_manager
        .get_library_mut(&library_name)
        .expect("writable project library")
        .add_cell(Cell::new("active_fixture"));

    let mut candidate = state.library_manager.clone();
    candidate
        .get_library_mut(&library_name)
        .and_then(|library| library.get_cell_mut("active_fixture"))
        .expect("existing cell")
        .add_view(View::new("testbench", ViewType::Testbench));
    let mut fixture = SchematicState::default();
    fixture.add_component(ComponentType::Resistor, Point::origin());
    publish_symbol_definition_candidate_with_fixture(
        &mut state,
        candidate,
        &library_name,
        "active_fixture",
        "add active fixture",
        Some(SymbolDefinitionFixtureDelta {
            reference: fixture_ref.clone(),
            before: None,
            after: Some(fixture.clone()),
        }),
    )
    .expect("publish fixture");
    state.workspace.active_view = fixture_ref.clone();
    state.schematic = fixture;

    assert!(!state.can_undo_project_design());
    let error = state
        .undo_project_design()
        .expect_err("active generated fixture must block undo with an actionable reason");
    assert!(error.contains("cannot be undone while it is open"));
    assert!(
        state
            .workspace
            .schematic_buffers
            .contains_key(&fixture_ref.key())
    );
}
