//! Rename symbol, Go to definition and Find references for Verilog-A and
//! Automation sources, from the caret of the source editor on screen.
//!
//! Go to definition moves the caret to the definition, opening its file when
//! it lives in another file of the bundle. Find references lists every
//! occurrence in the bottom dock's References page, one row each with its
//! file, line, column and text, and states the count on the status bar.
//! Rename symbol is a command form: the new name, and how many occurrences in
//! how many files it rewrites.
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

use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::documents::code_workspace::{
    self, CodeWorkspaceParts, CodeWorkspaceRuntimeState,
};

pub const RENAME_TITLE: &str = "Rename symbol";
const RENAME_PRIMARY: &str = "Rename";
const NEW_NAME_FIELD: &str = "workbench.rename-symbol.new-name";

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, in the `language` bundle it shows, through the
/// project source index. A netlist answers through its own language service
/// instead, which the shell asks.
pub fn navigate_source_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: crate::state::ProjectSourceLanguage,
    command: CodeCommand,
) -> Result<(), String> {
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle(language)
        .map(crate::state::ProjectSourceBundle::id)
        .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    let logical_path =
        code_workspace::active_bundle_path(&parts.model.workspace, parts.code, language)
            .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    // The caret the editor last reported for this document.
    let caret = parts
        .code
        .source_carets
        .get(&(bundle_id, logical_path.clone()))
        .copied()
        .unwrap_or(0);
    // Built from the exact closure on screen. Rename keeps it as the form's
    // state; the other two borrow it and leave an open form as they found it.
    let previous = parts.code.language_tools.take();
    if let Err(error) = code_workspace::open_language_tools_in_bundle(
        parts,
        language,
        bundle_id,
        &logical_path,
        caret,
    ) {
        parts.code.language_tools = previous;
        return Err(error);
    }
    let caret_name = parts
        .code
        .language_tools
        .as_ref()
        .map(|tools| tools.query.clone())
        .unwrap_or_default();
    if command == CodeCommand::RenameSymbol {
        if caret_name.is_empty() {
            parts.code.language_tools = previous;
            return Err("Place the caret on a name to rename it.".to_owned());
        }
        if let Some(tools) = parts.code.language_tools.as_mut() {
            tools.replacement = caret_name;
        }
        return Ok(());
    }
    let tools = std::mem::replace(&mut parts.code.language_tools, previous)
        .ok_or_else(|| "The source index could not be built.".to_owned())?;
    if command == CodeCommand::FindReferences {
        if caret_name.is_empty() {
            return Err("Place the caret on a name to find its references.".to_owned());
        }
        list_references(parts, &tools.index, &logical_path, caret, &caret_name);
        return Ok(());
    }
    if caret_name.is_empty() {
        return Err("Place the caret on a name to go to its definition.".to_owned());
    }
    let definition = tools
        .index
        .definition_at(&logical_path, caret, &caret_name)
        .ok_or_else(|| format!("No definition of {caret_name} exists in this source bundle."))?;
    if !crate::state::project_source_paths_equal(&definition.logical_path, &logical_path) {
        match language {
            crate::state::ProjectSourceLanguage::VerilogA => {
                parts.code.veriloga.selected_file = Some(code_workspace::VerilogAFileSelection {
                    bundle_id,
                    logical_path: definition.logical_path.clone(),
                });
            }
            crate::state::ProjectSourceLanguage::RSpiceAutomation => {
                parts.code.automation.selected_file = Some(definition.logical_path.clone());
            }
        }
    }
    // The page hands this to the editor it draws next, which is the
    // definition's file.
    parts.wb.outbox.push(
        crate::workbench::framework::requests::SourceEditorRequest::RevealLine(definition.line),
    );
    Ok(())
}

/// Find references: one row per occurrence on the References page of the
/// bottom dock, the page brought forward, and the count on the status bar.
fn list_references(
    parts: &mut CodeWorkspaceParts<'_>,
    index: &code_workspace::LanguageServiceIndex,
    logical_path: &str,
    caret: usize,
    name: &str,
) {
    let references = index.references_at(logical_path, caret, name);
    let files = references
        .iter()
        .map(|reference| crate::state::project_source_path_key(&reference.logical_path))
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let rows = references
        .iter()
        .map(|reference| crate::workbench::state::SourceReference {
            logical_path: reference.logical_path.clone(),
            line: reference.line,
            column: reference.column,
            text: reference.line_preview.clone(),
        })
        .collect::<Vec<_>>();
    let revision = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(index.bundle_id)
        .map_or(0, |bundle| bundle.revision().get());
    if !rows.is_empty() {
        parts
            .wb
            .show_references(crate::workbench::state::SourceReferences::new(
                name.to_owned(),
                index.bundle_id,
                revision,
                rows,
            ));
    }
    parts.wb.prompt(match references.len() {
        0 => format!("No references to {name} in this source bundle."),
        count => format!(
            "{count} reference{} to {name} in {files} file{}.",
            if count == 1 { "" } else { "s" },
            if files == 1 { "" } else { "s" }
        ),
    });
}

/// Rename symbol: the name under the caret, the new name, and what the
/// rename rewrites, over the Code workspace's state, the model and the
/// platform's state. Enter renames; Esc and Cancel close. Returns whether
/// the reader asked for the rename.
pub(in crate::workbench) fn render_rename_symbol_form(
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    platform: &PlatformState,
    ctx: &Context,
) -> bool {
    let Some(tools) = code.language_tools.clone() else {
        return false;
    };
    let stale = model
        .workspace
        .content
        .project_sources
        .get_bundle(tools.index.bundle_id)
        .is_none_or(|bundle| {
            !tools
                .index
                .is_current(model.workspace.content.project.id(), bundle)
        });
    let blocked = code_workspace::source_file_mutation_block_reason_in(
        model,
        code,
        platform,
        tools.index.language,
    )
    .map(str::to_owned)
    .or_else(|| {
        (!code_workspace::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
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
        tools
            .index
            .rename_preview_at(&tools.active_path, tools.caret, &tools.query, new_name)
            .err()
    };
    let unchanged = new_name.is_empty() || new_name == tools.query;
    let ready = refusal.is_none() && !unchanged;
    let (occurrences, files) =
        tools
            .index
            .rename_extent_at(&tools.active_path, tools.caret, &tools.query);
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
    if let Some(open) = code.language_tools.as_mut() {
        open.replacement = replacement;
        if edited {
            open.error = None;
        }
    }
    match choice {
        DialogChoice::Primary => true,
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
            code.language_tools = None;
            false
        }
        DialogChoice::None => false,
    }
}
