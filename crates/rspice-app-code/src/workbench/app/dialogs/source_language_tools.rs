//! Rename symbol, Go to definition and Find references for Verilog-A
//! sources, from the caret of the source editor on screen.
//!
//! Go to definition moves the caret to the definition, opening its file when
//! it lives in another file of the bundle. Find references lists every
//! occurrence in the bottom dock's References page, one row each with its
//! file, line, column and text, and states the count on the status bar.
//! Rename symbol is a command form: the new name, and under it, edge to
//! edge, how many occurrences in how many files it rewrites and each line it
//! changes, before and after.
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

use crate::ui::widgets::{CommandForm, DialogChoice};
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
    // Taken before the plan, which holds the form's state while it is read.
    let query = tools.query.clone();
    let mut replacement = tools.replacement.clone();
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
    let ready = plan.is_some();
    let title = messages.text(MessageId::SourceRenameTitle);
    let primary = messages.text(MessageId::SourceRenamePrimary);
    let symbol = messages.text(MessageId::SourceRenameSymbol);
    let new_name_label = messages.text(MessageId::SourceRenameNewName);
    let cancel = messages.text(MessageId::CommonCancel);

    let mut edited = false;
    let choice = CommandForm::new(&title, &primary)
        .transaction()
        .cancel_label(&cancel)
        .fit_labels(ctx, &[&symbol, &new_name_label])
        .primary_enabled(ready)
        .status(status)
        .show_over(
            ctx,
            |rows| {
                let field = Id::new(NEW_NAME_FIELD);
                rows.value_mono(&symbol, &query);
                edited = rows
                    .text(&new_name_label, field, &mut replacement, "")
                    .changed();
                Some(field)
            },
            |ui| {
                if let (Some(plan), Some(extent)) = (plan, extent.as_deref()) {
                    rename_preview(ui, messages, extent, &plan.rows);
                }
            },
        );
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

/// The fewest characters a Before or After cell is cut to, however narrow
/// the form: enough to read a name in its place.
const LEAST_ROOM: usize = 8;

/// The least width of a Before or After column, in points: what the
/// form's narrowest card leaves them.
const LEAST_CODE_WIDTH: f32 = 96.0;

/// The most rows the preview shows before it scrolls inside the form.
const PREVIEW_ROWS: usize = 12;

/// A changed line, before and after, each whole when it fits `room`
/// characters. When one does not, both are cut to a window that starts at
/// the same character, so the changed names line up, and holds the changed
/// name whole: only the end further from it is cut, and the near end too
/// only when the name sits far into a line too long on both sides. `…`
/// stands for what is cut.
fn around_the_change(before: &str, after: &str, room: usize) -> (String, String) {
    let room = room.max(LEAST_ROOM);
    let was = before.chars().collect::<Vec<_>>();
    let is = after.chars().collect::<Vec<_>>();
    if was.len() <= room && is.len() <= room {
        return (before.to_owned(), after.to_owned());
    }
    let same_head = was
        .iter()
        .zip(&is)
        .take_while(|(was, is)| was == is)
        .count();
    let same_tail = was
        .iter()
        .rev()
        .zip(is.iter().rev())
        .take_while(|(was, is)| was == is)
        .count()
        .min(was.len().min(is.len()) - same_head);
    // The changed name, whole: the change reaches out to the ends of the
    // name it is made in, which the two lines share.
    let in_name = |ch: &&char| ch.is_alphanumeric() || **ch == '_';
    let name_start = same_head - was[..same_head].iter().rev().take_while(in_name).count();
    let name_end = |chars: &[char]| {
        let changed_to = chars.len() - same_tail;
        changed_to + chars[changed_to..].iter().take_while(in_name).count()
    };
    // The first character a line shows: with a mark for a cut start, and
    // one for a cut end, the name in view from the end it is nearer.
    let start_for = |chars: &[char]| {
        let end = name_end(chars);
        if chars.len() <= room {
            0
        } else if name_start <= chars.len() - end {
            if end < room {
                0
            } else {
                (end + 2 - room).min(name_start)
            }
        } else {
            (chars.len() + 1 - room).min(name_start)
        }
    };
    let start = start_for(&was).max(start_for(&is));
    let cut = |chars: &[char]| {
        let lead = usize::from(start > 0);
        let end = if chars.len() - start + lead <= room {
            chars.len()
        } else {
            start + room - lead - 1
        };
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

/// Under a rename form's fields, edge to edge: a band saying how many
/// places in how many files the rename rewrites, then each line it
/// changes, by file and line, before and after. A Before or After line
/// longer than its column is cut around the change, and its whole shows
/// on hover.
fn rename_preview(
    ui: &mut egui::Ui,
    messages: MessageCatalog,
    extent: &str,
    rows: &[crate::workbench::documents::netlist_document::references::RenameRow],
) {
    use crate::ui::theme::{self, FontWeight};
    use crate::ui::tokens::{self, Tokens};
    use crate::ui::widgets::band_header;
    use crate::ui::widgets::table::{
        CELL_PAD, Cell, Column, DataTable, LineHeights, TABLE_EDGE_INSET,
    };

    band_header(ui, extent, None);
    let [file, line, before, after] = rename_columns(messages);
    let painter = ui.painter().clone();
    let width = |text: &str, font: egui::FontId| {
        painter
            .layout_no_wrap(text.to_owned(), font, egui::Color32::PLACEHOLDER)
            .size()
            .x
    };
    let words = theme::sans(tokens::FS_0, FontWeight::Regular);
    let head = theme::sans(tokens::FS_0, FontWeight::Medium);
    let code = theme::mono(tokens::FS_0, FontWeight::Regular);
    let file_width = rows
        .iter()
        .map(|row| width(&row.file, words.clone()))
        .fold(width(&file, head.clone()), f32::max)
        + TABLE_EDGE_INSET
        + CELL_PAD
        + 2.0;
    let line_width = rows
        .iter()
        .map(|row| width(&row.line.to_string(), code.clone()))
        .fold(width(&line, head.clone()), f32::max)
        + 2.0 * CELL_PAD
        + 2.0;
    // Before and After share what File and Line leave of the width the
    // table reads, and a line may take its cell less the padding, and less
    // the table's edge inset in the last column.
    let table_width = ui
        .available_rect_before_wrap()
        .intersect(ui.clip_rect())
        .width();
    let code_width = ((table_width - file_width - line_width) / 2.0).max(LEAST_CODE_WIDTH);
    let span = code_width - CELL_PAD - TABLE_EDGE_INSET;
    // Every character of the mono face advances alike: measured over a
    // run, so the rounding of one glyph's box is not counted for each.
    let advance = (width(&"0".repeat(32), code.clone()) / 32.0).max(1.0);
    let room = (span / advance).floor() as usize;
    let fits = |shown: &str| width(shown, code.clone()) <= span;
    let columns = [
        Column::words(file, file_width).unsortable(),
        Column::number(line, line_width).unsortable(),
        Column::name(before, LEAST_CODE_WIDTH).unsortable(),
        Column::name(after, LEAST_CODE_WIDTH).unsortable(),
    ];
    let lines = LineHeights::grid(&Tokens::get(ui.ctx()));
    let height = lines.head + lines.row * rows.len().clamp(1, PREVIEW_ROWS) as f32;
    // The table's own height, which the form grows to: laid out in what the
    // form's body has left, it would scroll its rows in that instead.
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), height),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            let _ = DataTable::new("workbench.rename-symbol.preview", &columns, rows.len())
                .max_height(height)
                .show(ui, |index| {
                    let row = &rows[index];
                    let mut room = room;
                    let (before, after) = loop {
                        let (before, after) = around_the_change(&row.before, &row.after, room);
                        if room <= LEAST_ROOM || (fits(&before) && fits(&after)) {
                            break (before, after);
                        }
                        room -= 1;
                    };
                    let cell = |shown: String, whole: &str| {
                        if shown == whole {
                            Cell::code(shown)
                        } else {
                            Cell::code(shown).hint(whole)
                        }
                    };
                    vec![
                        Cell::text(row.file.clone()),
                        Cell::number(row.line.to_string()),
                        cell(before, &row.before),
                        cell(after, &row.after),
                    ]
                });
        },
    );
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
    let label = messages.text(rename.caret.kind_words().1);
    let title = messages.text(MessageId::SourceRenameTitle);
    let primary = messages.text(MessageId::SourceRenamePrimary);
    let new_name_label = messages.text(MessageId::SourceRenameNewName);
    let cancel = messages.text(MessageId::CommonCancel);

    let mut replacement = rename.replacement.clone();
    let mut edited = false;
    let choice = CommandForm::new(&title, &primary)
        .transaction()
        .cancel_label(&cancel)
        .fit_labels(ctx, &[&label, &new_name_label])
        .primary_enabled(plan.is_some())
        .status(status)
        .show_over(
            ctx,
            |rows| {
                let field = Id::new(DECK_NEW_NAME_FIELD);
                rows.value_mono(&label, &rename.caret.written);
                edited = rows
                    .text(&new_name_label, field, &mut replacement, "")
                    .changed();
                Some(field)
            },
            |ui| {
                if let (Some(plan), Some(extent)) = (plan.as_ref(), extent.as_deref()) {
                    rename_preview(ui, messages, extent, &plan.rows);
                }
            },
        );
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

#[cfg(test)]
mod tests {
    use super::around_the_change;

    /// Where the changed name starts in a shown line.
    fn at(shown: &str, name: &str) -> usize {
        shown[..shown.find(name).expect("the name is shown")]
            .chars()
            .count()
    }

    /// A line that fits its cell is shown whole: the preview cut
    /// `parameter ` off a 22-character line it had room for.
    #[test]
    fn a_line_that_fits_is_shown_whole() {
        let (before, after) = around_the_change(
            "parameter real g = 1m;",
            "parameter real conductance = 1m;",
            40,
        );
        assert_eq!(before, "parameter real g = 1m;");
        assert_eq!(after, "parameter real conductance = 1m;");
    }

    /// A long line keeps its changed name in view and is cut at the end
    /// further from it; before and after start at one character, so the
    /// names line up.
    #[test]
    fn a_long_line_is_cut_at_the_end_far_from_the_change() {
        let tail = " w=4u l=180n m=2 nf=4 sa=0.2u sb=0.2u sd=0.3u";
        let (before, after) = around_the_change(
            &format!("XDP in ref o1 o2 t diffpair{tail}"),
            &format!("XDP in ref o1 o2 t pair_core{tail}"),
            40,
        );
        assert!(before.starts_with("XDP in ref"), "{before}");
        assert!(before.ends_with('\u{2026}') && after.ends_with('\u{2026}'));
        assert!(before.chars().count() <= 40 && after.chars().count() <= 40);
        assert_eq!(at(&before, "diffpair"), at(&after, "pair_core"));

        // Near the line's end the start is cut instead, on a word.
        let head = "XLONG in_with_a_long_node_name ref_with_a_long_node_name o1_long";
        let (before, after) = around_the_change(
            &format!("{head} diffpair w=4u"),
            &format!("{head} pair_core w=4u"),
            30,
        );
        assert!(before.starts_with('\u{2026}') && after.starts_with('\u{2026}'));
        assert!(before.ends_with("diffpair w=4u"), "{before}");
        assert!(after.ends_with("pair_core w=4u"), "{after}");
        assert_eq!(at(&before, "diffpair"), at(&after, "pair_core"));
        assert_eq!(after.chars().count(), 30, "the cut fills the room: {after}");
    }

    /// A change deep in a line too long on both sides keeps the changed
    /// name whole in view, with both ends cut: `amplification` and `gain`
    /// share their last letter, which is still shown.
    #[test]
    fn a_change_in_the_middle_of_a_long_line_stays_in_view() {
        let side = "abcdefghij ".repeat(6);
        let (before, after) = around_the_change(
            &format!("{side}gain {side}"),
            &format!("{side}amplification {side}"),
            24,
        );
        assert!(before.contains("gain"), "{before}");
        assert!(after.contains("amplification"), "{after}");
        assert!(before.starts_with('\u{2026}') && before.ends_with('\u{2026}'));
        assert!(after.chars().count() <= 24, "{after}");
        assert_eq!(at(&before, "gain"), at(&after, "amplification"));
    }
}
