//! Copy Cell and Rename Cell: Copy Cell on the command form, with the
//! cells the copied cell instantiates when asked, Rename Cell on the dialog
//! primitive; the semantics live in `Model::copy_cell_hierarchy` and
//! `Model::rename_cell`.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::workspace::ImportedInclude;
use crate::state::{
    CellViewRef, ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile,
    ProjectSourceLanguage, ProjectSourceOwner,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{CommandForm, Dialog, DialogChoice, DialogSize, input_row, kv_row};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{HierarchyCopy, WorkspacePruned};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::ProjectRequest;
use crate::workbench::slots::ProjectState;
use rspice_app_state::veriloga_library::{
    BUILTIN_VERILOGA_LIBRARY, BuiltinVerilogAModel, builtin_veriloga_model,
    read_builtin_veriloga_model,
};

impl ProjectState {
    /// Open Copy Cell pre-filled from a source cell, targeting the first
    /// writable library (preferring the source's own when writable).
    pub fn open_copy_cell_dialog(
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

        self.copy_cell_builtin_model = None;
        self.copy_cell_source_library = library.to_owned();
        self.copy_cell_source_cell = cell.to_owned();
        self.copy_cell_target_library = target;
        self.copy_cell_name = format!("{cell}_copy");
        self.copy_cell_include_children = false;
        self.copy_cell_repoint = true;
        self.copy_cell_error = None;
        self.copy_cell_library_revision = model.library_manager.revision();
        self.copy_cell_dialog = true;
        Ok(())
    }

    /// Open Copy Cell on a built-in model: the same form, into the design's
    /// library when it can be written.
    pub(crate) fn open_copy_builtin_model_dialog(
        &mut self,
        model: &Model,
        registry_name: &'static str,
    ) -> Result<(), String> {
        let builtin = builtin_veriloga_model(registry_name)
            .ok_or_else(|| format!("'{registry_name}' is not a built-in model."))?;
        let writable: Vec<String> = model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        let active = &model.workspace.content.active_view.library;
        let target = if writable.contains(active) {
            active.clone()
        } else {
            writable
                .first()
                .cloned()
                .ok_or_else(|| "No writable destination library is available.".to_owned())?
        };
        let taken = model
            .library_manager
            .get_library(&target)
            .is_some_and(|library| library.get_cell(builtin.module).is_some());
        self.copy_cell_builtin_model = Some(builtin.registry_name);
        self.copy_cell_source_library = BUILTIN_VERILOGA_LIBRARY.to_owned();
        self.copy_cell_source_cell = builtin.name.to_owned();
        self.copy_cell_target_library = target;
        self.copy_cell_name = if taken {
            format!("{}_copy", builtin.module)
        } else {
            builtin.module.to_owned()
        };
        self.copy_cell_error = None;
        self.copy_cell_library_revision = model.library_manager.revision();
        self.copy_cell_dialog = true;
        Ok(())
    }

    /// Open Rename Cell pre-filled with the current name.
    pub fn open_rename_cell_dialog(
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

/// Copy the cell the dialog names, with the cells it instantiates when the
/// form asks, stating on the status bar what the copy's drawing sync had to
/// say. Returns the cells and the views copied.
fn commit_copy_cell_dialog(cx: &mut Cx<'_, ProjectFeature>) -> Result<(usize, usize), String> {
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
    if let Some(registry_name) = cx.state.copy_cell_builtin_model {
        return commit_copy_builtin_model(cx, registry_name, &name);
    }
    let source_library = cx.state.copy_cell_source_library.clone();
    let source_cell = cx.state.copy_cell_source_cell.clone();
    let target = cx.state.copy_cell_target_library.clone();
    let hierarchy = HierarchyCopy {
        include_children: cx.state.copy_cell_include_children,
        repoint: cx.state.copy_cell_repoint,
    };
    let (outcome, prompt) =
        cx.model
            .copy_cell_hierarchy(&source_library, &source_cell, &target, &name, hierarchy);
    if let Some(prompt) = prompt {
        cx.wb.prompt(prompt);
    }
    outcome
}

/// Make the built-in model an editable cell in the target library, the source
/// and its includes copied, and leave the page on it.
fn commit_copy_builtin_model(
    cx: &mut Cx<'_, ProjectFeature>,
    registry_name: &str,
    name: &str,
) -> Result<(usize, usize), String> {
    let builtin = builtin_veriloga_model(registry_name)
        .ok_or_else(|| format!("'{registry_name}' is not a built-in model."))?;
    let target = cx.state.copy_cell_target_library.clone();
    let library = cx
        .model
        .library_manager
        .get_library(&target)
        .ok_or_else(|| format!("Library '{target}' no longer exists."))?;
    if library.read_only {
        return Err(format!("Library '{target}' cannot be written."));
    }
    if library.get_cell(name).is_some() {
        return Err(format!("Cell '{target}/{name}' already exists."));
    }
    let reference = CellViewRef::new(&target, name, "veriloga");
    create_cell_from_builtin_model(cx.model, builtin, &reference)?;
    cx.model
        .library_manager
        .select_view(&reference.library, &reference.cell, &reference.view);
    cx.state.builtin_models.open = false;
    Ok((1, 1))
}

/// Make `reference` a Verilog-A cell from a built-in model's source: the same
/// bundle and view an import of those files would make.
fn create_cell_from_builtin_model(
    model: &mut Model,
    builtin: &BuiltinVerilogAModel,
    reference: &CellViewRef,
) -> Result<(), String> {
    let source = read_builtin_veriloga_model(builtin);
    if let Some(missing) = source.closure.missing().next() {
        return Err(format!("{} is missing.", missing.path));
    }
    let declared = source
        .closure
        .modules
        .as_ref()
        .map_err(|reason| format!("{} cannot be read: {reason}", source.root))?
        .iter()
        .find(|module| module.name == builtin.module)
        .cloned()
        .ok_or_else(|| format!("{} does not declare {}.", source.root, builtin.module))?;
    let root = source
        .files
        .get(&source.root)
        .ok_or_else(|| format!("{} is not in the library.", source.root))?;
    let files = source
        .closure
        .files
        .iter()
        .filter(|file| file.include == ImportedInclude::Found)
        .map(|file| {
            let contents = source
                .files
                .get(&file.path)
                .ok_or_else(|| format!("{} is not in the library.", file.path))?;
            ProjectSourceFile::try_new(file.path.clone(), contents.clone())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    let edges = source
        .closure
        .edges
        .iter()
        .map(|(importer, imported)| {
            ProjectSourceDependency::try_new(importer.clone(), imported.clone())
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        source.root.as_str(),
        root.as_str(),
        files,
        edges,
    )
    .map_err(|error| error.to_string())?;
    let build_profile = builtin
        .ngspice_branch
        .then(|| {
            let mut profile =
                rspice_simulation::project_veriloga::build_profile::VerilogABuildProfile::starter(
                    builtin.module,
                );
            profile
                .preprocessor
                .defines
                .insert("__NGSPICE__".to_owned(), String::new());
            ProjectSourceFile::try_new(".rspice/veriloga-build.toml", profile.to_toml()?)
                .map_err(|error| error.to_string())
        })
        .transpose()?;
    let plan = [(declared, reference.clone(), true)];
    model.publish_veriloga_cells(&plan, true, bundle, build_profile)
}

/// Rename the cell the dialog names: how many instance references now name
/// it, and what the rename left outside the model, or `None` when the name
/// did not change.
pub fn commit_rename_cell_dialog(
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
        let targets: Vec<String> = cx
            .model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        let builtin = cx.state.copy_cell_builtin_model.is_some();
        let view_count = cx
            .model
            .library_manager
            .get_library(&cx.state.copy_cell_source_library)
            .and_then(|lib| lib.get_cell(&cx.state.copy_cell_source_cell))
            .map(|cell| cell.views.len())
            .unwrap_or(0);
        let content = if builtin {
            "The model's source and its includes, as an editable cell.".to_owned()
        } else {
            match view_count {
                1 => "Its view and its content.".to_owned(),
                count => format!("All {count} views and their content."),
            }
        };
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.copy_cell_library_revision;
        let error = (!catalog_current)
            .then(|| LIBRARY_CATALOG_STALE_MESSAGE.to_owned())
            .or_else(|| cx.state.copy_cell_error.clone());
        let dialogs = &mut *cx.state;
        let source = format!(
            "{}/{}",
            dialogs.copy_cell_source_library, dialogs.copy_cell_source_cell
        );
        let mut edited = false;
        let choice = CommandForm::new("Copy cell", "Copy")
            .describe(
                "Copy every view and its content from the selected cell into a writable library under a new name.",
            )
            .primary_enabled(
                catalog_current && !dialogs.copy_cell_name.trim().is_empty() && !targets.is_empty(),
            )
            .status(error.as_deref().map_or(Ok(None), Err))
            .show(ctx, |rows| {
                rows.value("Source", &source);
                rows.derived(&content);
                if let Some(index) = rows.select(
                    "To library",
                    "copy-cell-target",
                    &dialogs.copy_cell_target_library,
                    &targets,
                ) {
                    dialogs.copy_cell_target_library.clone_from(&targets[index]);
                    edited = true;
                }
                let name = egui::Id::new("copy-cell-name");
                edited |= rows
                    .text_sans("New name", name, &mut dialogs.copy_cell_name, "")
                    .changed();
                if !builtin {
                    edited |= rows.checks(
                        "Hierarchy",
                        &mut [(
                            "Include the cells it instantiates",
                            &mut dialogs.copy_cell_include_children,
                        )],
                    );
                    if dialogs.copy_cell_include_children {
                        edited |= rows.checks(
                            "Instances",
                            &mut [(
                                "Point at the copied cells",
                                &mut dialogs.copy_cell_repoint,
                            )],
                        );
                    }
                }
                Some(name)
            });
        // A refusal belongs to what it refused; an edited form has not been
        // refused yet.
        if edited && catalog_current {
            cx.state.copy_cell_error = None;
        }
        let close = match choice {
            DialogChoice::Primary => {
                let source_cell = cx.state.copy_cell_source_cell.clone();
                let target = cx.state.copy_cell_target_library.clone();
                let name = cx.state.copy_cell_name.trim().to_owned();
                match commit_copy_cell_dialog(cx) {
                    Ok((cells, views)) => {
                        cx.model.record(
                            LogSeverity::Info,
                            LogSource::Studio,
                            if builtin {
                                format!(
                                    "Copied '{source_cell}' to '{target}/{name}'; compile it before use."
                                )
                            } else if cells > 1 {
                                format!(
                                    "Copied '{source_cell}' to '{target}/{name}' with {} cells it instantiates ({views} views).",
                                    cells - 1
                                )
                            } else {
                                format!("Copied '{source_cell}' to '{target}/{name}' ({views} views).")
                            },
                        );
                        true
                    }
                    Err(error) => {
                        cx.state.copy_cell_error = Some(error);
                        false
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => true,
            DialogChoice::Secondary | DialogChoice::None => false,
        };
        if close {
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
            .description("Open tabs and every placed instance follow the new name.")
            .size(DialogSize::Transaction)
            .ghost("Cancel")
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
                        mutation: None,
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
    use crate::workbench::project::tests::harness;

    #[test]
    fn rename_cell_dialog_starts_with_the_exact_active_cell_identity() {
        let mut harness = harness();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        harness.model.library_manager.add_library(library);

        harness
            .state
            .open_rename_cell_dialog(&harness.model, "work", "amplifier")
            .expect("default work/amplifier cell is writable");

        assert!(harness.state.rename_cell_dialog);
        assert_eq!(harness.state.rename_cell_library, "work");
        assert_eq!(harness.state.rename_cell_current, "amplifier");
        assert_eq!(harness.state.rename_cell_name, "amplifier");
        assert!(harness.state.rename_cell_error.is_none());
        assert_eq!(
            harness.state.rename_cell_library_revision,
            harness.model.library_manager.revision()
        );
    }

    #[test]
    fn dialog_entry_points_reject_stale_targets_without_mutating_modal_state() {
        let mut harness = harness();
        let mut library = Library::new("work");
        library.add_cell(Cell::new("amplifier"));
        harness.model.library_manager.add_library(library);
        harness.state.copy_cell_source_library = "sentinel".to_owned();
        harness.state.rename_cell_library = "sentinel".to_owned();

        assert!(
            harness
                .state
                .open_copy_cell_dialog(&harness.model, "missing", "amplifier")
                .is_err()
        );
        assert!(
            harness
                .state
                .open_rename_cell_dialog(&harness.model, "work", "missing")
                .is_err()
        );

        assert!(!harness.state.copy_cell_dialog);
        assert!(!harness.state.rename_cell_dialog);
        assert_eq!(harness.state.copy_cell_source_library, "sentinel");
        assert_eq!(harness.state.rename_cell_library, "sentinel");
    }

    #[test]
    fn rename_entry_point_rejects_read_only_library_without_mutation() {
        let mut harness = harness();
        let mut work = Library::new("work");
        work.add_cell(Cell::new("amplifier"));
        harness.model.library_manager.add_library(work);
        let library = harness
            .model
            .library_manager
            .get_library_mut("work")
            .expect("default work library exists");
        library.read_only = true;

        let error = harness
            .state
            .open_rename_cell_dialog(&harness.model, "work", "amplifier")
            .expect_err("read-only libraries must fail closed");

        assert!(error.contains("read-only"));
        assert!(!harness.state.rename_cell_dialog);
        assert!(harness.state.rename_cell_library.is_empty());
    }

    #[test]
    fn copy_and_rename_dialog_commits_reject_intervening_catalog_change() {
        let mut copy_harness = harness();
        let mut copy_library = Library::new("work");
        copy_library.add_cell(Cell::new("amplifier"));
        copy_harness.model.library_manager.add_library(copy_library);
        copy_harness
            .state
            .open_copy_cell_dialog(&copy_harness.model, "work", "amplifier")
            .expect("copy dialog opens");
        assert_eq!(
            copy_harness.state.copy_cell_library_revision,
            copy_harness.model.library_manager.revision()
        );
        copy_harness
            .model
            .library_manager
            .add_library(Library::new("intervening"));

        let copy_error = commit_copy_cell_dialog(&mut copy_harness.cx())
            .expect_err("stale copy must fail closed");

        assert_eq!(copy_error, LIBRARY_CATALOG_STALE_MESSAGE);
        assert!(
            copy_harness
                .model
                .library_manager
                .get_library("work")
                .is_some_and(|library| library.get_cell("amplifier_copy").is_none())
        );

        let mut rename_harness = harness();
        let mut rename_library = Library::new("work");
        rename_library.add_cell(Cell::new("amplifier"));
        rename_harness
            .model
            .library_manager
            .add_library(rename_library);
        rename_harness
            .state
            .open_rename_cell_dialog(&rename_harness.model, "work", "amplifier")
            .expect("rename dialog opens");
        rename_harness.state.rename_cell_name = "amplifier_v2".to_owned();
        rename_harness
            .model
            .library_manager
            .add_library(Library::new("intervening"));

        let rename_error = commit_rename_cell_dialog(&mut rename_harness.cx())
            .expect_err("stale rename must fail closed");

        assert_eq!(rename_error, LIBRARY_CATALOG_STALE_MESSAGE);
        let library = rename_harness
            .model
            .library_manager
            .get_library("work")
            .expect("source library remains");
        assert!(library.get_cell("amplifier").is_some());
        assert!(library.get_cell("amplifier_v2").is_none());
    }
}
