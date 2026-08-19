//! Confirming a pending cell or view deletion.
//!
//! Deletion is staged rather than immediate so the dialog can report what
//! else references the object before it is removed. A master that is placed
//! somewhere is never deleted on the strength of the reader having asked for
//! it: they answer what becomes of the placements first, and that answer
//! travels with the staged deletion.

use egui::Context;

use crate::diagnostics::ConsoleMessage;
use crate::state::{CellViewRef, ProjectSourceOwner};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};
use crate::workbench::app::dialogs::state::{DeletionInstanceResolution, LibraryDeletionTarget};
use crate::workbench::{AppState, RSpiceApp};

use super::VERILOGA_LIBRARY_NAME;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LibraryDeletionImpact {
    views: usize,
    open_views: usize,
    dirty_open_views: usize,
    instance_references: usize,
    source_bundles: usize,
    configuration_roots: usize,
    project_root: bool,
}

impl AppState {
    pub(crate) fn open_library_cell_deletion_review(
        &mut self,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        self.open_library_deletion_review(LibraryDeletionTarget::Cell {
            library: library.to_owned(),
            cell: cell.to_owned(),
        })
    }

    pub(crate) fn open_library_view_deletion_review(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Result<(), String> {
        self.open_library_deletion_review(LibraryDeletionTarget::View {
            library: library.to_owned(),
            cell: cell.to_owned(),
            view: view.to_owned(),
        })
    }

    fn open_library_deletion_review(
        &mut self,
        target: LibraryDeletionTarget,
    ) -> Result<(), String> {
        validate_library_deletion_target(self, &target)?;
        if self.pending_delete_cell.is_some() || self.pending_delete_view.is_some() {
            return Err("Another library deletion is already pending.".to_owned());
        }
        self.dialogs.library_deletion_review.target = Some(target);
        self.dialogs
            .library_deletion_review
            .expected_library_revision = self.library_manager.revision();
        self.dialogs.library_deletion_review.error = None;
        Ok(())
    }

    fn confirm_library_deletion_review(
        &mut self,
        resolution: DeletionInstanceResolution,
    ) -> Result<(), String> {
        let review = &self.dialogs.library_deletion_review;
        let target = review
            .target
            .clone()
            .ok_or_else(|| "No library deletion is awaiting confirmation.".to_owned())?;
        if self.library_manager.revision() != review.expected_library_revision {
            return Err(
                "The library catalog changed after this review opened. Review the deletion again."
                    .to_owned(),
            );
        }
        let impact = library_deletion_impact(self, &target);
        can_delete(self, &target, &impact, Some(resolution))?;
        if self.pending_delete_cell.is_some() || self.pending_delete_view.is_some() {
            return Err("Another library deletion is already pending.".to_owned());
        }

        match target {
            LibraryDeletionTarget::Cell { library, cell } => {
                self.pending_delete_cell = Some((library, cell));
            }
            LibraryDeletionTarget::View {
                library,
                cell,
                view,
            } => {
                self.pending_delete_view = Some((library, cell, view));
            }
        }
        self.dialogs.library_deletion_review.close();
        self.dialogs.library_deletion_review.resolution = Some(resolution);
        Ok(())
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_library_deletion_review_dialog(&mut self, ctx: &Context) {
        let Some(target) = self.state.dialogs.library_deletion_review.target.clone() else {
            return;
        };
        let impact = library_deletion_impact(&self.state, &target);
        let revision_current = self.state.library_manager.revision()
            == self
                .state
                .dialogs
                .library_deletion_review
                .expected_library_revision;
        let gate = can_delete(
            &self.state,
            &target,
            &impact,
            Some(DeletionInstanceResolution::default()),
        );
        let deletable = revision_current && gate.is_ok();
        let placements = impact.instance_references;
        let target_path = target.display_path();
        let object = target.kind_label().to_ascii_lowercase();
        let view_count = impact.views.to_string();
        let open_count = format!(
            "{} ({} with working changes)",
            impact.open_views, impact.dirty_open_views
        );
        let reference_count = placements.to_string();
        let source_count = impact.source_bundles.to_string();
        let root_count = impact.configuration_roots.to_string();
        let project_root = if impact.project_root { "yes" } else { "no" };
        let stale_error = (!revision_current).then(|| {
            "The library catalog changed after this review opened. Cancel and review the deletion again.".to_owned()
        });
        let retained_error = self
            .state
            .dialogs
            .library_deletion_review
            .error
            .as_deref()
            .map(str::to_owned);
        let displayed_error = stale_error.or_else(|| gate.err()).or(retained_error);
        let title = match target {
            LibraryDeletionTarget::Cell { .. } => "Delete cell",
            LibraryDeletionTarget::View { .. } => "Delete view",
        };
        let description = match target {
            LibraryDeletionTarget::Cell { .. } => {
                "Permanently remove the exact cell, all of its views, owned sources, open documents, and loaded workspace buffers."
            }
            LibraryDeletionTarget::View { .. } => {
                "Permanently remove the exact view, its owned source, open document, and loaded workspace buffer."
            }
        };
        let placement_question = (placements > 0).then(|| {
            format!(
                "{placements} {} of this {object} {} drawn in this project. Deleting it does not decide what happens to {}.",
                if placements == 1 {
                    "placement"
                } else {
                    "placements"
                },
                if placements == 1 { "is" } else { "are" },
                if placements == 1 { "it" } else { "them" }
            )
        });
        let keep_label = if placements > 0 {
            "Delete, keep placements"
        } else {
            "Delete"
        };
        let mut dialog = Dialog::new("Library", title, keep_label)
            .description(description)
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .destructive()
            .primary_enabled(deletable)
            .hint("Permanent project mutation");
        if placements > 0 {
            dialog = dialog
                .secondary("Delete and remove placements")
                .secondary_enabled(deletable);
        }
        let choice = dialog.show(ctx, |ui| {
            kv_row(ui, "Target", &target_path);
            kv_row(ui, "Object", target.kind_label());
            if matches!(target, LibraryDeletionTarget::Cell { .. }) {
                kv_row(ui, "Views removed", &view_count);
            }
            kv_row(ui, "Open views", &open_count);
            kv_row(ui, "Placements in this project", &reference_count);
            kv_row(ui, "Owned source bundles", &source_count);
            kv_row(ui, "Configuration roots", &root_count);
            kv_row(ui, "Project root", project_root);

            let t = Tokens::get(ui.ctx());
            if let Some(question) = &placement_question {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(question)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(t.color.text),
                );
                ui.add_space(4.0);
                for line in [
                    "Keep placements: they stay on their sheets, still naming this master, and read as unresolved until you rebind or remove them.",
                    "Remove placements: they come off every sheet at once, as one step you can undo. Their wires stay where they are.",
                ] {
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_dim),
                    );
                }
            }

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(
                    "This action removes the selected object from the current project. It cannot be undone from the schematic-local history.",
                )
                .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                .color(t.color.warn),
            );
            if let Some(error) = &displayed_error {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.err),
                );
            }
        });

        let resolution = match choice {
            DialogChoice::Primary => Some(DeletionInstanceResolution::KeepUnresolved),
            DialogChoice::Secondary => Some(DeletionInstanceResolution::RemoveInstances),
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.library_deletion_review.close();
                None
            }
            DialogChoice::None => None,
        };
        if let Some(resolution) = resolution
            && let Err(error) = self.state.confirm_library_deletion_review(resolution)
        {
            self.state.dialogs.library_deletion_review.error = Some(error);
        }
    }

    pub(in crate::workbench) fn process_pending_library_deletions(&mut self) {
        if let Some((lib_name, cell_name)) = self.state.pending_delete_cell.take() {
            let target = LibraryDeletionTarget::Cell {
                library: lib_name.clone(),
                cell: cell_name.clone(),
            };
            let Some(resolution) = staged_resolution(&mut self.state, &target) else {
                return;
            };
            let ownership_removal =
                match prepare_design_management_removal(&self.state, &lib_name, &cell_name, None) {
                    Ok(removal) => removal,
                    Err(error) => {
                        self.state.push_user_message(ConsoleMessage::error(error));
                        return;
                    }
                };
            let project_mutation = match self.state.preflight_project_library_mutation(
                crate::state::ProjectLibraryMutation::DeleteCell {
                    library: lib_name.clone(),
                    cell: cell_name.clone(),
                },
            ) {
                Ok(prepared) => prepared,
                Err(error) => {
                    self.state.push_user_message(ConsoleMessage::error(error));
                    return;
                }
            };
            let mut deleted = false;
            if let Some(lib) = self.state.library_manager.get_library_mut(&lib_name) {
                deleted = lib.remove_cell(&cell_name);
                if deleted {
                    apply_design_management_removal(&mut self.state, ownership_removal);
                    remove_project_sources_for_deleted_scope(
                        &mut self.state,
                        &lib_name,
                        &cell_name,
                        None,
                    );
                    self.state
                        .prune_workspace_after_cell_deleted(&lib_name, &cell_name);
                    self.state
                        .publish_project_library_mutation(project_mutation);
                    // After the library mutation, never before it: publishing
                    // one retires the project design history, and the answer
                    // the reader gave about the placements is the one step
                    // that has to survive the deletion it was given for.
                    apply_instance_resolution(&mut self.state, &target, resolution);
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
            let target = LibraryDeletionTarget::View {
                library: lib_name.clone(),
                cell: cell_name.clone(),
                view: view_name.clone(),
            };
            let Some(resolution) = staged_resolution(&mut self.state, &target) else {
                return;
            };
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
            let project_mutation = match self.state.preflight_project_library_mutation(
                crate::state::ProjectLibraryMutation::DeleteView {
                    library: lib_name.clone(),
                    cell: cell_name.clone(),
                    view: view_name.clone(),
                },
            ) {
                Ok(prepared) => prepared,
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
                    apply_design_management_removal(&mut self.state, ownership_removal);
                    remove_project_sources_for_deleted_scope(
                        &mut self.state,
                        &lib_name,
                        &cell_name,
                        Some(&view_name),
                    );
                    self.state
                        .prune_workspace_after_view_deleted(&lib_name, &cell_name, &view_name);
                    self.state
                        .publish_project_library_mutation(project_mutation);
                    apply_instance_resolution(&mut self.state, &target, resolution);
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

/// The one gate every deletion passes, whichever route staged it.
///
/// The dialog asks it to decide whether its actions are live, and the staged
/// mutation asks it again immediately before removing anything, so the two can
/// never disagree about whether this object may go. The last clause is the
/// reason it takes a resolution at all: a master that is placed somewhere is
/// not deleted on the strength of the request alone — the reader has to have
/// said what becomes of the placements.
fn can_delete(
    state: &AppState,
    target: &LibraryDeletionTarget,
    impact: &LibraryDeletionImpact,
    resolution: Option<DeletionInstanceResolution>,
) -> Result<(), String> {
    validate_library_deletion_target(state, target)?;
    let roots = state.workspace.configuration_sets.roots_in_scope(
        target.library(),
        target.cell(),
        target.view(),
    );
    if !roots.is_empty() {
        let mut names = roots
            .iter()
            .take(4)
            .map(|configuration| configuration.name())
            .collect::<Vec<_>>()
            .join(", ");
        if roots.len() > 4 {
            names.push_str(&format!(" and {} more", roots.len() - 4));
        }
        return Err(format!(
            "Configuration set roots still reference this {} ({names}). Rebind or remove those configurations first.",
            target.kind_label().to_ascii_lowercase()
        ));
    }
    let placements = impact.instance_references;
    if placements > 0 && resolution.is_none() {
        return Err(format!(
            "{placements} {} of '{}' {} drawn in this project. Choose what happens to {} before deleting it.",
            if placements == 1 {
                "placement"
            } else {
                "placements"
            },
            target.display_path(),
            if placements == 1 { "is" } else { "are" },
            if placements == 1 { "it" } else { "them" }
        ));
    }
    Ok(())
}

/// Re-check a staged deletion and take the choice that was staged with it.
///
/// A deletion that reaches here without one was never reviewed, so it is
/// refused rather than resolved by a default the reader never saw.
fn staged_resolution(
    state: &mut AppState,
    target: &LibraryDeletionTarget,
) -> Option<DeletionInstanceResolution> {
    let resolution = state.dialogs.library_deletion_review.resolution.take();
    let impact = library_deletion_impact(state, target);
    match can_delete(state, target, &impact, resolution) {
        Ok(()) => Some(resolution.unwrap_or_default()),
        Err(error) => {
            state.push_user_message(ConsoleMessage::error(error));
            None
        }
    }
}

/// Carry out the reader's choice about the placements of a deleted object.
///
/// Removal spans the live sheet and every loaded buffer and is recorded as one
/// project step: one question was asked, so Undo asks it back once. Keeping
/// them is not a no-op — the placements are revalidated where the deletion
/// prunes the workspace, so each stops claiming the master's netlist identity.
fn apply_instance_resolution(
    state: &mut AppState,
    target: &LibraryDeletionTarget,
    resolution: DeletionInstanceResolution,
) {
    if resolution != DeletionInstanceResolution::RemoveInstances {
        return;
    }
    let library = target.library().to_owned();
    let cell = target.cell().to_owned();
    let view = target.view().map(str::to_owned);
    let places_target = |component: &crate::state::Component| {
        component.library_cell.as_ref().is_some_and(|binding| {
            binding.library == library
                && binding.cell == cell
                && view.as_ref().is_none_or(|view| &binding.view == view)
        })
    };

    let active = state.workspace.active_schematic_reference();
    let active_key = active.key();
    let mut edited = state
        .workspace
        .schematic_buffers
        .iter()
        .filter(|(key, schematic)| {
            key.as_str() != active_key && schematic.components.iter().any(&places_target)
        })
        .filter_map(|(key, schematic)| {
            buffer_reference(key).map(|reference| (reference, schematic.clone()))
        })
        .collect::<Vec<_>>();
    if state.schematic.components.iter().any(&places_target) {
        edited.push((active, state.schematic.clone()));
    }
    if edited.is_empty() {
        return;
    }

    let drawings = edited.len();
    let mut documents = Vec::with_capacity(drawings);
    for (reference, before) in edited {
        let mut after = before.clone();
        after
            .components
            .retain(|component| !places_target(component));
        after.recalculate_runtime_state();
        after.bump_topology_version();
        after.is_dirty = true;
        let key = reference.key();
        if key == active_key {
            state.schematic = after.clone();
        }
        state.workspace.schematic_buffers.insert(key, after.clone());
        documents.push((reference, before, after));
    }
    state.record_instance_removal_transaction(
        format!("remove placements of {}", target.display_path()),
        documents,
    );
    state.push_user_message(ConsoleMessage::info(format!(
        "Removed every placement of '{}' from {drawings} {}",
        target.display_path(),
        if drawings == 1 { "drawing" } else { "drawings" }
    )));
}

/// The exact cell view a loaded schematic buffer belongs to.
///
/// A buffer key is `library/cell/view` and no name segment may contain a
/// slash, so this split is the exact inverse of the key it reads.
fn buffer_reference(key: &str) -> Option<CellViewRef> {
    let mut segments = key.split('/');
    let reference = CellViewRef::new(segments.next()?, segments.next()?, segments.next()?);
    segments.next().is_none().then_some(reference)
}

fn validate_library_deletion_target(
    state: &AppState,
    target: &LibraryDeletionTarget,
) -> Result<(), String> {
    if state.workbench.safe_mode.project_read_only() {
        return Err("The project is open read-only; library objects cannot be deleted.".to_owned());
    }
    let library = state
        .library_manager
        .get_library(target.library())
        .ok_or_else(|| format!("Library '{}' no longer exists.", target.library()))?;
    if library.read_only {
        return Err(format!(
            "Library '{}' is read-only; its objects cannot be deleted.",
            target.library()
        ));
    }
    let cell = library.get_cell(target.cell()).ok_or_else(|| {
        format!(
            "Cell '{}/{}' no longer exists.",
            target.library(),
            target.cell()
        )
    })?;
    if let Some(view) = target.view()
        && cell.get_view(view).is_none()
    {
        return Err(format!(
            "View '{}/{}/{}' no longer exists.",
            target.library(),
            target.cell(),
            view
        ));
    }
    Ok(())
}

fn library_deletion_impact(
    state: &AppState,
    target: &LibraryDeletionTarget,
) -> LibraryDeletionImpact {
    let matches = |reference: &crate::state::CellViewRef| {
        reference.library == target.library()
            && reference.cell == target.cell()
            && target.view().is_none_or(|view| reference.view == view)
    };
    let views = state
        .library_manager
        .get_library(target.library())
        .and_then(|library| library.get_cell(target.cell()))
        .map_or(0, |cell| {
            target.view().map_or_else(
                || cell.view_count(),
                |view| usize::from(cell.get_view(view).is_some()),
            )
        });
    let open_views = state
        .workspace
        .open_views
        .iter()
        .filter(|open| matches(&open.reference))
        .count();
    let dirty_open_views = state
        .workspace
        .open_views
        .iter()
        .filter(|open| matches(&open.reference) && open.dirty)
        .count();
    let source_bundles = state
        .workspace
        .project_sources
        .iter_bundles()
        .filter(|bundle| {
            matches!(
                bundle.owner(),
                ProjectSourceOwner::CellView { reference } if matches(reference)
            )
        })
        .count();
    let configuration_roots = state
        .workspace
        .configuration_sets
        .roots_in_scope(target.library(), target.cell(), target.view())
        .len();
    let project_root = state.workspace.project.root_library == target.library()
        && state.workspace.project.top_cell == target.cell()
        && target
            .view()
            .is_none_or(|view| view == crate::state::workspace::DEFAULT_SCHEMATIC_VIEW);

    let count_references = |schematic: &crate::state::SchematicState| {
        schematic
            .components
            .iter()
            .filter(|component| {
                component.library_cell.as_ref().is_some_and(|binding| {
                    binding.library == target.library()
                        && binding.cell == target.cell()
                        && target.view().is_none_or(|view| binding.view == view)
                })
            })
            .count()
    };
    let active_key = state.workspace.active_view.key();
    let instance_references = count_references(&state.schematic)
        + state
            .workspace
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active_key))
            .map(|(_, schematic)| count_references(schematic))
            .sum::<usize>();

    LibraryDeletionImpact {
        views,
        open_views,
        dirty_open_views,
        instance_references,
        source_bundles,
        configuration_roots,
        project_root,
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
    Ok(Some(PendingDesignManagementRemoval { catalog }))
}

fn apply_design_management_removal(
    state: &mut crate::workbench::app_state::AppState,
    removal: Option<PendingDesignManagementRemoval>,
) -> bool {
    let Some(removal) = removal else {
        return false;
    };
    state.workspace.design_management = removal.catalog;
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
        let automation_runtime_project_id = state.workspace.project.id();
        RSpiceApp {
            state,
            first_frame: false,
            #[cfg(not(target_arch = "wasm32"))]
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            #[cfg(not(target_arch = "wasm32"))]
            automation_runtime: crate::automation_runtime::NativeAutomationRuntime::discover(),
            #[cfg(target_arch = "wasm32")]
            automation_runtime: crate::automation_runtime::BrowserAutomationRuntime::discover(),
            automation_runtime_project_id,
            cloud_account: crate::services::cloud_account::CloudAccountService::unconfigured(),
            model_hub: crate::services::model_hub::ModelHubService::unavailable(
                "This test instance runs without a model-pack store.",
            ),
            live_session: crate::workbench::live_session::LiveSessionEngine::default(),
            file_workflow_io: Box::new(
                crate::workbench::workflows::file_workflow::NativeFileWorkflowIo,
            ),
            export_workflow_io: Box::new(
                crate::workbench::workflows::export_workflow::NativeExportWorkflowIo,
            ),
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

    /// Two loaded drawings placing the same master, one of them in front.
    fn default_project_with_two_drawings_instancing_amp() -> crate::workbench::app_state::AppState {
        let mut state = default_project_with_active_top_instancing_amp();
        let aux_ref = CellViewRef::new("user", "aux", "schematic");
        if let Some(library) = state.library_manager.get_library_mut("user") {
            let mut aux = Cell::new("aux");
            aux.add_view(View::new("schematic", ViewType::Schematic));
            library.add_cell(aux);
        }
        let mut aux = SchematicState::default();
        aux.add_library_cell_component(
            Point::new(40, 40),
            LibraryCellInstance::new("user", "amp", "schematic"),
        );
        state.workspace.schematic_buffers.insert(aux_ref.key(), aux);
        state
            .workspace
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
        state.pending_delete_cell = Some(("user".to_owned(), cell.to_owned()));
        state.dialogs.library_deletion_review.resolution = Some(resolution);
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
    fn deletion_review_is_exact_revision_bound_and_only_confirmation_stages_mutation() {
        let mut state = state_with_open_amp_cell();

        state
            .open_library_cell_deletion_review("work", "amp")
            .expect("writable exact cell should open a deletion review");

        assert_eq!(
            state.dialogs.library_deletion_review.target,
            Some(LibraryDeletionTarget::Cell {
                library: "work".to_owned(),
                cell: "amp".to_owned(),
            })
        );
        assert!(state.pending_delete_cell.is_none());
        assert!(state.pending_delete_view.is_none());

        state
            .confirm_library_deletion_review(DeletionInstanceResolution::KeepUnresolved)
            .expect("unchanged reviewed cell should stage deletion");

        assert_eq!(
            state.pending_delete_cell,
            Some(("work".to_owned(), "amp".to_owned()))
        );
        assert!(state.dialogs.library_deletion_review.target.is_none());
        assert!(
            state
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
            .open_library_view_deletion_review("work", "amp", "symbol")
            .expect("writable exact view should open a deletion review");
        state.library_manager.add_library(Library::new("unrelated"));

        let stale_error = state
            .confirm_library_deletion_review(DeletionInstanceResolution::KeepUnresolved)
            .expect_err("catalog revision changes must invalidate the review");

        assert!(stale_error.contains("changed after this review opened"));
        assert!(state.pending_delete_view.is_none());
        assert!(state.dialogs.library_deletion_review.target.is_some());
        state.dialogs.library_deletion_review.close();

        state
            .library_manager
            .get_library_mut("work")
            .expect("work library exists")
            .read_only = true;
        let read_only_error = state
            .open_library_cell_deletion_review("work", "amp")
            .expect_err("read-only library must fail closed");
        assert!(read_only_error.contains("read-only"));
        assert!(state.pending_delete_cell.is_none());
        assert!(state.dialogs.library_deletion_review.target.is_none());
    }

    #[test]
    fn deletion_review_reports_loaded_dependents_without_double_counting_active_buffer() {
        let mut state = state_with_open_amp_cell();
        state
            .workspace
            .open_views
            .iter_mut()
            .find(|open| open.reference == CellViewRef::new("work", "amp", "schematic"))
            .expect("amp schematic is open")
            .dirty = true;
        let keep = state
            .workspace
            .schematic_buffers
            .get_mut(&CellViewRef::new("work", "keep", "schematic").key())
            .expect("keep schematic buffer exists");
        keep.add_library_cell_component(
            Point::origin(),
            LibraryCellInstance::new("work", "amp", "schematic"),
        );
        let target = LibraryDeletionTarget::Cell {
            library: "work".to_owned(),
            cell: "amp".to_owned(),
        };

        let impact = library_deletion_impact(&state, &target);

        assert_eq!(impact.views, 2);
        assert_eq!(impact.open_views, 1);
        assert_eq!(impact.dirty_open_views, 1);
        assert_eq!(impact.instance_references, 1);
    }

    #[test]
    fn deleting_open_cell_prunes_workspace_references_and_restores_valid_focus() {
        let mut app = app_with_state(state_with_open_amp_cell());
        let source_id = insert_cell_source(&mut app.state, "amp", "behavior");
        let project_revision_before = app.state.workspace.project.revision().get();
        app.state.pending_delete_cell = Some(("work".to_string(), "amp".to_string()));

        app.process_pending_library_deletions();

        assert_eq!(
            app.state.workspace.project.revision().get(),
            project_revision_before + 1
        );
        assert!(matches!(
            app.state
                .workspace
                .project
                .library_mutation_audit()
                .last()
                .map(|receipt| receipt.mutation()),
            Some(crate::state::ProjectLibraryMutation::DeleteCell { library, cell })
                if library == "work" && cell == "amp"
        ));
        assert!(app.state.workspace.project_metadata_dirty);
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

        assert!(matches!(
            app.state
                .workspace
                .project
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
        stage_cell_deletion(
            &mut app.state,
            "amp",
            DeletionInstanceResolution::KeepUnresolved,
        );

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
        app.state.dialogs.library_deletion_review.resolution =
            Some(DeletionInstanceResolution::KeepUnresolved);

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

    #[test]
    fn delete_cell_with_instances_requires_a_resolution() {
        let mut app = app_with_state(default_project_with_active_top_instancing_amp());
        app.state.pending_delete_cell = Some(("user".to_owned(), "amp".to_owned()));

        app.process_pending_library_deletions();

        assert!(
            app.state
                .library_manager
                .get_library("user")
                .and_then(|library| library.get_cell("amp"))
                .is_some(),
            "a placed master is not deleted until the reader has answered for its placements"
        );
        assert!(
            app.state.log_buffer.entries().any(|entry| {
                entry.message.contains("1 placement of 'user/amp' is drawn")
                    && entry.message.contains("Choose what happens to it")
            }),
            "the refusal has to say what is missing"
        );

        stage_cell_deletion(
            &mut app.state,
            "amp",
            DeletionInstanceResolution::KeepUnresolved,
        );
        app.process_pending_library_deletions();

        assert!(
            app.state
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
        let binding = state.schematic.components[0]
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

        app.process_pending_library_deletions();

        let top = app
            .state
            .workspace
            .schematic_buffers
            .get(&CellViewRef::new("user", "top", "schematic").key())
            .expect("the parent drawing survives");
        assert_eq!(top.components.len(), 1, "the placement stays drawn");
        let binding = top.components[0]
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
                .workspace
                .resolve_hierarchy(&app.state.library_manager)
                .unresolved_instances(),
            1,
            "the navigator has an unresolved placement to show"
        );
    }

    #[test]
    fn removing_placements_across_two_drawings_undoes_as_one_step() {
        let mut app = app_with_state(default_project_with_two_drawings_instancing_amp());
        app.state.project_lifecycle.project_open = true;
        let top_key = CellViewRef::new("user", "top", "schematic").key();
        let aux_key = CellViewRef::new("user", "aux", "schematic").key();
        stage_cell_deletion(
            &mut app.state,
            "amp",
            DeletionInstanceResolution::RemoveInstances,
        );

        app.process_pending_library_deletions();

        for key in [&top_key, &aux_key] {
            assert!(
                app.state.workspace.schematic_buffers[key]
                    .components
                    .is_empty(),
                "{key} still places the deleted master"
            );
        }
        assert!(
            app.state.can_undo_project_design(),
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
                app.state.workspace.schematic_buffers[key].components.len(),
                1,
                "{key} did not get its placement back"
            );
        }
        assert!(
            app.state.project_undo_sequence().is_none(),
            "both drawings came back on one step, not two"
        );
    }
}
