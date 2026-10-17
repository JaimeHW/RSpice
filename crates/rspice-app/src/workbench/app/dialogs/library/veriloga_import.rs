//! File > Import Verilog-A: the file becomes cell views of its own, whatever
//! view is in front. Replacing an open view's source from a file is the
//! Verilog-A editor's own command, and waits for the confirmation below.
//!
//! The form asks one question, which library, and states what the answer
//! makes: one cell per top module the file declares, each with a `veriloga`
//! view that owns a bundle of the file. Cells are named by the rule a migrated
//! project-wide source follows, so the two can never disagree about where a
//! module lands. A file the front end cannot read is refused in the status
//! line and nothing is created.

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::workspace::{
    DeclaredModule, declared_cell_interface, place_veriloga_view, veriloga_cell_for_module,
    veriloga_view,
};
use crate::state::{CellViewRef, ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner};
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::MessageId;
use crate::workbench::app::{RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::{VerilogAImportFormState, writable_libraries};

/// Where each top module of the form's file lands in its library: the module,
/// its cell view, and whether that cell is new.
fn planned_cells(
    state: &AppState,
    form: &VerilogAImportFormState,
) -> Vec<(DeclaredModule, CellViewRef, bool)> {
    let Ok(modules) = &form.modules else {
        return Vec::new();
    };
    let active = &state.workspace.content.active_view;
    let schematic_ports = |reference: &CellViewRef| {
        if reference == active {
            return Some(
                state
                    .schematic
                    .document()
                    .interface_ports()
                    .into_iter()
                    .map(|port| port.name)
                    .collect(),
            );
        }
        state
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
                &state.library_manager,
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

impl RSpiceApp {
    pub(in crate::workbench) fn render_veriloga_import_dialog(&mut self, ctx: &Context) {
        let Some(form) = self.state.ui.code_workspace.veriloga.import_form.clone() else {
            return;
        };
        let messages = self.state.ui.messages();
        let libraries = writable_libraries(&self.state.library_manager);
        let plan = planned_cells(&self.state, &form);
        let refusal = match &form.modules {
            Err(reason) => Some(messages.format(
                MessageId::VerilogAImportRefused,
                &[("file_name", &form.file_name), ("reason", reason)],
            )),
            Ok(_) if libraries.is_empty() => {
                Some(messages.text(MessageId::VerilogAImportNoLibrary))
            }
            Ok(_) => form.error.clone(),
        };
        let ready = refusal.is_none() && !plan.is_empty();
        let title = messages.text(MessageId::VerilogAImportTitle);
        let primary = messages.text(MessageId::VerilogAImportPrimary);
        let description = messages.text(MessageId::VerilogAImportDescription);
        let file_label = messages.text(MessageId::VerilogAImportFile);
        let library_label = messages.text(MessageId::VerilogAImportLibrary);
        let creates_label = messages.text(MessageId::VerilogAImportCreates);
        let line = plan_line(&plan);
        let mut picked = None;
        // Every refusal is known from the file and the libraries when the form
        // opens, so the status takes room only when it has something to say.
        let choice = CommandForm::new(&title, &primary)
            .describe(&description)
            .primary_enabled(ready)
            .fields_enabled(form.modules.is_ok() && !libraries.is_empty())
            .status(match refusal.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .status_fixed_at_open()
            .show(ctx, |rows| {
                rows.value_mono(&file_label, &form.file_name);
                picked = rows.select(
                    &library_label,
                    "workbench.veriloga-import.library",
                    &form.library,
                    &libraries,
                );
                if let Some(line) = line.as_deref() {
                    rows.value_mono(&creates_label, line);
                }
                None
            });
        if let Some(index) = picked
            && let Some(library) = libraries.get(index)
            && let Some(form) = self.state.ui.code_workspace.veriloga.import_form.as_mut()
        {
            form.library = library.clone();
            form.error = None;
        }
        match choice {
            DialogChoice::Primary if ready => {
                if let Err(error) = self.commit_veriloga_import()
                    && let Some(form) = self.state.ui.code_workspace.veriloga.import_form.as_mut()
                {
                    form.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.ui.code_workspace.veriloga.import_form = None;
            }
            DialogChoice::Primary | DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// The confirmation "Replace source from file…" opens once a file is
    /// picked: which file it replaces, with what, and whether the text it
    /// replaces can be restored. Nothing is replaced until it is confirmed.
    pub(in crate::workbench) fn render_veriloga_replace_dialog(&mut self, ctx: &Context) {
        let Some(review) = self.state.ui.code_workspace.veriloga.replace_review.clone() else {
            return;
        };
        let messages = self.state.ui.messages();
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
            .status_lines(2)
            .status_fixed_at_open()
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
                self.state.ui.code_workspace.veriloga.replace_review = None;
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }

    /// Create the form's cells in one library transaction and open the first
    /// in Design. Nothing is published unless every cell, view and bundle is.
    pub(in crate::workbench) fn commit_veriloga_import(&mut self) -> Result<CellViewRef, String> {
        let form = self
            .state
            .ui
            .code_workspace
            .veriloga
            .import_form
            .clone()
            .ok_or_else(|| "No Verilog-A import is open.".to_owned())?;
        if form.project_id != self.state.workspace.content.project.id() {
            return Err("The active project changed while the import was open.".to_owned());
        }
        if let Err(reason) = &form.modules {
            return Err(format!("{} cannot be read: {reason}", form.file_name));
        }
        if !writable_libraries(&self.state.library_manager).contains(&form.library) {
            return Err(format!("Library '{}' cannot be written.", form.library));
        }
        let plan = planned_cells(&self.state, &form);
        let Some((_, first, first_is_new)) = plan.first().cloned() else {
            return Err(format!("{} declares no module.", form.file_name));
        };

        let mut libraries = self.state.library_manager.clone();
        let mut sources = self.state.workspace.content.project_sources.clone();
        for (module, reference, _) in &plan {
            let bundle = ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(reference.clone()),
                ProjectSourceLanguage::VerilogA,
                form.file_name.as_str(),
                form.contents.as_str(),
                [],
                [],
            )
            .map_err(|error| error.to_string())?;
            sources
                .insert_bundle(bundle)
                .map_err(|error| error.to_string())?;
            place_veriloga_view(&mut libraries, reference, veriloga_view(Some(module))?)?;
        }
        let mutation = if first_is_new {
            crate::state::ProjectLibraryMutation::CreateCell {
                library: first.library.clone(),
                cell: first.cell.clone(),
            }
        } else {
            crate::state::ProjectLibraryMutation::CreateView {
                library: first.library.clone(),
                cell: first.cell.clone(),
                view: first.view.clone(),
            }
        };
        let prepared = self.state.preflight_project_library_mutation(mutation)?;
        self.state.workspace.content.project_sources = sources;
        self.state.workspace.content.project_sources_dirty = true;
        self.state.library_manager = libraries;
        self.state.publish_project_library_mutation(prepared);
        self.state.ui.code_workspace.veriloga.import_form = None;
        if form.library == VERILOGA_LIBRARY_NAME {
            self.persist_global_veriloga_library_with_feedback();
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
