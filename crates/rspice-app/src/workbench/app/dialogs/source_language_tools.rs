//! Rename symbol, Go to definition and Find references for Verilog-A and
//! Automation sources, from the caret of the source editor on screen.
//!
//! Go to definition moves the caret to the definition, opening its file when
//! it lives in another file of the bundle. Find references lists every
//! occurrence in the Console, one line each with its file and line, and
//! states the count on the status bar. Rename symbol is a command form: the
//! new name, and how many occurrences in how many files it rewrites.
//!
//! These used to be views of a nine-tab "Source navigation and language
//! tools" dialog (document symbols, completion, hover, signature help,
//! definitions, declarations, references, workspace symbols, code actions)
//! that had to be opened to use any of them. The dialog is deleted; what it
//! answered that a reader acts on is here, and Format document formats from
//! the editor.
//!
//! Every answer is bound to the exact closure the index was built from. A
//! rename refuses rather than applying to a newer revision, and says so.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Id};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app::RSpiceApp;
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace;

const RENAME_TITLE: &str = "Rename symbol";
const RENAME_PRIMARY: &str = "Rename";
const NEW_NAME_FIELD: &str = "workbench.rename-symbol.new-name";

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen.
///
/// A netlist answers through its own language service. Verilog-A and
/// Automation answer through the project source index.
pub(crate) fn navigate_active_source(app: &mut RSpiceApp, command: Command) -> Result<(), String> {
    use crate::workbench::documents::netlist_document::language as netlist;

    let Some(language) = code_workspace::active_source_language(
        app.state.workbench.workspace,
        &app.state.model.workspace,
        app.state.code.code_workspace.page,
    ) else {
        let message = match command {
            Command::GoToDefinition => Some(netlist::go_to_definition_at_cursor(&mut app.state)?),
            Command::FindReferences => Some(netlist::find_references_at_cursor(&mut app.state)?),
            Command::RenameSymbol => {
                netlist::begin_rename_at_cursor(&mut app.state)?;
                None
            }
            _ => None,
        };
        if let Some(message) = message {
            app.state.prompt(message);
        }
        return Ok(());
    };
    let bundle_id = app
        .state
        .model
        .workspace
        .active_source_bundle(language)
        .map(crate::state::ProjectSourceBundle::id)
        .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    let logical_path = code_workspace::active_bundle_path(
        &app.state.model.workspace,
        &app.state.code.code_workspace,
        language,
    )
    .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    // The caret the editor last reported for this document.
    let caret = app
        .state
        .code
        .code_workspace
        .source_carets
        .get(&(bundle_id, logical_path.clone()))
        .copied()
        .unwrap_or(0);
    // Built from the exact closure on screen. Rename keeps it as the form's
    // state; the other two borrow it and leave an open form as they found it.
    let previous = app.state.code.code_workspace.language_tools.take();
    if let Err(error) = code_workspace::open_language_tools_in_bundle(
        app,
        language,
        bundle_id,
        &logical_path,
        caret,
    ) {
        app.state.code.code_workspace.language_tools = previous;
        return Err(error);
    }
    let caret_name = app
        .state
        .code
        .code_workspace
        .language_tools
        .as_ref()
        .map(|tools| tools.query.clone())
        .unwrap_or_default();
    if command == Command::RenameSymbol {
        if caret_name.is_empty() {
            app.state.code.code_workspace.language_tools = previous;
            return Err("Place the caret on a name to rename it.".to_owned());
        }
        if let Some(tools) = app.state.code.code_workspace.language_tools.as_mut() {
            tools.replacement = caret_name;
        }
        return Ok(());
    }
    let tools = std::mem::replace(&mut app.state.code.code_workspace.language_tools, previous)
        .ok_or_else(|| "The source index could not be built.".to_owned())?;
    if command == Command::FindReferences {
        if caret_name.is_empty() {
            return Err("Place the caret on a name to find its references.".to_owned());
        }
        list_references(app, &tools.index, language, &caret_name);
        return Ok(());
    }
    if caret_name.is_empty() {
        return Err("Place the caret on a name to go to its definition.".to_owned());
    }
    let definitions = tools.index.definitions(&caret_name);
    // A definition in the file on screen wins; otherwise the bundle's first.
    let definition = definitions
        .iter()
        .find(|symbol| {
            crate::state::project_source_paths_equal(&symbol.logical_path, &logical_path)
        })
        .or_else(|| definitions.first())
        .ok_or_else(|| format!("No definition of {caret_name} exists in this source bundle."))?;
    if !crate::state::project_source_paths_equal(&definition.logical_path, &logical_path) {
        match language {
            crate::state::ProjectSourceLanguage::VerilogA => {
                app.state.code.code_workspace.veriloga.selected_file =
                    Some(code_workspace::VerilogAFileSelection {
                        bundle_id,
                        logical_path: definition.logical_path.clone(),
                    });
            }
            crate::state::ProjectSourceLanguage::RSpiceAutomation => {
                app.state.code.code_workspace.automation.selected_file =
                    Some(definition.logical_path.clone());
            }
        }
    }
    // The page hands this to the editor it draws next, which is the
    // definition's file.
    app.state.workbench.outbox.push(
        crate::workbench::framework::requests::SourceEditorRequest::RevealLine(definition.line),
    );
    Ok(())
}

/// Find references: one Console line per occurrence ("model.va:6: analog
/// I(p, n) <+ g * V(p, n);"), the Console brought forward, and the count on
/// the status bar.
fn list_references(
    app: &mut RSpiceApp,
    index: &code_workspace::LanguageServiceIndex,
    language: crate::state::ProjectSourceLanguage,
    name: &str,
) {
    let references = index.references(name);
    let source = match language {
        crate::state::ProjectSourceLanguage::VerilogA => LogSource::VerilogA,
        crate::state::ProjectSourceLanguage::RSpiceAutomation => LogSource::Studio,
    };
    let files = references
        .iter()
        .map(|reference| crate::state::project_source_path_key(&reference.logical_path))
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    for reference in &references {
        app.state.record(
            LogSeverity::Info,
            source,
            format!(
                "{}:{}: {}",
                reference.logical_path, reference.line, reference.line_preview
            ),
        );
    }
    if !references.is_empty() {
        app.state.workbench.open_console();
        app.state.workbench.console_page = crate::workbench::state::ConsolePage::Console;
    }
    app.state.prompt(match references.len() {
        0 => format!("No references to {name} in this source bundle."),
        count => format!(
            "{count} reference{} to {name} in {files} file{}, listed in the Console.",
            if count == 1 { "" } else { "s" },
            if files == 1 { "" } else { "s" }
        ),
    });
}

impl RSpiceApp {
    /// Rename symbol: the name under the caret, the new name, and what the
    /// rename rewrites. Enter renames; Esc and Cancel close.
    pub(in crate::workbench) fn render_rename_symbol_form(&mut self, ctx: &Context) {
        let Some(tools) = self.state.code.code_workspace.language_tools.clone() else {
            return;
        };
        let stale = self
            .state
            .model
            .workspace
            .content
            .project_sources
            .get_bundle(tools.index.bundle_id)
            .is_none_or(|bundle| {
                !tools
                    .index
                    .is_current(self.state.model.workspace.content.project.id(), bundle)
            });
        let blocked = code_workspace::source_file_mutation_block_reason(self, tools.index.language)
            .map(str::to_owned)
            .or_else(|| {
                (!code_workspace::source_bundle_document_is_editable(
                    self,
                    tools.index.language,
                    tools.index.bundle_id,
                    &tools.active_path,
                ))
                .then(|| {
                    format!(
                        "{} is read-only, so nothing in it is renamed.",
                        tools.active_path
                    )
                })
            });
        let new_name = tools.replacement.trim();
        let refusal = if let Some(error) = tools.error.clone() {
            Some(error)
        } else if stale {
            Some("The source changed after Rename opened. Close it and rename again.".to_owned())
        } else if let Some(reason) = blocked {
            Some(reason)
        } else if new_name.is_empty() || new_name == tools.query {
            None
        } else {
            tools.index.rename_preview(&tools.query, new_name).err()
        };
        let unchanged = new_name.is_empty() || new_name == tools.query;
        let ready = refusal.is_none() && !unchanged;
        let (occurrences, files) = tools.index.rename_extent(&tools.query);
        let extent = format!(
            "Renames {}",
            code_workspace::extent_words(occurrences, files)
        );
        let status = match (refusal.as_deref(), unchanged) {
            (Some(refusal), _) => Err(refusal),
            (None, true) => Ok(Some("Enter a new name.")),
            (None, false) => Ok(None),
        };

        let mut replacement = tools.replacement.clone();
        let mut edited = false;
        let choice = CommandForm::new(RENAME_TITLE, RENAME_PRIMARY)
            .primary_enabled(ready)
            .status(status)
            .show(ctx, |rows| {
                let field = Id::new(NEW_NAME_FIELD);
                rows.value_mono("Symbol", &tools.query);
                edited = rows.text("New name", field, &mut replacement, "").changed();
                rows.field_line(Some(field), FieldLine::Derived(&extent));
                Some(field)
            });
        if let Some(open) = self.state.code.code_workspace.language_tools.as_mut() {
            open.replacement = replacement;
            if edited {
                open.error = None;
            }
        }
        match choice {
            DialogChoice::Primary => {
                if let Err(error) = code_workspace::commit_language_rename(self)
                    && let Some(open) = self.state.code.code_workspace.language_tools.as_mut()
                {
                    open.error = Some(error);
                }
            }
            DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
                self.state.code.code_workspace.language_tools = None;
            }
            DialogChoice::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `resolve_veriloga_compile` was written to gate the compile command and
    /// never attached to it, so the button stayed enabled while a compile was
    /// already running and the surface then refused the request.
    #[test]
    fn compiling_verilog_a_is_refused_while_a_compile_is_already_pending() {
        let mut app = RSpiceApp::test_instance();
        // A Verilog-A cell view open in Design: the place Compile is offered.
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "gain".to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(
                crate::state::ProjectSourceBundle::try_new(
                    crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                    crate::state::ProjectSourceLanguage::VerilogA,
                    "gain.va",
                    "module gain(p, n); inout p, n; electrical p, n; endmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        let enabled_before = Command::CompileVerilogA.is_enabled(&app);
        assert!(
            enabled_before,
            "a writable project compiles the Verilog-A cell view in front"
        );

        app.state.model.safe_mode.active = true;
        app.state.model.safe_mode.applied.open_project_read_only = true;
        assert!(
            !Command::CompileVerilogA.is_enabled(&app),
            "a read-only project cannot compile; it was enabled before as {enabled_before}"
        );
    }
}
