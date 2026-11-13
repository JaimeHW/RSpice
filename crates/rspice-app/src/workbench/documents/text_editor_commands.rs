//! Deterministic structural editing shared by every source-language page.
//!
//! EGUI owns ordinary IME, clipboard, selection, and undo input. Commands
//! which operate on complete source lines live here so Netlist, Verilog-A,
//! Python, YAML, and TOML cannot drift into subtly different behavior.

use std::ops::Range;

use egui::{Id, Key, KeyboardShortcut, Modifiers, Ui};

// The workbench session records which source editor a page shows.
pub use crate::workbench::state::ActiveSourceEditor;

use crate::workbench::framework::requests::{
    EditorViewCommand, SourceEditorRequest, StandardCommand, StructuralCommand,
};

pub(crate) mod veriloga_blocks;
pub(crate) mod veriloga_syntax;

#[derive(Debug, Clone, Default)]
struct GoToLineState {
    open: bool,
    input: String,
    /// The field has been given the caret's line. It is given it once, so a
    /// field the reader clears stays clear.
    seeded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EditorSyntax {
    Spice,
    VerilogA,
    Python,
    Yaml,
    Toml,
}

impl EditorSyntax {
    pub(crate) const fn indentation(self) -> &'static str {
        match self {
            Self::Yaml => "  ",
            Self::Spice | Self::VerilogA | Self::Python | Self::Toml => "    ",
        }
    }

    const fn comment(self) -> &'static str {
        match self {
            Self::Spice => "* ",
            Self::VerilogA => "// ",
            Self::Python | Self::Yaml | Self::Toml => "# ",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ByteSelection {
    pub anchor: usize,
    pub cursor: usize,
}

impl ByteSelection {
    fn sorted(self) -> Range<usize> {
        self.anchor.min(self.cursor)..self.anchor.max(self.cursor)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct StructuralEditResult {
    pub changed: bool,
    pub selection: ByteSelection,
}

pub(crate) fn queue_structural_command(
    context: &egui::Context,
    editor_id: Id,
    command: StructuralCommand,
) {
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("pending-structural-command"), command);
    });
}

pub(crate) fn take_queued_structural_command(
    context: &egui::Context,
    editor_id: Id,
) -> Option<StructuralCommand> {
    context.data_mut(|data| {
        data.remove_temp::<StructuralCommand>(editor_id.with("pending-structural-command"))
    })
}

fn queue_view_command(context: &egui::Context, editor_id: Id, command: EditorViewCommand) {
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("pending-editor-view-command"), command);
    });
}

pub(crate) fn take_queued_view_command(
    context: &egui::Context,
    editor_id: Id,
) -> Option<EditorViewCommand> {
    context.data_mut(|data| {
        data.remove_temp::<EditorViewCommand>(editor_id.with("pending-editor-view-command"))
    })
}

fn queue_standard_command(context: &egui::Context, editor_id: Id, command: StandardCommand) {
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("pending-standard-command"), command);
    });
}

pub(crate) fn take_queued_standard_command(
    context: &egui::Context,
    editor_id: Id,
) -> Option<StandardCommand> {
    context.data_mut(|data| {
        data.remove_temp::<StandardCommand>(editor_id.with("pending-standard-command"))
    })
}

/// Hand `request` to the editor `editor_id`, which applies it the next time
/// it is drawn.
pub(crate) fn queue_source_editor_request(
    context: &egui::Context,
    editor_id: Id,
    request: SourceEditorRequest,
) {
    match request {
        SourceEditorRequest::Standard(command) => {
            queue_standard_command(context, editor_id, command);
        }
        SourceEditorRequest::Structural(command) => {
            queue_structural_command(context, editor_id, command);
        }
        SourceEditorRequest::View(command) => queue_view_command(context, editor_id, command),
        SourceEditorRequest::GoToLine => queue_go_to_line(context, editor_id),
        SourceEditorRequest::FormatDocument => queue_format_document(context, editor_id),
        SourceEditorRequest::RevealLine(line) => queue_reveal_line(context, editor_id, line),
    }
    context.request_repaint();
}

pub(crate) fn queue_go_to_line(context: &egui::Context, editor_id: Id) {
    let state_id = editor_id.with("go-to-line-state");
    let mut state = context
        .data_mut(|data| data.get_temp::<GoToLineState>(state_id))
        .unwrap_or_default();
    state.open = true;
    state.seeded = false;
    context.data_mut(|data| data.insert_temp(state_id, state));
}

/// Ask the editor to place its caret at an exact character index next frame.
///
/// A completion splices into the document buffer, so the edit reaches the
/// editor the same way typing does — through the source it synchronizes
/// against. The caret cannot be moved directly for the same reason, hence a
/// request that is applied after that synchronization rather than before it.
pub fn queue_caret_char_index(context: &egui::Context, editor_id: Id, char_index: usize) {
    context.data_mut(|data| data.insert_temp(editor_id.with("caret-char-index"), char_index));
}

pub(crate) fn take_queued_caret_char_index(
    context: &egui::Context,
    editor_id: Id,
) -> Option<usize> {
    context.data_mut(|data| data.remove_temp::<usize>(editor_id.with("caret-char-index")))
}

/// Ask the source editor `editor_id` to replace a stretch of its text, in
/// characters, as one undoable edit the next time it is drawn, leaving the
/// caret after the new text.
pub(crate) fn queue_text_replacement(
    context: &egui::Context,
    editor_id: Id,
    range: std::ops::Range<usize>,
    text: String,
) {
    context.data_mut(|data| data.insert_temp(editor_id.with("text-replacement"), (range, text)));
}

pub(crate) fn take_queued_text_replacement(
    context: &egui::Context,
    editor_id: Id,
) -> Option<(std::ops::Range<usize>, String)> {
    context.data_mut(|data| data.remove_temp(editor_id.with("text-replacement")))
}

/// Ask the source editor `editor_id` to put its caret at a one-based line
/// and column next frame and bring it into view: left where it is when
/// already on screen, centred when it is not. Out-of-range positions clamp
/// to the document, and a folded line unfolds. Go to line, a revealed line
/// and a jump to a diagnostic all land through here.
pub(crate) fn queue_caret_position(
    context: &egui::Context,
    editor_id: Id,
    one_based_line: usize,
    one_based_column: usize,
) {
    context.data_mut(|data| {
        data.insert_temp(
            editor_id.with("caret-position"),
            (one_based_line.max(1), one_based_column.max(1)),
        );
    });
    context.request_repaint();
}

pub(crate) fn take_queued_caret_position(
    context: &egui::Context,
    editor_id: Id,
) -> Option<(usize, usize)> {
    context.data_mut(|data| data.remove_temp::<(usize, usize)>(editor_id.with("caret-position")))
}

/// Record the zero-based line of a source editor's primary caret, so Go to
/// line proposes where the reader is.
pub(crate) fn publish_caret_line(context: &egui::Context, editor_id: Id, line: usize) {
    context.data_mut(|data| data.insert_temp(editor_id.with("caret-line"), line));
}

pub fn queue_format_document(context: &egui::Context, editor_id: Id) {
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("format-document"), true);
    });
}

pub(crate) fn take_format_document_request(ui: &Ui, editor_id: Id) -> bool {
    ui.ctx()
        .data_mut(|data| data.remove_temp::<bool>(editor_id.with("format-document")))
        .unwrap_or(false)
}

/// Consume the conventional desktop bundle-search shortcut while this editor
/// owns focus.
pub(crate) fn take_find_in_source_bundle_request(
    ui: &Ui,
    editor_id: Id,
) -> Option<crate::ui::input::InputTransition> {
    if !ui.ctx().memory(|memory| memory.has_focus(editor_id)) {
        return None;
    }
    ui.ctx().input(|input| {
        crate::ui::input::InputTransition::after_shortcut(
            input,
            KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::H),
        )
    })
}

pub(crate) fn queue_reveal_line(context: &egui::Context, editor_id: Id, line: usize) {
    context.data_mut(|data| {
        data.insert_temp(editor_id.with("reveal-line"), line.max(1));
    });
    context.request_repaint();
}

pub(crate) fn take_reveal_line(ui: &Ui, editor_id: Id, source: &str) -> Option<usize> {
    let line = ui
        .ctx()
        .data_mut(|data| data.remove_temp::<usize>(editor_id.with("reveal-line")))?;
    let line_count = source.lines().count().max(1) + usize::from(source.ends_with('\n'));
    let line = line.clamp(1, line_count);
    go_to_line_start(ui.ctx(), editor_id, line);
    Some(line)
}

/// Render the shared line-navigation dialog once Go to line has been asked
/// for. Returns the accepted one-based logical line exactly once.
///
/// It is the kit's command form, placed like every other dialog: one Line
/// field, its range or its refusal under it, Enter goes and Esc cancels. It
/// was an egui window with egui's own chrome, opened at the top left over
/// the title and menu bar, with its buttons inside the body.
pub(crate) fn show_go_to_line(ui: &Ui, editor_id: Id, source: &str) -> Option<usize> {
    use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};

    let state_id = editor_id.with("go-to-line-state");
    let mut state = ui
        .ctx()
        .data_mut(|data| data.get_temp::<GoToLineState>(state_id))
        .unwrap_or_default();
    if !state.open {
        return None;
    }
    let line_count = source.lines().count().max(1) + usize::from(source.ends_with('\n'));
    if !state.seeded {
        state.input = current_one_based_line(ui.ctx(), editor_id, line_count).to_string();
        state.seeded = true;
    }
    let target = state
        .input
        .trim()
        .parse::<usize>()
        .ok()
        .filter(|line| (1..=line_count).contains(line));
    let range = format!("1 to {line_count}");
    let refusal = format!("Enter a line from 1 to {line_count}.");
    let input_id = editor_id.with("go-to-line-input");
    let choice = CommandForm::new("Go to line", "Go")
        .primary_enabled(target.is_some())
        .show(ui.ctx(), |rows| {
            rows.text("Line", input_id, &mut state.input, "");
            rows.field_line(
                Some(input_id),
                if target.is_some() {
                    FieldLine::Derived(&range)
                } else {
                    FieldLine::Error(&refusal)
                },
            );
            Some(input_id)
        });
    match (choice, target) {
        (DialogChoice::Primary, Some(line)) => {
            ui.ctx().data_mut(|data| {
                data.remove_temp::<GoToLineState>(state_id);
            });
            go_to_line_start(ui.ctx(), editor_id, line);
            Some(line)
        }
        (DialogChoice::Ghost | DialogChoice::Cancelled, _) => {
            ui.ctx().data_mut(|data| {
                data.remove_temp::<GoToLineState>(state_id);
            });
            ui.ctx()
                .memory_mut(|memory| memory.request_focus(editor_id));
            None
        }
        _ => {
            ui.ctx().data_mut(|data| data.insert_temp(state_id, state));
            None
        }
    }
}

/// Put the editor's caret at the start of a one-based line, in view, and give
/// it the keyboard.
fn go_to_line_start(context: &egui::Context, editor_id: Id, one_based_line: usize) {
    queue_caret_position(context, editor_id, one_based_line, 1);
    context.memory_mut(|memory| memory.request_focus(editor_id));
}

/// The one-based line the editor's caret is on, as it last drew it; the first
/// line before it has drawn.
fn current_one_based_line(context: &egui::Context, editor_id: Id, line_count: usize) -> usize {
    context
        .data(|data| data.get_temp::<usize>(editor_id.with("caret-line")))
        .map_or(1, |line| (line + 1).min(line_count))
}

pub(crate) fn apply_structural_command(
    source: &mut String,
    selection: ByteSelection,
    syntax: EditorSyntax,
    command: StructuralCommand,
) -> StructuralEditResult {
    let selection = clamp_selection(source, selection);
    if command == StructuralCommand::MatchBracket {
        return StructuralEditResult {
            changed: false,
            selection: match_bracket(source, selection),
        };
    }
    let lines = selected_line_range(source, selection);
    match command {
        StructuralCommand::Indent => indent(source, selection, lines, syntax.indentation()),
        StructuralCommand::Outdent => outdent(source, selection, lines, syntax.indentation()),
        StructuralCommand::ToggleComment => {
            toggle_comment(source, selection, lines, syntax.comment())
        }
        StructuralCommand::DuplicateLine => duplicate_lines(source, selection, lines),
        StructuralCommand::DeleteLine => delete_lines(source, lines),
        StructuralCommand::MoveLineUp => move_lines_up(source, selection, lines),
        StructuralCommand::MoveLineDown => move_lines_down(source, selection, lines),
        StructuralCommand::MatchBracket => unreachable!("handled above"),
    }
}

fn clamp_selection(source: &str, selection: ByteSelection) -> ByteSelection {
    ByteSelection {
        anchor: previous_char_boundary(source, selection.anchor.min(source.len())),
        cursor: previous_char_boundary(source, selection.cursor.min(source.len())),
    }
}

fn selected_line_range(source: &str, selection: ByteSelection) -> Range<usize> {
    let sorted = selection.sorted();
    let start = source[..sorted.start]
        .rfind('\n')
        .map_or(0, |newline| newline + 1);
    let effective_end = if sorted.end > sorted.start
        && sorted.end > 0
        && source.as_bytes().get(sorted.end - 1) == Some(&b'\n')
    {
        sorted.end - 1
    } else {
        sorted.end
    };
    let end = source[effective_end..]
        .find('\n')
        .map_or(source.len(), |relative| effective_end + relative + 1);
    start..end
}

fn line_starts(block: &str) -> Vec<usize> {
    let mut starts = vec![0];
    starts.extend(block.bytes().enumerate().filter_map(|(index, byte)| {
        (byte == b'\n' && index + 1 < block.len()).then_some(index + 1)
    }));
    starts
}

fn indent(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
    indentation: &str,
) -> StructuralEditResult {
    let block = &source[lines.clone()];
    let starts = line_starts(block);
    let mut replacement = String::with_capacity(block.len() + starts.len() * indentation.len());
    for (index, line) in block.split_inclusive('\n').enumerate() {
        replacement.push_str(indentation);
        replacement.push_str(line);
        if index + 1 == starts.len() && !block.ends_with('\n') {
            break;
        }
    }
    if block.is_empty() {
        replacement.push_str(indentation);
    }
    source.replace_range(lines.clone(), &replacement);
    let before_anchor = starts
        .iter()
        .filter(|start| **start <= selection.anchor - lines.start)
        .count();
    let before_cursor = starts
        .iter()
        .filter(|start| **start <= selection.cursor - lines.start)
        .count();
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: selection.anchor + before_anchor * indentation.len(),
            cursor: selection.cursor + before_cursor * indentation.len(),
        },
    }
}

fn outdent(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
    indentation: &str,
) -> StructuralEditResult {
    let block = source[lines.clone()].to_owned();
    let mut replacement = String::with_capacity(block.len());
    let mut removed_before_anchor = 0;
    let mut removed_before_cursor = 0;
    let mut offset = 0;
    let mut total_removed = 0;
    for line in block.split_inclusive('\n') {
        let removed = removable_indent(line, indentation.len());
        replacement.push_str(&line[removed..]);
        if offset < selection.anchor.saturating_sub(lines.start) {
            removed_before_anchor += removed.min(selection.anchor - lines.start - offset);
        }
        if offset < selection.cursor.saturating_sub(lines.start) {
            removed_before_cursor += removed.min(selection.cursor - lines.start - offset);
        }
        total_removed += removed;
        offset += line.len();
    }
    if total_removed == 0 {
        return StructuralEditResult {
            changed: false,
            selection,
        };
    }
    source.replace_range(lines, &replacement);
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: selection.anchor.saturating_sub(removed_before_anchor),
            cursor: selection.cursor.saturating_sub(removed_before_cursor),
        },
    }
}

fn removable_indent(line: &str, width: usize) -> usize {
    if line.starts_with('\t') {
        1
    } else {
        line.bytes()
            .take(width)
            .take_while(|byte| *byte == b' ')
            .count()
    }
}

fn toggle_comment(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
    comment: &str,
) -> StructuralEditResult {
    let block = source[lines.clone()].to_owned();
    let rows = block.split_inclusive('\n').collect::<Vec<_>>();
    let uncomment = rows
        .iter()
        .filter(|line| !line.trim().is_empty())
        .all(|line| {
            let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
            line[indent..].starts_with(comment.trim_end())
        });
    let mut replacement = String::with_capacity(block.len() + rows.len() * comment.len());
    let mut anchor = selection.anchor;
    let mut cursor = selection.cursor;
    let mut offset = lines.start;
    let mut cumulative_delta = 0_isize;
    let mut changed = false;
    for line in rows {
        if line.trim().is_empty() {
            replacement.push_str(line);
            offset += line.len();
            continue;
        }
        let indent = line.len() - line.trim_start_matches([' ', '\t']).len();
        let edit_at = (offset + indent).saturating_add_signed(cumulative_delta);
        if uncomment {
            let tail = &line[indent..];
            let marker = if tail.starts_with(comment) {
                comment.len()
            } else {
                comment.trim_end().len()
            };
            replacement.push_str(&line[..indent]);
            replacement.push_str(&tail[marker..]);
            anchor = shift_after(anchor, edit_at, -(marker as isize));
            cursor = shift_after(cursor, edit_at, -(marker as isize));
            cumulative_delta -= marker as isize;
        } else {
            replacement.push_str(&line[..indent]);
            replacement.push_str(comment);
            replacement.push_str(&line[indent..]);
            anchor = shift_after(anchor, edit_at, comment.len() as isize);
            cursor = shift_after(cursor, edit_at, comment.len() as isize);
            cumulative_delta += comment.len() as isize;
        }
        changed = true;
        offset += line.len();
    }
    if changed {
        source.replace_range(lines, &replacement);
    }
    StructuralEditResult {
        changed,
        selection: ByteSelection { anchor, cursor },
    }
}

fn duplicate_lines(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
) -> StructuralEditResult {
    let mut block = source[lines.clone()].to_owned();
    let insert_at = lines.end;
    let insertion = if block.ends_with('\n') {
        block
    } else {
        block.insert(0, '\n');
        block
    };
    source.insert_str(insert_at, &insertion);
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: selection.anchor + insertion.len(),
            cursor: selection.cursor + insertion.len(),
        },
    }
}

fn delete_lines(source: &mut String, lines: Range<usize>) -> StructuralEditResult {
    let caret = lines.start;
    source.replace_range(lines, "");
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: caret.min(source.len()),
            cursor: caret.min(source.len()),
        },
    }
}

fn move_lines_up(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
) -> StructuralEditResult {
    if lines.start == 0 {
        return StructuralEditResult {
            changed: false,
            selection,
        };
    }
    let previous_start = source[..lines.start - 1]
        .rfind('\n')
        .map_or(0, |newline| newline + 1);
    let previous = source[previous_start..lines.start].to_owned();
    let block = source[lines.clone()].to_owned();
    source.replace_range(previous_start..lines.end, &(block + &previous));
    let shift = lines.start - previous_start;
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: selection.anchor - shift,
            cursor: selection.cursor - shift,
        },
    }
}

fn move_lines_down(
    source: &mut String,
    selection: ByteSelection,
    lines: Range<usize>,
) -> StructuralEditResult {
    if lines.end >= source.len() {
        return StructuralEditResult {
            changed: false,
            selection,
        };
    }
    let next_end = source[lines.end..]
        .find('\n')
        .map_or(source.len(), |relative| lines.end + relative + 1);
    let block = source[lines.clone()].to_owned();
    let next = source[lines.end..next_end].to_owned();
    source.replace_range(lines.start..next_end, &(next.clone() + &block));
    StructuralEditResult {
        changed: true,
        selection: ByteSelection {
            anchor: selection.anchor + next.len(),
            cursor: selection.cursor + next.len(),
        },
    }
}

fn match_bracket(source: &str, selection: ByteSelection) -> ByteSelection {
    let cursor = selection.cursor;
    let candidate = source
        .get(cursor..)
        .and_then(|tail| tail.chars().next().map(|character| (cursor, character)))
        .filter(|(_, character)| is_bracket(*character))
        .or_else(|| {
            source[..cursor]
                .char_indices()
                .next_back()
                .filter(|(_, character)| is_bracket(*character))
        });
    let Some((at, bracket)) = candidate else {
        return selection;
    };
    let pair = match bracket {
        '(' => Some(('(', ')', 1_isize)),
        '[' => Some(('[', ']', 1)),
        '{' => Some(('{', '}', 1)),
        ')' => Some(('(', ')', -1)),
        ']' => Some(('[', ']', -1)),
        '}' => Some(('{', '}', -1)),
        _ => None,
    };
    let Some((open, close, direction)) = pair else {
        return selection;
    };
    let mut depth = 0_i64;
    let match_at = if direction > 0 {
        source[at..]
            .char_indices()
            .find_map(|(relative, character)| {
                if character == open {
                    depth += 1;
                } else if character == close {
                    depth -= 1;
                    if depth == 0 {
                        return Some(at + relative);
                    }
                }
                None
            })
    } else {
        source[..=at]
            .char_indices()
            .rev()
            .find_map(|(index, character)| {
                if character == close {
                    depth += 1;
                } else if character == open {
                    depth -= 1;
                    if depth == 0 {
                        return Some(index);
                    }
                }
                None
            })
    };
    match_at.map_or(selection, |matched| ByteSelection {
        anchor: matched,
        cursor: matched,
    })
}

const fn is_bracket(character: char) -> bool {
    matches!(character, '(' | ')' | '[' | ']' | '{' | '}')
}

fn shift_after(position: usize, edit_at: usize, delta: isize) -> usize {
    if position < edit_at {
        position
    } else if delta >= 0 {
        position.saturating_add(delta as usize)
    } else {
        position.saturating_sub((-delta) as usize)
    }
}

fn previous_char_boundary(source: &str, mut byte: usize) -> usize {
    while !source.is_char_boundary(byte) {
        byte -= 1;
    }
    byte
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caret(position: usize) -> ByteSelection {
        ByteSelection {
            anchor: position,
            cursor: position,
        }
    }

    #[test]
    fn indent_outdent_and_comment_are_selection_stable() {
        let mut source = "α = 1\nβ = 2\n".to_owned();
        let selection = ByteSelection {
            anchor: 0,
            cursor: source.len(),
        };
        let indented = apply_structural_command(
            &mut source,
            selection,
            EditorSyntax::Python,
            StructuralCommand::Indent,
        );
        assert_eq!(source, "    α = 1\n    β = 2\n");
        let commented = apply_structural_command(
            &mut source,
            indented.selection,
            EditorSyntax::Python,
            StructuralCommand::ToggleComment,
        );
        assert_eq!(source, "    # α = 1\n    # β = 2\n");
        let uncommented = apply_structural_command(
            &mut source,
            commented.selection,
            EditorSyntax::Python,
            StructuralCommand::ToggleComment,
        );
        assert_eq!(source, "    α = 1\n    β = 2\n");
        apply_structural_command(
            &mut source,
            uncommented.selection,
            EditorSyntax::Python,
            StructuralCommand::Outdent,
        );
        assert_eq!(source, "α = 1\nβ = 2\n");
    }

    #[test]
    fn duplicate_delete_and_move_operate_on_complete_lines() {
        let mut source = "one\ntwo\nthree\n".to_owned();
        let duplicated = apply_structural_command(
            &mut source,
            caret(5),
            EditorSyntax::Spice,
            StructuralCommand::DuplicateLine,
        );
        assert_eq!(source, "one\ntwo\ntwo\nthree\n");
        let moved = apply_structural_command(
            &mut source,
            duplicated.selection,
            EditorSyntax::Spice,
            StructuralCommand::MoveLineDown,
        );
        assert_eq!(source, "one\ntwo\nthree\ntwo\n");
        apply_structural_command(
            &mut source,
            moved.selection,
            EditorSyntax::Spice,
            StructuralCommand::DeleteLine,
        );
        assert_eq!(source, "one\ntwo\nthree\n");
    }

    #[test]
    fn duplicate_final_line_without_trailing_newline_preserves_two_lines() {
        let mut source = "one\ntwo".to_owned();
        let duplicated = apply_structural_command(
            &mut source,
            caret(5),
            EditorSyntax::Spice,
            StructuralCommand::DuplicateLine,
        );
        assert_eq!(source, "one\ntwo\ntwo");
        assert_eq!(duplicated.selection, caret(9));
    }

    #[test]
    fn match_bracket_walks_nested_pairs_in_both_directions() {
        let source = "f(a[0], {b: 2})";
        let open = apply_structural_command(
            &mut source.to_owned(),
            caret(1),
            EditorSyntax::Python,
            StructuralCommand::MatchBracket,
        );
        assert_eq!(open.selection.cursor, source.len() - 1);
        let close = apply_structural_command(
            &mut source.to_owned(),
            caret(source.len()),
            EditorSyntax::Python,
            StructuralCommand::MatchBracket,
        );
        assert_eq!(close.selection.cursor, 1);

        let immediately_after_open = apply_structural_command(
            &mut source.to_owned(),
            caret(2),
            EditorSyntax::Python,
            StructuralCommand::MatchBracket,
        );
        assert_eq!(immediately_after_open.selection.cursor, source.len() - 1);
    }

    #[test]
    fn spice_comments_begin_after_existing_indentation() {
        let mut source = "  R1 in out 1k\n".to_owned();
        apply_structural_command(
            &mut source,
            caret(4),
            EditorSyntax::Spice,
            StructuralCommand::ToggleComment,
        );
        assert_eq!(source, "  * R1 in out 1k\n");
    }

    /// Go to line is the kit's command form, centred like every other
    /// dialog, and Enter goes. It was an egui window opened at the top left,
    /// over the title and menu bar.
    #[test]
    fn go_to_line_is_a_centred_form_that_enter_accepts() {
        let context = egui::Context::default();
        crate::ui::Theme::default().apply(&context);
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 900.0));
        let input = |events: Vec<egui::Event>| egui::RawInput {
            screen_rect: Some(screen),
            events,
            ..Default::default()
        };
        let editor_id = Id::new("go-to-line-test");
        let source = "one\ntwo\nthree\nfour\n";
        queue_go_to_line(&context, editor_id);
        let mut accepted = None;
        for _ in 0..3 {
            let _ = context.run_ui(input(Vec::new()), |ui| {
                accepted = show_go_to_line(ui, editor_id, source);
            });
        }
        assert_eq!(accepted, None, "nothing is accepted before Enter");
        let field = context
            .read_response(editor_id.with("go-to-line-input"))
            .expect("the Line field is on screen")
            .rect;
        assert!(
            field.top() > 300.0 && (field.center().x - screen.center().x).abs() < 210.0,
            "the form sits in the middle of the window, not over its title bar: {field:?}"
        );

        let state_id = editor_id.with("go-to-line-state");
        context.data_mut(|data| {
            let state = data.get_temp_mut_or_default::<GoToLineState>(state_id);
            state.input = "3".to_owned();
        });
        let enter = egui::Event::Key {
            key: Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        let _ = context.run_ui(input(vec![enter]), |ui| {
            accepted = show_go_to_line(ui, editor_id, source);
        });
        assert_eq!(accepted, Some(3));
    }
}
