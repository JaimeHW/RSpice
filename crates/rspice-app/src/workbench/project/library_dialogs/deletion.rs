//! Confirming a pending cell or view deletion.
//!
//! Deletion is staged rather than immediate so the dialog can report what
//! else references the object before it is removed. A master that is placed
//! somewhere is never deleted on the strength of the reader having asked for
//! it: they answer what becomes of the placements first, and that answer
//! travels with the staged deletion.

use egui::Context;

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::state::{ProjectSourceId, ProjectSourceOwner};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, kv_row};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};
use crate::workbench::project::workspace_state::{
    DeletionInstanceResolution, LibraryDeletionTarget,
};
use crate::workbench::slots::ProjectState;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct LibraryDeletionImpact {
    pub(crate) views: usize,
    pub(crate) open_views: usize,
    pub(crate) dirty_open_views: usize,
    pub(crate) instance_references: usize,
    source_bundles: usize,
    configuration_roots: usize,
    project_root: bool,
}

impl ProjectState {
    /// Open the deletion review on the exact cell `library`/`cell`, against
    /// the library revision `model` holds now.
    pub(crate) fn open_library_cell_deletion_review(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        self.open_library_deletion_review(
            model,
            LibraryDeletionTarget::Cell {
                library: library.to_owned(),
                cell: cell.to_owned(),
            },
        )
    }

    /// Open the deletion review on the exact view `library`/`cell`/`view`,
    /// against the library revision `model` holds now.
    pub(crate) fn open_library_view_deletion_review(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Result<(), String> {
        self.open_library_deletion_review(
            model,
            LibraryDeletionTarget::View {
                library: library.to_owned(),
                cell: cell.to_owned(),
                view: view.to_owned(),
            },
        )
    }

    fn open_library_deletion_review(
        &mut self,
        model: &Model,
        target: LibraryDeletionTarget,
    ) -> Result<(), String> {
        validate_library_deletion_target(model, &target)?;
        if self.pending_delete_cell.is_some() || self.pending_delete_view.is_some() {
            return Err("Another library deletion is already pending.".to_owned());
        }
        self.library_deletion_review.target = Some(target);
        self.library_deletion_review.expected_library_revision = model.library_manager.revision();
        self.library_deletion_review.error = None;
        Ok(())
    }

    pub(crate) fn confirm_library_deletion_review(
        &mut self,
        model: &Model,
        resolution: DeletionInstanceResolution,
    ) -> Result<(), String> {
        let review = &self.library_deletion_review;
        let target = review
            .target
            .clone()
            .ok_or_else(|| "No library deletion is awaiting confirmation.".to_owned())?;
        if model.library_manager.revision() != review.expected_library_revision {
            return Err(
                "The library catalog changed after this review opened. Review the deletion again."
                    .to_owned(),
            );
        }
        let impact = library_deletion_impact(model, &target);
        can_delete(model, &target, &impact, Some(resolution))?;
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
        self.library_deletion_review.close();
        self.library_deletion_review.resolution = Some(resolution);
        Ok(())
    }
}

impl ProjectFeature {
    /// Draw the deletion review. Confirming it stages the deletion with the
    /// reader's answer about the placements; nothing is deleted here.
    pub(in crate::workbench) fn process_library_deletion_review_dialog(
        cx: &mut Cx<'_, Self>,
        ctx: &Context,
    ) {
        let Some(target) = cx.state.library_deletion_review.target.clone() else {
            return;
        };
        let impact = library_deletion_impact(cx.model, &target);
        let revision_current = cx.model.library_manager.revision()
            == cx.state.library_deletion_review.expected_library_revision;
        let gate = can_delete(
            cx.model,
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
        let retained_error = cx
            .state
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
                "Permanently remove this cell, all of its views, owned sources, open documents, and loaded workspace buffers."
            }
            LibraryDeletionTarget::View { .. } => {
                "Permanently remove this view, its owned source, open document, and loaded workspace buffer."
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
        let mut dialog = Dialog::new(title, keep_label)
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
                cx.state.library_deletion_review.close();
                None
            }
            DialogChoice::None => None,
        };
        if let Some(resolution) = resolution
            && let Err(error) = cx
                .state
                .confirm_library_deletion_review(cx.model, resolution)
        {
            cx.state.library_deletion_review.error = Some(error);
        }
    }

    /// Carry out the deletion the review staged, if one waits. What it
    /// leaves outside its catalog change goes to the shell, which carries it
    /// out as this step returns. A review neither opens nor confirms while a
    /// deletion waits, so at most one does.
    pub(crate) fn process_pending_library_deletions(cx: &mut Cx<'_, Self>) {
        if let Some((lib_name, cell_name)) = cx.state.pending_delete_cell.take() {
            delete_staged_cell(cx, lib_name, cell_name);
        } else if let Some((lib_name, cell_name, view_name)) = cx.state.pending_delete_view.take() {
            delete_staged_view(cx, lib_name, cell_name, view_name);
        }
    }
}

/// Delete the staged cell, unless the gate now refuses it, which the status
/// bar states.
fn delete_staged_cell(cx: &mut Cx<'_, ProjectFeature>, lib_name: String, cell_name: String) {
    let target = LibraryDeletionTarget::Cell {
        library: lib_name.clone(),
        cell: cell_name.clone(),
    };
    let Some(resolution) = staged_resolution(cx, &target) else {
        return;
    };
    let ownership_removal =
        match prepare_design_management_removal(cx.model, &lib_name, &cell_name, None) {
            Ok(removal) => removal,
            Err(error) => {
                cx.wb.prompt(error);
                return;
            }
        };
    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::DeleteCell {
            library: lib_name.clone(),
            cell: cell_name.clone(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.wb.prompt(error);
            return;
        }
    };
    let Some(mut lib) = cx.model.library_manager.edit_library(&lib_name) else {
        return;
    };
    if !lib.remove_cell(&cell_name) {
        return;
    }
    apply_design_management_removal(cx.model, ownership_removal);
    let removed_sources =
        remove_project_sources_for_deleted_scope(cx.model, &lib_name, &cell_name, None);
    let pruned = cx
        .model
        .prune_workspace_after_cell_deleted(&lib_name, &cell_name);
    cx.wb.outbox.push(ProjectRequest::CellOrViewDeleted {
        removed_sources,
        pruned,
        mutation: ReservedLibraryRevision::new(project_mutation),
        remove_placements: (resolution == DeletionInstanceResolution::RemoveInstances)
            .then(|| Box::new(target)),
        notice: format!("Deleted cell '{cell_name}' from library '{lib_name}'."),
        persist_global_veriloga: lib_name == VERILOGA_LIBRARY_NAME,
    });
}

/// Delete the staged view, unless the gate now refuses it, which the status
/// bar states.
fn delete_staged_view(
    cx: &mut Cx<'_, ProjectFeature>,
    lib_name: String,
    cell_name: String,
    view_name: String,
) {
    let target = LibraryDeletionTarget::View {
        library: lib_name.clone(),
        cell: cell_name.clone(),
        view: view_name.clone(),
    };
    let Some(resolution) = staged_resolution(cx, &target) else {
        return;
    };
    let ownership_removal = match prepare_design_management_removal(
        cx.model,
        &lib_name,
        &cell_name,
        Some(&view_name),
    ) {
        Ok(removal) => removal,
        Err(error) => {
            cx.wb.prompt(error);
            return;
        }
    };
    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::DeleteView {
            library: lib_name.clone(),
            cell: cell_name.clone(),
            view: view_name.clone(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.wb.prompt(error);
            return;
        }
    };
    let Some(mut lib) = cx.model.library_manager.edit_library(&lib_name) else {
        return;
    };
    if lib.remove_view(&cell_name, &view_name) != Some(true) {
        return;
    }
    apply_design_management_removal(cx.model, ownership_removal);
    let removed_sources =
        remove_project_sources_for_deleted_scope(cx.model, &lib_name, &cell_name, Some(&view_name));
    let pruned = cx
        .model
        .prune_workspace_after_view_deleted(&lib_name, &cell_name, &view_name);
    cx.wb.outbox.push(ProjectRequest::CellOrViewDeleted {
        removed_sources,
        pruned,
        mutation: ReservedLibraryRevision::new(project_mutation),
        remove_placements: (resolution == DeletionInstanceResolution::RemoveInstances)
            .then(|| Box::new(target)),
        notice: format!("Deleted view '{view_name}' from cell '{cell_name}'."),
        persist_global_veriloga: lib_name == VERILOGA_LIBRARY_NAME,
    });
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
    model: &Model,
    target: &LibraryDeletionTarget,
    impact: &LibraryDeletionImpact,
    resolution: Option<DeletionInstanceResolution>,
) -> Result<(), String> {
    validate_library_deletion_target(model, target)?;
    let roots = model.workspace.content.configuration_sets.roots_in_scope(
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
/// The reader's answer about the placements the staged deletion carries,
/// if the gate still lets the object go; why it does not is stated on the
/// status bar.
fn staged_resolution(
    cx: &mut Cx<'_, ProjectFeature>,
    target: &LibraryDeletionTarget,
) -> Option<DeletionInstanceResolution> {
    let resolution = cx.state.library_deletion_review.resolution.take();
    let impact = library_deletion_impact(cx.model, target);
    match can_delete(cx.model, target, &impact, resolution) {
        Ok(()) => Some(resolution.unwrap_or_default()),
        Err(error) => {
            cx.wb.prompt(error);
            None
        }
    }
}

fn validate_library_deletion_target(
    model: &Model,
    target: &LibraryDeletionTarget,
) -> Result<(), String> {
    if model.safe_mode.project_read_only() {
        return Err("The project is open read-only; library objects cannot be deleted.".to_owned());
    }
    let library = model
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

pub(crate) fn library_deletion_impact(
    model: &Model,
    target: &LibraryDeletionTarget,
) -> LibraryDeletionImpact {
    let matches = |reference: &crate::state::CellViewRef| {
        reference.library == target.library()
            && reference.cell == target.cell()
            && target.view().is_none_or(|view| reference.view == view)
    };
    let views = model
        .library_manager
        .get_library(target.library())
        .and_then(|library| library.get_cell(target.cell()))
        .map_or(0, |cell| {
            target.view().map_or_else(
                || cell.view_count(),
                |view| usize::from(cell.get_view(view).is_some()),
            )
        });
    let open_views = model
        .workspace
        .content
        .open_views
        .iter()
        .filter(|open| matches(&open.reference))
        .count();
    let dirty_open_views = model
        .workspace
        .content
        .open_views
        .iter()
        .filter(|open| matches(&open.reference) && open.dirty)
        .count();
    let source_bundles = model
        .workspace
        .content
        .project_sources
        .iter_bundles()
        .filter(|bundle| {
            matches!(
                bundle.owner(),
                ProjectSourceOwner::CellView { reference } if matches(reference)
            )
        })
        .count();
    let configuration_roots = model
        .workspace
        .content
        .configuration_sets
        .roots_in_scope(target.library(), target.cell(), target.view())
        .len();
    let project_root = model.workspace.content.project.root_library == target.library()
        && model.workspace.content.project.top_cell == target.cell()
        && target
            .view()
            .is_none_or(|view| view == crate::state::workspace::DEFAULT_SCHEMATIC_VIEW);

    let count_references = |document: &rspice_design::schematic::document::SchematicDocument| {
        document
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
    let active_key = model.workspace.content.active_view.key();
    let instance_references = count_references(model.schematic.document())
        + model
            .workspace
            .content
            .schematic_buffers
            .iter()
            .filter(|(key, _)| !key.eq_ignore_ascii_case(&active_key))
            .map(|(_, schematic)| count_references(schematic.document()))
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
    model: &Model,
    library: &str,
    cell: &str,
    view: Option<&str>,
) -> Result<Option<PendingDesignManagementRemoval>, String> {
    let mut catalog = (*model.workspace.content.design_management).clone();
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
    model: &mut Model,
    removal: Option<PendingDesignManagementRemoval>,
) -> bool {
    let Some(removal) = removal else {
        return false;
    };
    *model.workspace.content.design_management = removal.catalog;
    true
}

/// Take the project sources the deleted scope owned out of the project, and
/// which they were. Compile evidence naming one of them no longer describes
/// its source.
fn remove_project_sources_for_deleted_scope(
    model: &mut Model,
    library: &str,
    cell: &str,
    view: Option<&str>,
) -> Vec<ProjectSourceId> {
    let removed = model
        .workspace
        .content
        .project_sources
        .remove_cell_view_bundles(library, cell, view);
    if !removed.is_empty() {
        model.workspace.content.project_sources_dirty = true;
    }
    removed
}
