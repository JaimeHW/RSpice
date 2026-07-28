use crate::diagnostics::ConsoleMessage;

use super::{RSpiceApp, VERILOGA_LIBRARY_NAME};

impl RSpiceApp {
    pub(in crate::workbench) fn process_pending_library_deletions(&mut self) {
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            if block_configuration_root_deletion(&mut self.state, &lib_name, &cell_name, None) {
                return;
            }
            let ownership_removal =
                match prepare_design_management_removal(&self.state, &lib_name, &cell_name, None) {
                    Ok(removal) => removal,
                    Err(error) => {
                        self.state.push_user_message(ConsoleMessage::error(error));
                        return;
                    }
                };
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    if apply_design_management_removal(&mut self.state, ownership_removal) {
                        self.invalidate_simulation_preflight();
                    }
                    remove_project_sources_for_deleted_scope(
                        &mut self.state,
                        &lib_name,
                        &cell_name,
                        None,
                    );
                    self.state
                        .prune_workspace_after_cell_deleted(&lib_name, &cell_name);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted cell '{}' from library '{}'",
                        cell_name, lib_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }

        if let Some((lib_name, cell_name, view_name)) = self.state.pending_delete_view.take() {
            if block_configuration_root_deletion(
                &mut self.state,
                &lib_name,
                &cell_name,
                Some(&view_name),
            ) {
                return;
            }
            let ownership_removal = match prepare_design_management_removal(
                &self.state,
                &lib_name,
                &cell_name,
                Some(&view_name),
            ) {
                Ok(removal) => removal,
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
            };
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name)
                && let Some(cell) = lib.get_cell_mut(&cell_name)
            {
                deleted = cell.remove_view(&view_name);
                if deleted {
                    if apply_design_management_removal(&mut self.state, ownership_removal) {
                        self.invalidate_simulation_preflight();
                    }
                    remove_project_sources_for_deleted_scope(
                        &mut self.state,
                        &lib_name,
                        &cell_name,
                        Some(&view_name),
                    );
                    self.state
                        .prune_workspace_after_view_deleted(&lib_name, &cell_name, &view_name);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Deleted view '{}' from cell '{}'",
                        view_name, cell_name
                    )));
                }
            }
            if deleted && lib_name == VERILOGA_LIBRARY_NAME {
                self.persist_global_veriloga_library_with_feedback();
            }
        }
    }
}

struct PendingDesignManagementRemoval {
    catalog: crate::state::DesignManagementCatalog,
}

fn prepare_design_management_removal(
    state: &crate::workbench::app_state::AppState,
    library: &str,
    cell: &str,
    view: Option<&str>,
) -> Result<Option<PendingDesignManagementRemoval>, String> {
    let mut catalog = state.workspace.design_management.clone();
    let receipt = match view {
        Some(view) => catalog.remove_sheet_catalog_for_view(
            &crate::state::CellViewRef::new(library, cell, view).key(),
        ),
        None => catalog.remove_sheet_catalogs_for_cell(library, cell),
    }
    .map_err(|error| {
        let target = view.map_or_else(
            || format!("cell '{library}/{cell}'"),
            |view| format!("view '{library}/{cell}/{view}'"),
        );
        format!("Cannot delete {target}: Design Management still references it ({error}).")
    })?;
    if receipt.affected_sheet_catalogs == 0
        && receipt.remapped_variant_objects == 0
        && receipt.remapped_annotation_objects == 0
    {
        return Ok(None);
    }
    state
        .workspace
        .project
        .next_revision()
        .map_err(|error| format!("Could not advance the project revision: {error}"))?;
    Ok(Some(PendingDesignManagementRemoval { catalog }))
}

fn apply_design_management_removal(
    state: &mut crate::workbench::app_state::AppState,
    removal: Option<PendingDesignManagementRemoval>,
) -> bool {
    let Some(removal) = removal else {
        return false;
    };
    state
        .workspace
        .project
        .advance_revision()
        .expect("the project revision was preflighted without intervening mutation");
    state.workspace.design_management = removal.catalog;
    state.workspace.project_metadata_dirty = true;
    state.design_execution_epoch = state.design_execution_epoch.wrapping_add(1);
    state.ui.netlist.current_generation_input_digest = None;
    state.clear_project_design_history();
    true
}

fn block_configuration_root_deletion(
    state: &mut crate::workbench::app_state::AppState,
    library: &str,
    cell: &str,
    view: Option<&str>,
) -> bool {
    let roots = state
        .workspace
        .configuration_sets
        .roots_in_scope(library, cell, view);
    if roots.is_empty() {
        return false;
    }
    let mut names = roots
        .iter()
        .take(4)
        .map(|configuration| configuration.name())
        .collect::<Vec<_>>()
        .join(", ");
    if roots.len() > 4 {
        names.push_str(&format!(" and {} more", roots.len() - 4));
    }
    let target = view.map_or_else(
        || format!("cell '{library}/{cell}'"),
        |view| format!("view '{library}/{cell}/{view}'"),
    );
    state.push_user_message(ConsoleMessage::error(format!(
        "Cannot delete {target}: configuration set roots still reference it ({names}). Rebind or remove those configurations first."
    )));
    true
}

fn remove_project_sources_for_deleted_scope(
    state: &mut crate::workbench::app_state::AppState,
    library: &str,
    cell: &str,
    view: Option<&str>,
) {
    let removed = state
        .workspace
        .project_sources
        .remove_cell_view_bundles(library, cell, view);
    if removed.is_empty() {
        return;
    }
    state.workspace.project_sources_dirty = true;
    let transient_uses_removed = state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .is_some_and(|receipt| removed.contains(&receipt.token.bundle_id))
        || state
            .ui
            .code_workspace
            .veriloga
            .pending
            .as_ref()
            .is_some_and(|pending| removed.contains(&pending.token.bundle_id));
    if transient_uses_removed {
        state.ui.code_workspace.veriloga = Default::default();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, CellViewRef, ComponentType, Library, LibraryCellInstance, OpenCellView, Point,
        SchematicState, View, ViewType,
    };

    fn app_with_state(state: crate::workbench::app_state::AppState) -> RSpiceApp {
        RSpiceApp {
            state,
            first_frame: false,
            #[cfg(not(target_arch = "wasm32"))]
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(crate::workbench::workflows::file_workflow::NativeFileWorkflowIo),
            export_workflow_io: Box::new(crate::workbench::workflows::export_workflow::NativeExportWorkflowIo),
        }
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
        state.library_manager.add_library(library);

        let amp_ref = CellViewRef::new("work", "amp", "schematic");
        let keep_ref = CellViewRef::new("work", "keep", "schematic");
        let mut amp_schematic = SchematicState::default();
        amp_schematic.add_component(ComponentType::Resistor, Point::new(10, 10));
        state
            .workspace
            .schematic_buffers
            .insert(amp_ref.key(), amp_schematic.clone());
        state
            .workspace
            .schematic_buffers
            .insert(keep_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
            OpenCellView::new(amp_ref.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = amp_ref.clone();
        state.workspace.hierarchy_stack = vec![keep_ref, amp_ref.clone()];
        state.workspace.hierarchy_instances = vec!["XAMP".to_string()];
        state.schematic = amp_schematic;
        state
    }

    fn insert_cell_source(
        state: &mut crate::workbench::app_state::AppState,
        cell: &str,
        view: &str,
    ) -> crate::state::ProjectSourceId {
        state
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
            .workspace
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
            .workspace
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
        state.library_manager.add_library(library);

        let top = CellViewRef::new("work", "top", "schematic");
        let amp = CellViewRef::new("work", "amp", "schematic");
        let leaf = CellViewRef::new("work", "leaf", "schematic");
        state
            .workspace
            .schematic_buffers
            .insert(top.key(), SchematicState::default());
        state
            .workspace
            .schematic_buffers
            .insert(amp.key(), SchematicState::default());
        state
            .workspace
            .schematic_buffers
            .insert(leaf.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(top.clone(), ViewType::Schematic),
            OpenCellView::new(amp.clone(), ViewType::Schematic),
            OpenCellView::new(leaf.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = leaf.clone();
        state.workspace.hierarchy_stack = vec![top, amp, leaf.clone()];
        state.workspace.hierarchy_instances = vec!["XAMP".to_string(), "XLEAF".to_string()];
        state.schematic = SchematicState::default();
        state
    }

    fn default_project_with_active_keep_cell() -> crate::workbench::app_state::AppState {
        let mut state = crate::workbench::app_state::AppState::default();
        let keep_ref = CellViewRef::new("user", "keep", "schematic");
        if let Some(library) = state.library_manager.get_library_mut("user") {
            let mut keep = Cell::new("keep");
            keep.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(keep);
        }
        state
            .workspace
            .schematic_buffers
            .insert(keep_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![
            OpenCellView::new(
                CellViewRef::new("user", "top", "schematic"),
                ViewType::Schematic,
            ),
            OpenCellView::new(keep_ref.clone(), ViewType::Schematic),
        ];
        state.workspace.active_view = keep_ref.clone();
        state.workspace.hierarchy_stack = vec![keep_ref];
        state.workspace.hierarchy_instances.clear();
        state.schematic = SchematicState::default();
        state
    }

    fn default_project_with_active_top_instancing_amp() -> crate::workbench::app_state::AppState {
        let mut state = crate::workbench::app_state::AppState::default();
        let amp_ref = CellViewRef::new("user", "amp", "schematic");
        let top_ref = CellViewRef::new("user", "top", "schematic");

        if let Some(library) = state.library_manager.get_library_mut("user") {
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
            .workspace
            .schematic_buffers
            .insert(top_ref.key(), top.clone());
        state
            .workspace
            .schematic_buffers
            .insert(amp_ref.key(), SchematicState::default());
        state.workspace.open_views = vec![OpenCellView::new(top_ref.clone(), ViewType::Schematic)];
        state.workspace.active_view = top_ref.clone();
        state.workspace.hierarchy_stack = vec![top_ref];
        state.workspace.hierarchy_instances.clear();
        state.schematic = top;
        state
    }

    #[test]
    fn deleting_open_cell_prunes_workspace_references_and_restores_valid_focus() {
        let mut app = app_with_state(state_with_open_amp_cell());
        let source_id = insert_cell_source(&mut app.state, "amp", "behavior");
        app.state.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .open_views
                .iter()
                .all(|open| { open.reference.library != "work" || open.reference.cell != "amp" })
        );
        assert!(
            app.state
                .workspace
                .hierarchy_stack
                .iter()
                .all(|reference| reference.library != "work" || reference.cell != "amp")
        );
        assert!(
            !app.state
                .workspace
                .schematic_buffers
                .contains_key("work/amp/schematic")
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("work", "amp", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
        assert!(
            app.state
                .workspace
                .project_sources
                .get_bundle(source_id)
                .is_none()
        );
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[test]
    fn deleting_active_view_prunes_workspace_references_and_falls_back_to_schematic() {
        let mut state = state_with_open_amp_cell();
        let symbol_ref = CellViewRef::new("work", "amp", "symbol");
        state
            .workspace
            .open_views
            .push(OpenCellView::new(symbol_ref.clone(), ViewType::Symbol));
        state.workspace.active_view = symbol_ref.clone();
        state.workspace.hierarchy_stack = vec![symbol_ref.clone()];
        state.workspace.hierarchy_instances.clear();
        let mut app = app_with_state(state);
        app.state.pending_delete_view =
            Some(("work".to_string(), "amp".to_string(), "symbol".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("symbol"))
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .open_views
                .iter()
                .all(|open| open.reference != symbol_ref)
        );
        assert!(
            !app.state
                .workspace
                .hierarchy_stack
                .iter()
                .any(|reference| reference == &symbol_ref)
        );
        assert_eq!(
            app.state.workspace.active_view,
            CellViewRef::new("work", "amp", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_veriloga_view_removes_only_its_owned_source_bundle() {
        let mut state = state_with_open_amp_cell();
        let removed_id = insert_cell_source(&mut state, "amp", "behavior");
        let retained_id = insert_cell_source(&mut state, "keep", "behavior");
        let mut app = app_with_state(state);
        app.state.pending_delete_view =
            Some(("work".to_owned(), "amp".to_owned(), "behavior".to_owned()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .workspace
                .project_sources
                .get_bundle(removed_id)
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .project_sources
                .get_bundle(retained_id)
                .is_some()
        );
        assert!(app.state.workspace.project_sources_dirty);
    }

    #[test]
    fn deleting_cell_or_view_is_blocked_while_any_configuration_owns_the_root() {
        let mut state = state_with_open_amp_cell();
        let root = CellViewRef::new("work", "amp", "schematic");
        let active = add_configuration_root(&mut state, "Release", root.clone());
        let inactive = state
            .workspace
            .configuration_sets
            .clone_configuration(active, 1, "Characterization")
            .expect("inactive configuration");
        assert_ne!(active, inactive);
        let before_catalog = state.workspace.configuration_sets.clone();
        let mut app = app_with_state(state);

        app.state.pending_delete_view =
            Some(("work".to_owned(), "amp".to_owned(), "schematic".to_owned()));
        app.process_pending_library_deletions();
        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_some()
        );

        app.state.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
        app.process_pending_library_deletions();
        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_some()
        );
        assert_eq!(app.state.workspace.configuration_sets, before_catalog);
    }

    #[test]
    fn deleting_hierarchy_ancestor_resets_invalid_breadcrumb_path() {
        let leaf = CellViewRef::new("work", "leaf", "schematic");
        let mut app = app_with_state(state_with_active_leaf_under_amp());
        app.state.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view, leaf);
        assert_eq!(app.state.workspace.hierarchy_stack, vec![leaf]);
        assert!(app.state.workspace.hierarchy_instances.is_empty());
    }

    #[test]
    fn deleting_default_top_cell_does_not_recreate_deleted_design() {
        let mut app = app_with_state(crate::workbench::app_state::AppState::default());
        app.state.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .is_none(),
            "deleted default top cell should not be recreated as fallback"
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("user", "top", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_default_top_schematic_view_does_not_recreate_deleted_view() {
        let mut app = app_with_state(crate::workbench::app_state::AppState::default());
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "top".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_none(),
            "deleted default top schematic view should not be recreated as fallback"
        );
        assert_ne!(
            app.state.workspace.active_view,
            CellViewRef::new("user", "top", "schematic")
        );
        assert!(app.state.workspace.active_context_schematic().is_some());
    }

    #[test]
    fn deleting_non_active_project_top_cell_repoints_root_and_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_keep_cell());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_cell = Some(("user".to_string(), "top".to_string()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .is_none(),
            "deleted project top cell should stay deleted"
        );
        assert_eq!(app.state.workspace.active_view.cell, "keep");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "keep");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_non_active_project_top_schematic_repoints_root_and_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_keep_cell());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "top".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("top"))
                .and_then(|cell| cell.get_view("schematic"))
                .is_none(),
            "deleted project top schematic should stay deleted"
        );
        assert_eq!(app.state.workspace.active_view.cell, "keep");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "keep");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_instanced_non_active_cell_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_top_instancing_amp());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_cell = Some(("user".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view.cell, "top");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "top");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_instanced_non_active_schematic_invalidates_runs() {
        let mut app = app_with_state(default_project_with_active_top_instancing_amp());
        let original_epoch = app.state.design_execution_epoch;
        app.state.pending_delete_view = Some((
            "user".to_string(),
            "amp".to_string(),
            "schematic".to_string(),
        ));

        app.process_pending_library_deletions();

        assert_eq!(app.state.workspace.active_view.cell, "top");
        assert_eq!(app.state.workspace.project.root_library, "user");
        assert_eq!(app.state.workspace.project.top_cell, "top");
        assert_ne!(app.state.design_execution_epoch, original_epoch);
    }

    #[test]
    fn deleting_cell_blocks_live_variant_then_removes_and_tombstones_design_management_ownership() {
        let mut state = state_with_open_amp_cell();
        let object_id = state.schematic.components[0].id;
        let owner = CellViewRef::new("work", "amp", "schematic").key();
        let sheet_id = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&owner, "Main", [object_id])
            .expect("owned sheet catalog");
        let object = crate::state::SchematicObjectKey::new(&owner, object_id)
            .expect("scoped schematic object");
        let variant_id = state
            .workspace
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
            protected_references:
                crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
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
            .workspace
            .design_management
            .annotation()
            .preview_renumbering(&renumber_request)
            .expect("renumber preview");
        state
            .workspace
            .design_management
            .annotation_mut()
            .commit_renumbering(&preview, &renumber_request)
            .expect("reviewed annotation");
        let catalog_before_blocked_delete = state.workspace.design_management.clone();
        let mut app = app_with_state(state);

        app.state.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_some(),
            "a live scoped variant must block the library mutation"
        );
        assert_eq!(
            app.state.workspace.design_management, catalog_before_blocked_delete,
            "blocked deletion must not partially mutate governed design state"
        );
        assert!(app.state.log_buffer.entries().any(|entry| {
            entry.message.contains("Cannot delete cell 'work/amp'")
                && entry
                    .message
                    .contains("Design Management still references it")
        }));

        let variant_revision = app
            .state
            .workspace
            .design_management
            .variants()
            .find(variant_id)
            .expect("variant remains after blocked delete")
            .revision();
        app.state
            .workspace
            .design_management
            .variants_mut()
            .update(
                variant_id,
                variant_revision,
                crate::state::AssemblyVariantDraft {
                    name: "Industrial".to_owned(),
                    parent_id: None,
                    inheritance: crate::state::VariantInheritance::OverrideChangedObjectsOnly,
                    qualification_plan:
                        crate::state::VariantQualificationPlan::InvalidateAffectedTests,
                    overrides: std::collections::BTreeMap::new(),
                },
            )
            .expect("variant reference is reviewed away");
        let annotation_journal_len = app
            .state
            .workspace
            .design_management
            .annotation()
            .journal()
            .len();

        app.state.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_none()
        );
        assert!(
            app.state
                .workspace
                .design_management
                .sheet_catalog(&owner)
                .is_none()
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .annotation()
                .journal()
                .len(),
            annotation_journal_len,
            "deletion must retain immutable reviewed annotation history"
        );
        assert!(
            app.state
                .workspace
                .design_management
                .annotation()
                .effective_mapping_for(&owner, object_id)
                .expect("deleted annotation lookup")
                .is_none()
        );
        assert!(matches!(
            app.state
                .workspace
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
        let object_id = state.schematic.components[0].id;
        let owner = CellViewRef::new("work", "amp", "schematic").key();
        let object = crate::state::SchematicObjectKey::new(&owner, object_id)
            .expect("scoped schematic object");
        let renumber_request = crate::state::RenumberRequest {
            scope: crate::state::RenumberScope::WholeProject,
            order: crate::state::RenumberOrder::HierarchyThenCoordinates,
            protected_references:
                crate::state::ProtectedReferencePolicy::RetainLockedAndExternalIds,
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
            .workspace
            .design_management
            .annotation()
            .preview_renumbering(&renumber_request)
            .expect("renumber preview");
        state
            .workspace
            .design_management
            .annotation_mut()
            .commit_renumbering(&preview, &renumber_request)
            .expect("reviewed annotation");
        assert!(
            state
                .workspace
                .design_management
                .sheet_catalog(&owner)
                .is_none()
        );
        let journal_len = state
            .workspace
            .design_management
            .annotation()
            .journal()
            .len();
        let mut app = app_with_state(state);

        app.state.pending_delete_cell = Some(("work".to_owned(), "amp".to_owned()));
        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("work")
                .and_then(|library| library.get_cell("amp"))
                .is_none()
        );
        assert_eq!(
            app.state
                .workspace
                .design_management
                .annotation()
                .journal()
                .len(),
            journal_len,
            "deletion must keep immutable annotation evidence"
        );
        assert!(
            app.state
                .workspace
                .design_management
                .annotation()
                .effective_mapping_for(&owner, object_id)
                .expect("deleted annotation lookup")
                .is_none()
        );
        assert!(matches!(
            app.state
                .workspace
                .design_management
                .annotation()
                .object_authorities()
                .get(&object),
            Some(crate::state::AnnotationObjectAuthority::Tombstone)
        ));
    }
}
