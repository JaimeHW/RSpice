//! File > Import Verilog-A/AMS: a root file and every file its includes
//! reach become cell views of their own, whatever view is in front.
//! Replacing an open view's source from a file is the Verilog-A editor's own
//! command, and waits for the confirmation below.
//!
//! The form lists what the root's includes reach: each file found, each
//! standard header the compiler supplies, and each file still missing, which
//! the reader locates rather than having the import refused. It asks which
//! library, and which top modules get a cell, and states what the answer
//! makes. Cells are named by the rule a migrated project-wide source follows,
//! so the two can never disagree about where a module lands. Nothing is
//! created while a file is missing or the source cannot be read.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::workspace::ImportedInclude;
use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::state::workspace::{DeclaredModule, declared_cell_interface, veriloga_cell_for_module};
use crate::state::{
    CellViewRef, ProjectSourceBundle, ProjectSourceDependency, ProjectSourceFile,
    ProjectSourceLanguage, ProjectSourceOwner,
};
use crate::ui::widgets::{CommandForm, DialogChoice, TableColumn};
use crate::workbench::MessageId;
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::session::veriloga_library::persist_global_veriloga_library;
use crate::workbench::documents::code_workspace::{
    VerilogAImportFormState, close_veriloga_import_form, request_veriloga_import_locate,
    writable_libraries,
};

/// Where each top module of the form's file lands in its library: the module,
/// its cell view, and whether that cell is new.
fn planned_cells(
    state: &AppState,
    form: &VerilogAImportFormState,
) -> Vec<(DeclaredModule, CellViewRef, bool)> {
    if form.closure.modules.is_err() {
        return Vec::new();
    }
    let modules = form.chosen_modules();
    let active = &state.model.workspace.content.active_view;
    let schematic_ports = |reference: &CellViewRef| {
        if reference == active {
            return Some(
                state
                    .model
                    .schematic
                    .document()
                    .interface_ports()
                    .into_iter()
                    .map(|port| port.name)
                    .collect(),
            );
        }
        state
            .model
            .workspace
            .content
            .schematic_buffers
            .get(&reference.key())
            .map(|schematic| {
                schematic
                    .document()
                    .interface_ports()
                    .into_iter()
                    .map(|port| port.name)
                    .collect()
            })
    };
    let interface_of = |cell: &crate::state::Cell, reference: &CellViewRef| {
        declared_cell_interface(&schematic_ports, cell, &reference.library)
    };
    let mut reserved = Vec::new();
    modules
        .iter()
        .map(|module| {
            let (reference, new_cell) = veriloga_cell_for_module(
                &state.model.library_manager,
                &interface_of,
                &form.library,
                module,
                &reserved,
            );
            reserved.push(reference.cell.clone());
            (module.clone(), reference, new_cell)
        })
        .collect()
}

/// The Creates row: the cell views the commit makes, one per top module. A
/// module is named once when its cell takes its name, and with the cell it
/// lands in when the name was taken.
fn plan_line(plan: &[(DeclaredModule, CellViewRef, bool)]) -> Option<String> {
    (!plan.is_empty()).then(|| {
        plan.iter()
            .map(|(module, reference, new_cell)| {
                let mut entry = if module.name == reference.cell {
                    module.name.clone()
                } else {
                    format!("{} \u{2192} {}", module.name, reference.cell)
                };
                if !new_cell {
                    entry.push_str(" (existing cell)");
                }
                entry
            })
            .collect::<Vec<_>>()
            .join(", ")
    })
}

/// The files the root's includes reach, as bundle files at the paths the
/// form found them under.
fn imported_files(form: &VerilogAImportFormState) -> Result<Vec<ProjectSourceFile>, String> {
    form.closure
        .files
        .iter()
        .filter(|file| file.include == ImportedInclude::Found)
        .map(|file| {
            let contents = form
                .files
                .get(&file.path)
                .ok_or_else(|| format!("{} is not in the import.", file.path))?;
            ProjectSourceFile::try_new(file.path.clone(), contents.clone())
                .map_err(|error| error.to_string())
        })
        .collect()
}

/// The include each found file was first reached by, as bundle edges.
fn imported_edges(form: &VerilogAImportFormState) -> Result<Vec<ProjectSourceDependency>, String> {
    form.closure
        .edges
        .iter()
        .map(|(importer, imported)| {
            ProjectSourceDependency::try_new(importer.clone(), imported.clone())
                .map_err(|error| error.to_string())
        })
        .collect()
}

/// Why the form cannot create its cells yet, when it cannot: the source does
/// not parse, a file is missing, no library can be written, or no module is
/// chosen. A missing file is stated before a parse failure it may cause.
fn import_refusal(
    messages: crate::workbench::MessageCatalog,
    form: &VerilogAImportFormState,
    libraries: &[String],
) -> Option<String> {
    let missing = form.closure.missing().collect::<Vec<_>>();
    match (missing.as_slice(), &form.closure.modules) {
        ([], Err(reason)) => Some(messages.format(
            MessageId::VerilogAImportRefused,
            &[("file_name", &form.file_name), ("reason", reason)],
        )),
        ([], Ok(_)) if libraries.is_empty() => {
            Some(messages.text(MessageId::VerilogAImportNoLibrary))
        }
        ([], Ok(_)) if form.chosen_modules().is_empty() => {
            Some(messages.text(MessageId::VerilogAImportNoModuleChosen))
        }
        ([], Ok(_)) => form.error.clone(),
        ([file], _) => Some(messages.format(
            MessageId::VerilogAImportMissingOne,
            &[("file", &file.path), ("including", included_by(file))],
        )),
        (files, _) => Some(messages.format(
            MessageId::VerilogAImportMissingMany,
            &[("count", &files.len().to_string())],
        )),
    }
}

/// The file whose directive asks for `file`.
fn included_by(file: &crate::state::workspace::ImportClosureFile) -> &str {
    match &file.include {
        ImportedInclude::Missing { included_by } => included_by,
        ImportedInclude::Found | ImportedInclude::BuiltIn => "",
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_veriloga_import_dialog(&mut self, ctx: &Context) {
        let Some(form) = self.state.code.code_workspace.veriloga.import_form.clone() else {
            return;
        };
        let messages = self.state.workbench.messages();
        let libraries = writable_libraries(&self.state.model.library_manager);
        let plan = planned_cells(&self.state, &form);
        let refusal = import_refusal(messages, &form, &libraries);
        let ready = refusal.is_none() && !plan.is_empty();
        let title = messages.text(MessageId::VerilogAImportTitle);
        let primary = messages.text(MessageId::VerilogAImportPrimary);
        let description = messages.text(MessageId::VerilogAImportDescription);
        let file_label = messages.text(MessageId::VerilogAImportFile);
        let library_label = messages.text(MessageId::VerilogAImportLibrary);
        let modules_label = messages.text(MessageId::VerilogAImportModules);
        let creates_label = messages.text(MessageId::VerilogAImportCreates);
        let missing_label = messages.text(MessageId::VerilogAImportMissing);
        let locate = messages.text(MessageId::VerilogAImportLocate);
        let include_header = messages.text(MessageId::VerilogAImportIncludes);
        let source_header = messages.text(MessageId::VerilogAImportIncludeSource);
        let found = messages.text(MessageId::VerilogAImportIncludeFound);
        let built_in = messages.text(MessageId::VerilogAImportIncludeBuiltIn);
        let columns = [
            TableColumn {
                header: &include_header,
                mono: true,
            },
            TableColumn {
                header: &source_header,
                mono: false,
            },
        ];
        let reached = form
            .closure
            .files
            .iter()
            .filter_map(|file| match file.include {
                ImportedInclude::Found => Some(vec![file.path.clone(), found.clone()]),
                ImportedInclude::BuiltIn => Some(vec![file.path.clone(), built_in.clone()]),
                ImportedInclude::Missing { .. } => None,
            })
            .collect::<Vec<_>>();
        let missing = form
            .closure
            .missing()
            .map(|file| {
                (
                    file.path.clone(),
                    messages.format(
                        MessageId::VerilogAImportMissingFile,
                        &[("file", &file.path), ("including", included_by(file))],
                    ),
                )
            })
            .collect::<Vec<_>>();
        let modules = form
            .closure
            .modules
            .as_ref()
            .map_or_else(|_| Vec::new(), Clone::clone);
        let mut chosen = modules
            .iter()
            .map(|module| form.chosen.contains(&module.name))
            .collect::<Vec<_>>();
        let line = plan_line(&plan);
        let mut picked = None;
        let mut locate_path = None;
        let mut chosen_changed = false;
        let choice = CommandForm::new(&title, &primary)
            .describe(&description)
            .primary_enabled(ready)
            .fields_enabled(!libraries.is_empty())
            .status(match refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .show(ctx, |rows| {
                rows.value_mono(&file_label, &form.file_name);
                if !reached.is_empty() {
                    rows.table(&columns, &reached);
                }
                for (path, text) in &missing {
                    if rows.value_action(&missing_label, text, &locate) {
                        locate_path = Some(path.clone());
                    }
                }
                picked = rows.select(
                    &library_label,
                    "workbench.veriloga-import.library",
                    &form.library,
                    &libraries,
                );
                // One module is chosen by importing at all; several are each
                // the reader's choice.
                if modules.len() > 1 {
                    let mut items = modules
                        .iter()
                        .zip(chosen.iter_mut())
                        .map(|(module, on)| (module.name.as_str(), on))
                        .collect::<Vec<_>>();
                    chosen_changed = rows.checks(&modules_label, &mut items);
                }
                if let Some(line) = line.as_deref() {
                    rows.value_mono(&creates_label, line);
                }
                None
            });
        if let Some(form) = self.state.code.code_workspace.veriloga.import_form.as_mut() {
            if let Some(index) = picked
                && let Some(library) = libraries.get(index)
            {
                form.library = library.clone();
                form.error = None;
            }
            if chosen_changed {
                form.chosen = modules
                    .iter()
                    .zip(&chosen)
                    .filter(|(_, on)| **on)
                    .map(|(module, _)| module.name.clone())
                    .collect();
                form.error = None;
            }
        }
        if let Some(path) = locate_path {
            request_veriloga_import_locate(&mut self.state.code.code_workspace.veriloga, &path);
        }
        match choice {
            DialogChoice::Primary if ready => {
                if let Err(error) = self.commit_veriloga_import()
                    && let Some(form) = self.state.code.code_workspace.veriloga.import_form.as_mut()
                {
                    form.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                close_veriloga_import_form(&mut self.state.code.code_workspace.veriloga);
            }
            DialogChoice::Primary | DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// The confirmation "Replace source from file…" opens once a file is
    /// picked: which file it replaces, with what, and whether the text it
    /// replaces can be restored. Nothing is replaced until it is confirmed.
    pub(in crate::workbench) fn render_veriloga_replace_dialog(&mut self, ctx: &Context) {
        let Some(review) = self
            .state
            .code
            .code_workspace
            .veriloga
            .replace_review
            .clone()
        else {
            return;
        };
        let messages = self.state.workbench.messages();
        let title = messages.text(MessageId::VerilogAReplaceTitle);
        let primary = messages.text(MessageId::VerilogAReplacePrimary);
        let description = messages.text(MessageId::VerilogAReplaceDescription);
        let view_label = messages.text(MessageId::VerilogAReplaceView);
        let replaces = messages.text(MessageId::VerilogAReplaceFile);
        let with = messages.text(MessageId::VerilogAReplaceWith);
        let note = if review.restorable {
            messages.text(MessageId::VerilogAReplaceRestorable)
        } else {
            messages.format(
                MessageId::VerilogAReplaceNotRestorable,
                &[("file", &review.target.logical_path)],
            )
        };
        let view = review.target.view.display_path();
        let choice = CommandForm::new(&title, &primary)
            .describe(&description)
            .status(Ok(Some(note.as_str())))
            .show(ctx, |rows| {
                rows.value_mono(&view_label, &view);
                rows.value_mono(&replaces, &review.target.logical_path);
                rows.value_mono(&with, &review.file_name);
                None
            });
        match choice {
            DialogChoice::Primary => {
                if let Err(error) =
                    crate::workbench::documents::code_workspace::commit_veriloga_source_replacement(
                        self,
                    )
                {
                    self.state
                        .record(LogSeverity::Error, LogSource::Studio, error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.code.code_workspace.veriloga.replace_review = None;
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// Create the form's cells in one library transaction and open the first
    /// in Design. Nothing is published unless every cell, view and bundle is.
    pub(in crate::workbench) fn commit_veriloga_import(&mut self) -> Result<CellViewRef, String> {
        let form = self
            .state
            .code
            .code_workspace
            .veriloga
            .import_form
            .clone()
            .ok_or_else(|| "No Verilog-A import is open.".to_owned())?;
        if form.project_id != self.state.model.workspace.content.project.id() {
            return Err("The active project changed while the import was open.".to_owned());
        }
        if let Some(missing) = form.closure.missing().next() {
            return Err(format!("{} is missing.", missing.path));
        }
        if let Err(reason) = &form.closure.modules {
            return Err(format!("{} cannot be read: {reason}", form.file_name));
        }
        if !writable_libraries(&self.state.model.library_manager).contains(&form.library) {
            return Err(format!("Library '{}' cannot be written.", form.library));
        }
        let plan = planned_cells(&self.state, &form);
        let Some((_, first, first_is_new)) = plan.first().cloned() else {
            return Err(format!("{} declares no module.", form.file_name));
        };

        let root = form
            .files
            .get(&form.file_name)
            .ok_or_else(|| format!("{} is not in the import.", form.file_name))?;
        // One source for every module: the first cell's view owns it, and
        // each other cell's view shares it, so an edit made through any of
        // them is the source all of them compile.
        let bundle = ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(first.clone()),
            ProjectSourceLanguage::VerilogA,
            form.file_name.as_str(),
            root.as_str(),
            imported_files(&form)?,
            imported_edges(&form)?,
        )
        .map_err(|error| error.to_string())?;
        self.state
            .model
            .publish_veriloga_cells(&plan, first_is_new, bundle, None)?;
        close_veriloga_import_form(&mut self.state.code.code_workspace.veriloga);
        if form.library == VERILOGA_LIBRARY_NAME {
            persist_global_veriloga_library(&mut self.state.model);
        }
        // A cell view is a Design document; the first module opens there.
        self.state.open_workspace_view(first.clone());
        self.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        let created = plan
            .iter()
            .map(|(_, reference, _)| reference.display_path())
            .collect::<Vec<_>>()
            .join(", ");
        self.state.record(
            LogSeverity::Info,
            LogSource::Studio,
            format!(
                "Imported {} as {created}; compile it before use.",
                form.file_name
            ),
        );
        Ok(first)
    }
}
