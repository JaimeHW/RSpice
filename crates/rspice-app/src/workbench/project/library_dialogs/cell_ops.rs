//! Copy Cell and Rename Cell — small modals on the dialog primitive
//! The forms are one field
//! deep; the semantics live in `Model::copy_cell` / `rename_cell`.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::WorkspacePruned;
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::slots::ProjectState;

impl ProjectState {
    /// Open Copy Cell pre-filled from a source cell, targeting the first
    /// writable library (preferring the source's own when writable).
    pub(crate) fn open_copy_cell_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        let source = model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if source.get_cell(cell).is_none() {
            return Err(format!("Cell '{library}/{cell}' no longer exists."));
        }
        let writable: Vec<String> = model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        let target = if writable.iter().any(|name| name == library) {
            library.to_owned()
        } else {
            writable
                .first()
                .cloned()
                .ok_or_else(|| "No writable destination library is available.".to_owned())?
        };

        self.copy_cell_source_library = library.to_owned();
        self.copy_cell_source_cell = cell.to_owned();
        self.copy_cell_target_library = target;
        self.copy_cell_name = format!("{cell}_copy");
        self.copy_cell_error = None;
        self.copy_cell_library_revision = model.library_manager.revision();
        self.copy_cell_dialog = true;
        Ok(())
    }

    /// Open Rename Cell pre-filled with the current name.
    pub(crate) fn open_rename_cell_dialog(
        &mut self,
        model: &Model,
        library: &str,
        cell: &str,
    ) -> Result<(), String> {
        let target = model
            .library_manager
            .get_library(library)
            .ok_or_else(|| format!("Library '{library}' no longer exists."))?;
        if target.read_only {
            return Err(format!(
                "Library '{library}' is read-only; its cells cannot be renamed."
            ));
        }
        if target.get_cell(cell).is_none() {
            return Err(format!("Cell '{library}/{cell}' no longer exists."));
        }

        self.rename_cell_library = library.to_owned();
        self.rename_cell_current = cell.to_owned();
        self.rename_cell_name = cell.to_owned();
        self.rename_cell_error = None;
        self.rename_cell_library_revision = model.library_manager.revision();
        self.rename_cell_dialog = true;
        Ok(())
    }
}

/// Copy the cell the dialog names, stating on the status bar what the copy's
/// drawing sync had to say.
fn commit_copy_cell_dialog(cx: &mut Cx<'_, ProjectFeature>) -> Result<usize, String> {
    if cx.model.library_manager.revision() != cx.state.copy_cell_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = cx.state.copy_cell_name.trim().to_owned();
    if let Some(error) = validate_lcv_name(&name, "Cell name") {
        return Err(error);
    }
    if cx.state.copy_cell_target_library.is_empty() {
        return Err("Please select a library".to_owned());
    }
    let source_library = cx.state.copy_cell_source_library.clone();
    let source_cell = cx.state.copy_cell_source_cell.clone();
    let target = cx.state.copy_cell_target_library.clone();
    let (outcome, prompt) = cx
        .model
        .copy_cell(&source_library, &source_cell, &target, &name);
    if let Some(prompt) = prompt {
        cx.wb.prompt(prompt);
    }
    outcome
}

/// Rename the cell the dialog names: how many instance references now name
/// it, and what the rename left outside the model, or `None` when the name
/// did not change.
fn commit_rename_cell_dialog(
    cx: &mut Cx<'_, ProjectFeature>,
) -> Result<Option<(usize, WorkspacePruned)>, String> {
    if cx.model.library_manager.revision() != cx.state.rename_cell_library_revision {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
    }
    let name = cx.state.rename_cell_name.trim().to_owned();
    let library = cx.state.rename_cell_library.clone();
    let current = cx.state.rename_cell_current.clone();
    if name == current {
        return Ok(None);
    }
    if let Some(error) = validate_lcv_name(&name, "Cell name") {
        return Err(error);
    }
    cx.model.rename_cell(&library, &current, &name).map(Some)
}

impl ProjectFeature {
    pub(in crate::workbench) fn process_copy_cell_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.copy_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_copy = false;

        let lib_names: Vec<String> = cx
            .model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();

        let view_count = cx
            .model
            .library_manager
            .get_library(&cx.state.copy_cell_source_library)
            .and_then(|lib| lib.get_cell(&cx.state.copy_cell_source_cell))
            .map(|cell| cell.views.len())
            .unwrap_or(0);
        let hint = format!("copies all {view_count} views and their content");
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.copy_cell_library_revision;
        if !catalog_current {
            cx.state.copy_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }

        let dialogs = &mut *cx.state;
        let source = format!(
            "{} / {}",
            dialogs.copy_cell_source_library, dialogs.copy_cell_source_cell
        );
        let choice = Dialog::new("Copy cell", "Copy")
            .description(
                "Copy every view and its content from the selected cell into a writable library under a new name.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint(&hint)
            .primary_enabled(catalog_current && !dialogs.copy_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Source", &source);

                // Target library — writable only.
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), t.metrics.row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        let (label_rect, _) = ui.allocate_exact_size(
                            egui::vec2(92.0, t.metrics.row_h),
                            egui::Sense::hover(),
                        );
                        ui.painter().text(
                            egui::pos2(label_rect.left(), label_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            "To library",
                            theme::sans(tokens::FS_1, FontWeight::Regular),
                            c.text_dim,
                        );
                        egui::ComboBox::from_id_salt("copy_cell_target_combo")
                            .selected_text(&dialogs.copy_cell_target_library)
                            .width(ui.available_width())
                            .show_ui(ui, |ui| {
                                for name in &lib_names {
                                    ui.selectable_value(
                                        &mut dialogs.copy_cell_target_library,
                                        name.clone(),
                                        name,
                                    );
                                }
                            });
                    },
                );

                input_row(ui, "New name", &mut dialogs.copy_cell_name);

                if let Some(error) = dialogs.copy_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_copy = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_copy {
            let source_cell = cx.state.copy_cell_source_cell.clone();
            let target = cx.state.copy_cell_target_library.clone();
            let name = cx.state.copy_cell_name.trim().to_owned();
            match commit_copy_cell_dialog(cx) {
                Ok(views) => {
                    cx.model.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!("Copied '{source_cell}' to '{target}/{name}' ({views} views)."),
                    );
                    cx.state.copy_cell_error = None;
                    should_close = true;
                }
                Err(error) => cx.state.copy_cell_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.copy_cell_dialog = false;
            dialogs.copy_cell_name.clear();
            dialogs.copy_cell_error = None;
        }
    }

    pub(in crate::workbench) fn process_rename_cell_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.rename_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_rename = false;

        let catalog_current =
            cx.model.library_manager.revision() == cx.state.rename_cell_library_revision;
        if !catalog_current {
            cx.state.rename_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }
        let dialogs = &mut *cx.state;
        let source = format!(
            "{} / {}",
            dialogs.rename_cell_library, dialogs.rename_cell_current
        );
        let choice = Dialog::new("Rename cell", "Rename")
            .description(
                "Rename the selected cell and remap open buffers, tabs, and instance bindings that reference it.",
            )
            .size(DialogSize::Transaction)
            .ghost("Cancel")
            .hint("remaps buffers, tabs, and instance bindings")
            .primary_enabled(catalog_current && !dialogs.rename_cell_name.trim().is_empty())
            .show(ctx, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let t = Tokens::get(ui.ctx());
                let c = t.color;

                kv_row(ui, "Cell", &source);
                input_row(ui, "New name", &mut dialogs.rename_cell_name);

                if let Some(error) = dialogs.rename_cell_error.clone() {
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(error)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            });

        match choice {
            DialogChoice::Primary => should_rename = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_rename {
            let current = cx.state.rename_cell_current.clone();
            let name = cx.state.rename_cell_name.trim().to_owned();
            match commit_rename_cell_dialog(cx) {
                Ok(Some((remapped, pruned))) => {
                    cx.wb.outbox.push(ProjectRequest::CatalogChanged {
                        pruned,
                        notice: format!(
                            "Renamed cell '{current}' to '{name}'; {remapped} instance \
                             references now name it."
                        ),
                    });
                    cx.state.rename_cell_error = None;
                    should_close = true;
                }
                Ok(None) => should_close = true,
                Err(error) => cx.state.rename_cell_error = Some(error),
            }
        }

        if should_close {
            let dialogs = &mut *cx.state;
            dialogs.rename_cell_dialog = false;
            dialogs.rename_cell_name.clear();
            dialogs.rename_cell_error = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Cell, Library};
    use crate::workbench::project::tests::with_cx;
    use crate::workbench::{AppState, RSpiceApp};

    #[test]
    fn rename_cell_dialog_starts_with_the_exact_active_cell_identity() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        state.model.library_manager.add_library(library);

        state
            .project
            .open_rename_cell_dialog(&state.model, "work", "amplifier")
            .expect("default work/amplifier cell is writable");

        assert!(state.project.rename_cell_dialog);
        assert_eq!(state.project.rename_cell_library, "work");
        assert_eq!(state.project.rename_cell_current, "amplifier");
        assert_eq!(state.project.rename_cell_name, "amplifier");
        assert!(state.project.rename_cell_error.is_none());
        assert_eq!(
            state.project.rename_cell_library_revision,
            state.model.library_manager.revision()
        );
    }

    #[test]
    fn dialog_entry_points_reject_stale_targets_without_mutating_modal_state() {
        let mut state = AppState::default();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        state.model.library_manager.add_library(library);
        state.project.copy_cell_source_library = "sentinel".to_owned();
        state.project.rename_cell_library = "sentinel".to_owned();

        assert!(
            state
                .project
                .open_copy_cell_dialog(&state.model, "missing", "amplifier")
                .is_err()
        );
        assert!(
            state
                .project
                .open_rename_cell_dialog(&state.model, "work", "missing")
                .is_err()
        );

        assert!(!state.project.copy_cell_dialog);
        assert!(!state.project.rename_cell_dialog);
        assert_eq!(state.project.copy_cell_source_library, "sentinel");
        assert_eq!(state.project.rename_cell_library, "sentinel");
    }

    #[test]
    fn rename_entry_point_rejects_read_only_library_without_mutation() {
        let mut state = AppState::default();
        let mut work = Library::new("work");
        work.add_cell(Cell::new("amplifier"));
        state.model.library_manager.add_library(work);
        let library = state
            .model
            .library_manager
            .get_library_mut("work")
            .expect("default work library exists");
        library.read_only = true;

        let error = state
            .project
            .open_rename_cell_dialog(&state.model, "work", "amplifier")
            .expect_err("read-only libraries must fail closed");

        assert!(error.contains("read-only"));
        assert!(!state.project.rename_cell_dialog);
        assert!(state.project.rename_cell_library.is_empty());
    }

    #[test]
    fn copy_and_rename_dialog_commits_reject_intervening_catalog_change() {
        let mut copy_app = RSpiceApp::test_instance();
        copy_app.state = AppState::default();
        let mut copy_library = Library::new("work");
        copy_library.add_cell(Cell::new("amplifier"));
        copy_app
            .state
            .model
            .library_manager
            .add_library(copy_library);
        copy_app
            .state
            .project
            .open_copy_cell_dialog(&copy_app.state.model, "work", "amplifier")
            .expect("copy dialog opens");
        assert_eq!(
            copy_app.state.project.copy_cell_library_revision,
            copy_app.state.model.library_manager.revision()
        );
        copy_app
            .state
            .model
            .library_manager
            .add_library(Library::new("intervening"));

        let copy_error = with_cx(&mut copy_app, commit_copy_cell_dialog)
            .expect_err("stale copy must fail closed");

        assert_eq!(copy_error, LIBRARY_CATALOG_STALE_MESSAGE);
        assert!(
            copy_app
                .state
                .model
                .library_manager
                .get_library("work")
                .is_some_and(|library| library.get_cell("amplifier_copy").is_none())
        );

        let mut rename_app = RSpiceApp::test_instance();
        rename_app.state = AppState::default();
        let mut rename_library = Library::new("work");
        rename_library.add_cell(Cell::new("amplifier"));
        rename_app
            .state
            .model
            .library_manager
            .add_library(rename_library);
        rename_app
            .state
            .project
            .open_rename_cell_dialog(&rename_app.state.model, "work", "amplifier")
            .expect("rename dialog opens");
        rename_app.state.project.rename_cell_name = "amplifier_v2".to_owned();
        rename_app
            .state
            .model
            .library_manager
            .add_library(Library::new("intervening"));

        let rename_error = with_cx(&mut rename_app, commit_rename_cell_dialog)
            .expect_err("stale rename must fail closed");

        assert_eq!(rename_error, LIBRARY_CATALOG_STALE_MESSAGE);
        let library = rename_app
            .state
            .model
            .library_manager
            .get_library("work")
            .expect("source library remains");
        assert!(library.get_cell("amplifier").is_some());
        assert!(library.get_cell("amplifier_v2").is_none());
    }

    /// The reference design with its active cell's rename typed into the
    /// dialog, and the active cell.
    fn renaming_the_active_cell() -> (RSpiceApp, String, String) {
        let mut app = RSpiceApp::test_instance();
        app.state = crate::workbench::examples::hierarchy_reference::build().state;
        let active = app.state.model.workspace.content.active_view.clone();
        let (library, cell) = (active.library.clone(), active.cell.clone());
        let state = &mut app.state;
        state
            .project
            .open_rename_cell_dialog(&state.model, &library, &cell)
            .expect("the active cell can be renamed");
        state.project.rename_cell_name = format!("{cell}_v2");
        (app, library, cell)
    }

    /// What the status bar, the Console and the drawing show.
    fn shown(app: &RSpiceApp) -> (Option<String>, Vec<String>, String) {
        (
            app.state.prompt_text().map(str::to_owned),
            app.state
                .model
                .log_buffer
                .entries()
                .map(|entry| entry.message.clone())
                .collect(),
            app.state.model.workspace.content.active_view.display_path(),
        )
    }

    /// A rename carried out in the feature's context, with what it left
    /// handed to the shell, ends where the rename carried out in place did:
    /// what it left is carried out first, and only then is the rename
    /// recorded.
    #[test]
    fn a_renamed_cell_states_what_the_rename_left_before_the_rename_itself() {
        let (mut in_place, library, cell) = renaming_the_active_cell();
        let name = format!("{cell}_v2");
        let remapped = in_place
            .state
            .rename_cell(&library, &cell, &name)
            .expect("the rename succeeds in place");
        in_place.state.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!(
                "Renamed cell '{cell}' to '{name}'; {remapped} instance references now name it."
            ),
        );

        let (mut handed, _, _) = renaming_the_active_cell();
        let (handed_remapped, pruned) = with_cx(&mut handed, commit_rename_cell_dialog)
            .expect("the rename succeeds in the feature's context")
            .expect("the name changed");
        assert_eq!(handed_remapped, remapped);
        handed
            .state
            .workbench
            .outbox
            .push(ProjectRequest::CatalogChanged {
                pruned,
                notice: format!(
                    "Renamed cell '{cell}' to '{name}'; {remapped} instance references now name it."
                ),
            });
        handed.after_contribution();

        assert_eq!(shown(&handed), shown(&in_place));
    }
}
