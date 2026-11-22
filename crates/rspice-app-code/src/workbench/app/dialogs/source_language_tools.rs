//! Rename symbol, Go to definition and Find references for Verilog-A
//! sources, from the caret of the source editor on screen.
//!
//! Go to definition moves the caret to the definition, opening its file when
//! it lives in another file of the bundle. Find references lists every
//! occurrence in the bottom dock's References page, one row each with its
//! file, line, column and text, and states the count on the status bar.
//! Rename symbol is a command form: the new name, how many occurrences in
//! how many files it rewrites, and each line it changes, before and after.
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

use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine, TableColumn};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::documents::code_workspace::{
    self, CodeWorkspaceParts, CodeWorkspaceRuntimeState,
};
use crate::workbench::{MessageCatalog, MessageId};

const NEW_NAME_FIELD: &str = "workbench.rename-symbol.new-name";

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, in the Verilog-A bundle it shows, through the
/// project source index. A netlist answers through its own language service
/// instead, which the shell asks.
pub fn navigate_source_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    command: CodeCommand,
) -> Result<(), String> {
    let messages = parts.wb.messages();
    let no_bundle = || messages.text(MessageId::SourceNoBundle);
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle()
        .map(crate::state::ProjectSourceBundle::id)
        .ok_or_else(no_bundle)?;
    let logical_path = code_workspace::active_bundle_path(&parts.model.workspace, parts.code)
        .ok_or_else(no_bundle)?;
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
    if let Err(error) =
        code_workspace::open_language_tools_in_bundle(parts, bundle_id, &logical_path, caret)
    {
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
            return Err(messages.text(MessageId::SourcePlaceCaretToRename));
        }
        if let Some(tools) = parts.code.language_tools.as_mut() {
            tools.replacement = caret_name;
        }
        return Ok(());
    }
    let tools = std::mem::replace(&mut parts.code.language_tools, previous)
        .ok_or_else(|| messages.text(MessageId::SourceIndexUnavailable))?;
    if command == CodeCommand::FindReferences {
        if caret_name.is_empty() {
            return Err(messages.text(MessageId::SourcePlaceCaretToFindReferences));
        }
        list_references(parts, &tools.index, &logical_path, caret, &caret_name);
        return Ok(());
    }
    if caret_name.is_empty() {
        return Err(messages.text(MessageId::SourcePlaceCaretToGoToDefinition));
    }
    let definition = tools
        .index
        .definition_at(&logical_path, caret, &caret_name)
        .ok_or_else(|| messages.format(MessageId::SourceNoDefinition, &[("name", &caret_name)]))?;
    if !crate::state::project_source_paths_equal(&definition.logical_path, &logical_path) {
        parts.code.veriloga.selected_file = Some(code_workspace::VerilogAFileSelection {
            bundle_id,
            logical_path: definition.logical_path.clone(),
        });
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
            include: None,
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
    let messages = parts.wb.messages();
    parts.wb.prompt(match references.len() {
        0 => messages.format(MessageId::SourceNoReferences, &[("name", name)]),
        count => messages.format(
            MessageId::SourceReferencesFound,
            &[
                (
                    "references",
                    &messages.counted(
                        count,
                        MessageId::CountOneReference,
                        MessageId::CountReferences,
                    ),
                ),
                ("name", name),
                (
                    "files",
                    &messages.counted(files, MessageId::CountOneFile, MessageId::CountFiles),
                ),
            ],
        ),
    });
}

/// Rename symbol: the name under the caret, the new name, and each line
/// renaming to it changes, before and after, worked out once for each new
/// name; or why it refuses. Over the Code workspace's state, the model and
/// the platform's state. Enter renames; Esc and Cancel close. Returns
/// whether the reader asked for the rename.
pub(in crate::workbench) fn render_rename_symbol_form(
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    platform: &PlatformState,
    ctx: &Context,
    messages: MessageCatalog,
) -> bool {
    let Some(tools) = code.language_tools.as_ref() else {
        return false;
    };
    let (bundle_id, active_path) = (tools.index.bundle_id, tools.active_path.clone());
    let blocked = code_workspace::source_file_mutation_block_reason_in(model, code, platform)
        .map(str::to_owned)
        .or_else(|| {
            (!code_workspace::source_bundle_document_is_editable_in(
                model,
                code,
                platform,
                bundle_id,
                &active_path,
            ))
            .then(|| messages.format(MessageId::SourceRenameReadOnly, &[("file", &active_path)]))
        });
    let Some(tools) = code.language_tools.as_mut() else {
        return false;
    };
    let current = model
        .workspace
        .content
        .project_sources
        .get_bundle(bundle_id)
        .is_some_and(|bundle| tools.reads(model.workspace.content.project.id(), bundle));
    let unchanged = tools.unchanged();
    let refusal = if let Some(error) = tools.error.clone() {
        Some(error)
    } else if !current {
        Some(messages.text(MessageId::SourceRenameSourceChanged))
    } else if let Some(reason) = blocked {
        Some(reason)
    } else if unchanged {
        None
    } else {
        tools.plan(messages).as_ref().err().cloned()
    };
    let plan = (refusal.is_none() && !unchanged)
        .then(|| tools.plan(messages).as_ref().ok())
        .flatten();
    let enter_name = messages.text(MessageId::SourceRenameEnterName);
    let status = match (refusal.as_deref(), unchanged) {
        (Some(refusal), _) => Err(refusal),
        (None, true) => Ok(Some(enter_name.as_str())),
        (None, false) => Ok(None),
    };
    let extent = plan.map(|plan| {
        messages.format(
            MessageId::SourceRenameExtent,
            &[(
                "extent",
                &code_workspace::extent_words(messages, plan.places, plan.files),
            )],
        )
    });
    let room = preview_room(ctx.content_rect().width());
    let table = plan.map(|plan| {
        plan.rows
            .iter()
            .map(|row| {
                let (before, after) = around_the_change(&row.before, &row.after, room);
                vec![row.file.clone(), row.line.to_string(), before, after]
            })
            .collect::<Vec<_>>()
    });
    let ready = plan.is_some();
    let title = messages.text(MessageId::SourceRenameTitle);
    let primary = messages.text(MessageId::SourceRenamePrimary);
    let symbol = messages.text(MessageId::SourceRenameSymbol);
    let new_name_label = messages.text(MessageId::SourceRenameNewName);
    let cancel = messages.text(MessageId::CommonCancel);
    let headers = rename_columns(messages);

    let mut replacement = tools.replacement.clone();
    let mut edited = false;
    let choice = CommandForm::new(&title, &primary)
        .wide()
        .cancel_label(&cancel)
        .fit_labels(ctx, &[&symbol, &new_name_label])
        .primary_enabled(ready)
        .status(status)
        .show(ctx, |rows| {
            let field = Id::new(NEW_NAME_FIELD);
            rows.value_mono(&symbol, &tools.query);
            edited = rows
                .text(&new_name_label, field, &mut replacement, "")
                .changed();
            if let (Some(extent), Some(table)) = (extent.as_deref(), table.as_deref()) {
                rows.field_line(Some(field), FieldLine::Derived(extent));
                rows.table(
                    &headers
                        .each_ref()
                        .map(|header| TableColumn { header, mono: true }),
                    table,
                );
            }
            Some(field)
        });
    tools.replacement = replacement;
    if edited {
        tools.error = None;
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

/// The columns of a rename's preview: each line it changes, by file and
/// line, before and after.
fn rename_columns(messages: MessageCatalog) -> [String; 4] {
    [
        MessageId::SourceRenameColumnFile,
        MessageId::SourceRenameColumnLine,
        MessageId::SourceRenameColumnBefore,
        MessageId::SourceRenameColumnAfter,
    ]
    .map(|header| messages.text(header))
}

const DECK_NEW_NAME_FIELD: &str = "workbench.rename-symbol.deck-new-name";

/// The most characters of a changed line the rename preview shows: enough
/// to read the name in its place on a long card.
const PREVIEW_ROOM: usize = 20;

/// The width of a wide form's body, what its File and Line columns and the
/// gaps between the columns take of it, and one character of the table's
/// code, in points: what decides how much of a line fits a Before or After
/// cell on a narrow screen.
const WIDE_FORM_BODY: f32 = 455.0;
const FILE_AND_LINE: f32 = 150.0;
const CODE_CHARACTER: f32 = 7.0;

/// How many characters of a changed line fit a Before or After cell of the
/// preview on a screen `screen` points wide.
fn preview_room(screen: f32) -> usize {
    let body = (screen - 48.0).min(WIDE_FORM_BODY);
    let room = ((body - FILE_AND_LINE) / 2.0 / CODE_CHARACTER).floor();
    (room.max(8.0) as usize).min(PREVIEW_ROOM)
}

/// A changed line, before and after, cut to the same window about its first
/// change, starting on a word, so a long card still shows the name the
/// rename changes; `…` stands for what is cut.
fn around_the_change(before: &str, after: &str, room: usize) -> (String, String) {
    let was = before.chars().collect::<Vec<_>>();
    let is = after.chars().collect::<Vec<_>>();
    let first = was
        .iter()
        .zip(&is)
        .take_while(|(was, is)| was == is)
        .count();
    // A change near the start shows the line from its start; one further
    // in, the words just before it. Both lines read the same up to the
    // change, so one window start serves both.
    let mut start = if first <= room / 2 {
        0
    } else {
        (first - room / 3)
            .min(was.len().saturating_sub(room))
            .min(is.len().saturating_sub(room))
    };
    while start > 0 && start < first && was[start - 1] != ' ' {
        start += 1;
    }
    let cut = |chars: &[char]| {
        if chars.len() <= room {
            return chars.iter().collect::<String>();
        }
        let end = (start + room).min(chars.len());
        let mut shown = String::new();
        if start > 0 {
            shown.push('\u{2026}');
        }
        shown.extend(&chars[start..end]);
        if end < chars.len() {
            shown.push('\u{2026}');
        }
        shown
    };
    (cut(&was), cut(&is))
}

/// Rename symbol over the deck: the name under the caret, the new name, and
/// each line renaming to it changes, before and after; or why it refuses,
/// naming the file and line. Enter renames, as one change Undo takes back a
/// file at a time; Esc and Cancel close.
pub(in crate::workbench) fn render_deck_rename_form(
    cx: &mut crate::workbench::framework::feature::Cx<'_, crate::workbench::code::CodeFeature>,
    ctx: &Context,
) {
    use crate::workbench::documents::netlist_document::references;

    let Some(mut rename) = cx.state.deck_rename.take() else {
        return;
    };
    let messages = cx.wb.messages();
    let unchanged = rename.unchanged();
    let refusal = if !rename.is_current(cx.model, &cx.state.netlist) {
        Some(messages.text(MessageId::SourceRenameDeckChanged))
    } else if let Some(error) = rename.error.clone() {
        Some(error)
    } else if unchanged {
        None
    } else {
        rename.plan().as_ref().err().cloned()
    };
    let plan = (refusal.is_none() && !unchanged)
        .then(|| rename.plan().as_ref().ok().cloned())
        .flatten();
    let enter_name = messages.text(MessageId::SourceRenameEnterName);
    let status = match (refusal.as_deref(), unchanged) {
        (Some(refusal), _) => Err(refusal),
        (None, true) => Ok(Some(enter_name.as_str())),
        (None, false) => Ok(None),
    };
    let extent = plan.as_ref().map(|plan| {
        messages.format(
            MessageId::SourceRenameExtent,
            &[(
                "extent",
                &code_workspace::extent_words(messages, plan.places, plan.file_count()),
            )],
        )
    });
    let room = preview_room(ctx.content_rect().width());
    let table = plan.as_ref().map(|plan| {
        plan.rows
            .iter()
            .map(|row| {
                let (before, after) = around_the_change(&row.before, &row.after, room);
                vec![row.file.clone(), row.line.to_string(), before, after]
            })
            .collect::<Vec<_>>()
    });
    let label = messages.text(rename.caret.kind_words().1);
    let title = messages.text(MessageId::SourceRenameTitle);
    let primary = messages.text(MessageId::SourceRenamePrimary);
    let new_name_label = messages.text(MessageId::SourceRenameNewName);
    let cancel = messages.text(MessageId::CommonCancel);
    let headers = rename_columns(messages);

    let mut replacement = rename.replacement.clone();
    let mut edited = false;
    let choice = CommandForm::new(&title, &primary)
        .wide()
        .cancel_label(&cancel)
        .fit_labels(ctx, &[&label, &new_name_label])
        .primary_enabled(plan.is_some())
        .status(status)
        .show(ctx, |rows| {
            let field = Id::new(DECK_NEW_NAME_FIELD);
            rows.value_mono(&label, &rename.caret.written);
            edited = rows
                .text(&new_name_label, field, &mut replacement, "")
                .changed();
            if let (Some(extent), Some(table)) = (extent.as_deref(), table.as_deref()) {
                rows.field_line(Some(field), FieldLine::Derived(extent));
                rows.table(
                    &headers
                        .each_ref()
                        .map(|header| TableColumn { header, mono: true }),
                    table,
                );
            }
            Some(field)
        });
    rename.replacement = replacement;
    if edited {
        rename.error = None;
    }
    match choice {
        DialogChoice::Primary => {
            match references::commit_rename(cx.model, &mut cx.state.netlist, &mut rename) {
                Ok(done) => cx.wb.prompt(done),
                Err(error) => {
                    rename.error = Some(error);
                    cx.state.deck_rename = Some(rename);
                }
            }
        }
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {}
        DialogChoice::None => cx.state.deck_rename = Some(rename),
    }
}
