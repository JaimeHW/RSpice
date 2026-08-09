//! Deterministic structural editing shared by every source-language page.
//!
//! EGUI owns ordinary IME, clipboard, selection, and undo input. Commands
//! which operate on complete source lines live here so Netlist, Verilog-A,
//! Python, YAML, and TOML cannot drift into subtly different behavior.

use std::ops::Range;

use egui::text::{CCursor, CCursorRange};
use egui::{Id, Key, KeyboardShortcut, Modifiers, Ui};

#[derive(Debug, Clone, Default)]
struct GoToLineState {
    open: bool,
    input: String,
    error: Option<String>,
    focus_input: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum StandardCommand {
    #[default]
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    SelectAll,
}

impl StandardCommand {
    const fn label(self) -> &'static str {
        match self {
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Paste => "Paste",
            Self::SelectAll => "Select all",
        }
    }

    const fn shortcut(self) -> &'static str {
        match self {
            Self::Undo => "Ctrl+Z",
            Self::Redo => "Ctrl+Shift+Z",
            Self::Cut => "Ctrl+X",
            Self::Copy => "Ctrl+C",
            Self::Paste => "Ctrl+V",
            Self::SelectAll => "Ctrl+A",
        }
    }

    pub(crate) const fn mutates_source(self) -> bool {
        matches!(self, Self::Undo | Self::Redo | Self::Cut | Self::Paste)
    }
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum StructuralCommand {
    #[default]
    Indent,
    Outdent,
    ToggleComment,
    DuplicateLine,
    DeleteLine,
    MoveLineUp,
    MoveLineDown,
    MatchBracket,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum EditorViewCommand {
    #[default]
    FoldSelection,
    UnfoldAtCaret,
    FoldAll,
    UnfoldAll,
}

impl EditorViewCommand {
    const fn label(self) -> &'static str {
        match self {
            Self::FoldSelection => "Fold selection",
            Self::UnfoldAtCaret => "Unfold at caret",
            Self::FoldAll => "Fold all regions",
            Self::UnfoldAll => "Unfold all regions",
        }
    }

    const fn shortcut(self) -> &'static str {
        match self {
            Self::FoldSelection => "Ctrl+Shift+[",
            Self::UnfoldAtCaret => "Ctrl+Shift+]",
            Self::FoldAll | Self::UnfoldAll => "",
        }
    }
}

impl StructuralCommand {
    const fn label(self) -> &'static str {
        match self {
            Self::Indent => "Indent lines",
            Self::Outdent => "Outdent lines",
            Self::ToggleComment => "Toggle line comment",
            Self::DuplicateLine => "Duplicate line",
            Self::DeleteLine => "Delete line",
            Self::MoveLineUp => "Move line up",
            Self::MoveLineDown => "Move line down",
            Self::MatchBracket => "Go to matching bracket",
        }
    }

    const fn shortcut(self) -> &'static str {
        match self {
            Self::Indent => "Tab",
            Self::Outdent => "Shift+Tab",
            Self::ToggleComment => "Ctrl+/",
            Self::DuplicateLine => "Ctrl+D",
            Self::DeleteLine => "Ctrl+Shift+K",
            Self::MoveLineUp => "Alt+Up",
            Self::MoveLineDown => "Alt+Down",
            Self::MatchBracket => "Ctrl+Shift+\\",
        }
    }

    pub(crate) const fn mutates_source(self) -> bool {
        !matches!(self, Self::MatchBracket)
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

/// Consume one structural editor shortcut when `editor_id` owns focus.
/// Returns true only when source bytes changed.
pub(crate) fn consume_structural_shortcut(
    ui: &Ui,
    editor_id: Id,
    source: &mut String,
    syntax: EditorSyntax,
    editable: bool,
) -> bool {
    let queued = take_queued_structural_command(ui.ctx(), editor_id);
    if queued.is_none() && !ui.ctx().memory(|memory| memory.has_focus(editor_id)) {
        return false;
    }
    let command = queued.or_else(|| {
        ui.ctx().input_mut(|input| {
            let command_shift = Modifiers::COMMAND | Modifiers::SHIFT;
            if editable && input.consume_key(Modifiers::NONE, Key::Tab) {
                Some(StructuralCommand::Indent)
            } else if editable && input.consume_key(Modifiers::SHIFT, Key::Tab) {
                Some(StructuralCommand::Outdent)
            } else if editable
                && input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::Slash))
            {
                Some(StructuralCommand::ToggleComment)
            } else if editable
                && input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::D))
            {
                Some(StructuralCommand::DuplicateLine)
            } else if editable
                && input.consume_shortcut(&KeyboardShortcut::new(command_shift, Key::K))
            {
                Some(StructuralCommand::DeleteLine)
            } else if editable && input.consume_key(Modifiers::ALT, Key::ArrowUp) {
                Some(StructuralCommand::MoveLineUp)
            } else if editable && input.consume_key(Modifiers::ALT, Key::ArrowDown) {
                Some(StructuralCommand::MoveLineDown)
            } else if input.consume_shortcut(&KeyboardShortcut::new(command_shift, Key::Backslash))
            {
                Some(StructuralCommand::MatchBracket)
            } else {
                None
            }
        })
    });
    let Some(command) = command else {
        return false;
    };
    let mut state = egui::text_edit::TextEditState::load(ui.ctx(), editor_id).unwrap_or_default();
    let char_range = state
        .cursor
        .char_range()
        .unwrap_or_else(|| CCursorRange::one(CCursor::new(source.chars().count())));
    let selection = ByteSelection {
        anchor: char_to_byte(source, char_range.secondary.index.0),
        cursor: char_to_byte(source, char_range.primary.index.0),
    };
    if command.mutates_source() {
        let mut undoer = state.undoer();
        undoer.add_undo(&(char_range, source.clone()));
        state.set_undoer(undoer);
    }
    let result = apply_structural_command(source, selection, syntax, command);
    let anchor = byte_to_char(source, result.selection.anchor);
    let cursor = byte_to_char(source, result.selection.cursor);
    state.cursor.set_char_range(Some(CCursorRange {
        primary: CCursor::new(cursor),
        secondary: CCursor::new(anchor),
        h_pos: None,
    }));
    state.store(ui.ctx(), editor_id);
    ui.ctx()
        .memory_mut(|memory| memory.request_focus(editor_id));
    result.changed
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

/// Consume a standard command queued by the shared Editor menu. Keyboard
/// shortcuts remain owned by TextEdit itself.
pub(crate) fn consume_standard_command(
    ui: &Ui,
    editor_id: Id,
    source: &mut String,
    editable: bool,
) -> bool {
    let command = take_queued_standard_command(ui.ctx(), editor_id);
    let Some(command) = command else {
        return false;
    };
    if command.mutates_source() && !editable {
        return false;
    }

    let mut state = egui::text_edit::TextEditState::load(ui.ctx(), editor_id).unwrap_or_default();
    let char_range = state
        .cursor
        .char_range()
        .unwrap_or_else(|| CCursorRange::one(CCursor::new(source.chars().count())));
    let mut changed = false;
    match command {
        StandardCommand::Undo | StandardCommand::Redo => {
            let current = (char_range, source.clone());
            let mut undoer = state.undoer();
            let restored = if command == StandardCommand::Undo {
                undoer.undo(&current).cloned()
            } else {
                undoer.redo(&current).cloned()
            };
            state.set_undoer(undoer);
            if let Some((selection, restored_source)) = restored {
                *source = restored_source;
                state.cursor.set_char_range(Some(selection));
                changed = true;
            }
        }
        StandardCommand::Copy | StandardCommand::Cut => {
            if !char_range.is_empty() {
                ui.copy_text(char_range.slice_str(source).to_owned());
                if command == StandardCommand::Cut {
                    let sorted = char_range.as_sorted_char_range();
                    let start = char_to_byte(source, sorted.start.0);
                    let end = char_to_byte(source, sorted.end.0);
                    let mut undoer = state.undoer();
                    undoer.add_undo(&(char_range, source.clone()));
                    state.set_undoer(undoer);
                    source.replace_range(start..end, "");
                    state
                        .cursor
                        .set_char_range(Some(CCursorRange::one(CCursor::new(sorted.start.0))));
                    changed = true;
                }
            }
        }
        StandardCommand::Paste => {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::RequestPaste);
        }
        StandardCommand::SelectAll => {
            state.cursor.set_char_range(Some(CCursorRange::two(
                CCursor::new(0),
                CCursor::new(source.chars().count()),
            )));
        }
    }
    state.store(ui.ctx(), editor_id);
    ui.ctx()
        .memory_mut(|memory| memory.request_focus(editor_id));
    changed
}

pub(crate) fn queue_go_to_line(context: &egui::Context, editor_id: Id) {
    let state_id = editor_id.with("go-to-line-state");
    let mut state = context
        .data_mut(|data| data.get_temp::<GoToLineState>(state_id))
        .unwrap_or_default();
    state.open = true;
    state.focus_input = true;
    state.error = None;
    context.data_mut(|data| data.insert_temp(state_id, state));
}

/// A single command menu shared by SPICE, Verilog-A, Python, YAML, and TOML.
pub(crate) fn editor_command_menu(
    ui: &mut Ui,
    editor_id: Id,
    editable: bool,
    source_bundle_search: bool,
) {
    ui.menu_button("Editor", |ui| {
        for command in [
            StandardCommand::Undo,
            StandardCommand::Redo,
            StandardCommand::Cut,
            StandardCommand::Copy,
            StandardCommand::Paste,
            StandardCommand::SelectAll,
        ] {
            let enabled = editable || !command.mutates_source();
            if ui
                .add_enabled(
                    enabled,
                    egui::Button::new(command.label()).shortcut_text(command.shortcut()),
                )
                .clicked()
            {
                queue_standard_command(ui.ctx(), editor_id, command);
                ui.close();
            }
        }
        ui.separator();
        if ui
            .add_enabled(editable, egui::Button::new("Format document"))
            .clicked()
        {
            ui.ctx().data_mut(|data| {
                data.insert_temp(editor_id.with("format-document"), true);
            });
            ui.close();
        }
        for command in [
            EditorViewCommand::FoldSelection,
            EditorViewCommand::UnfoldAtCaret,
            EditorViewCommand::FoldAll,
            EditorViewCommand::UnfoldAll,
        ] {
            let mut button = egui::Button::new(command.label());
            if !command.shortcut().is_empty() {
                button = button.shortcut_text(command.shortcut());
            }
            if ui.add(button).clicked() {
                queue_view_command(ui.ctx(), editor_id, command);
                ui.close();
            }
        }
        ui.separator();
        for command in [
            StructuralCommand::Indent,
            StructuralCommand::Outdent,
            StructuralCommand::ToggleComment,
            StructuralCommand::DuplicateLine,
            StructuralCommand::DeleteLine,
            StructuralCommand::MoveLineUp,
            StructuralCommand::MoveLineDown,
            StructuralCommand::MatchBracket,
        ] {
            let enabled = editable || !command.mutates_source();
            let response = ui.add_enabled(
                enabled,
                egui::Button::new(command.label()).shortcut_text(command.shortcut()),
            );
            if response.clicked() {
                queue_structural_command(ui.ctx(), editor_id, command);
                ui.close();
            }
        }
        ui.separator();
        if ui
            .add(egui::Button::new("Go to line...").shortcut_text("Ctrl+G"))
            .clicked()
        {
            queue_go_to_line(ui.ctx(), editor_id);
            ui.close();
        }
        if source_bundle_search
            && ui
                .add(egui::Button::new("Find in source bundle...").shortcut_text("Ctrl+Shift+H"))
                .clicked()
        {
            ui.ctx().data_mut(|data| {
                data.insert_temp(editor_id.with("find-in-source-bundle"), true);
            });
            ui.close();
        }
        if source_bundle_search
            && ui
                .button("Source navigation and language tools...")
                .clicked()
        {
            ui.ctx().data_mut(|data| {
                data.insert_temp(editor_id.with("source-language-tools"), true);
            });
            ui.close();
        }
    });
}

pub(crate) fn take_format_document_request(ui: &Ui, editor_id: Id) -> bool {
    ui.ctx()
        .data_mut(|data| data.remove_temp::<bool>(editor_id.with("format-document")))
        .unwrap_or(false)
}

pub(crate) fn take_source_language_tools_request(ui: &Ui, editor_id: Id) -> bool {
    ui.ctx()
        .data_mut(|data| data.remove_temp::<bool>(editor_id.with("source-language-tools")))
        .unwrap_or(false)
}

/// Consume a bundle-search request from the shared editor command surface or
/// the conventional desktop shortcut while this editor owns focus.
pub(crate) fn take_find_in_source_bundle_request(ui: &Ui, editor_id: Id) -> bool {
    let queued = ui.ctx().data_mut(|data| {
        data.remove_temp::<bool>(editor_id.with("find-in-source-bundle"))
            .unwrap_or(false)
    });
    let shortcut = ui.ctx().memory(|memory| memory.has_focus(editor_id))
        && ui.ctx().input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(
                Modifiers::COMMAND | Modifiers::SHIFT,
                Key::H,
            ))
        });
    queued || shortcut
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
    place_caret_at_line(ui.ctx(), editor_id, source, line);
    Some(line)
}

/// Process Ctrl/Cmd+G and render the shared line-navigation dialog. Returns
/// the accepted one-based logical line exactly once.
pub(crate) fn show_go_to_line(ui: &Ui, editor_id: Id, source: &str) -> Option<usize> {
    if ui.ctx().memory(|memory| memory.has_focus(editor_id))
        && ui.ctx().input_mut(|input| {
            input.consume_shortcut(&KeyboardShortcut::new(Modifiers::COMMAND, Key::G))
        })
    {
        queue_go_to_line(ui.ctx(), editor_id);
    }

    let state_id = editor_id.with("go-to-line-state");
    let mut state = ui
        .ctx()
        .data_mut(|data| data.get_temp::<GoToLineState>(state_id))
        .unwrap_or_default();
    if !state.open {
        return None;
    }
    let line_count = source.lines().count().max(1) + usize::from(source.ends_with('\n'));
    if state.input.is_empty() {
        state.input = current_one_based_line(ui.ctx(), editor_id, source).to_string();
    }
    let input_id = editor_id.with("go-to-line-input");
    let mut accepted = false;
    let mut cancelled = false;
    let mut window_open = true;
    egui::Window::new("Go to line")
        .id(editor_id.with("go-to-line-window"))
        .collapsible(false)
        .resizable(false)
        .default_width(300.0)
        .open(&mut window_open)
        .show(ui.ctx(), |dialog| {
            dialog.label(format!("Line number (1 to {line_count})"));
            let response = dialog.add(
                egui::TextEdit::singleline(&mut state.input)
                    .id(input_id)
                    .desired_width(f32::INFINITY)
                    .char_limit(20),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::TextEdit,
                    true,
                    format!("Line number, 1 through {line_count}"),
                )
            });
            if state.focus_input {
                response.request_focus();
                state.focus_input = false;
            }
            if let Some(error) = state.error.as_deref() {
                dialog.colored_label(dialog.visuals().error_fg_color, error);
            }
            dialog.add_space(6.0);
            dialog.horizontal(|dialog| {
                if dialog.button("Cancel").clicked() {
                    cancelled = true;
                }
                if dialog.button("Go to line").clicked() {
                    accepted = true;
                }
            });
            if dialog.input(|input| input.key_pressed(Key::Enter)) {
                accepted = true;
            }
            if dialog.input(|input| input.key_pressed(Key::Escape)) {
                cancelled = true;
            }
        });
    if !window_open {
        cancelled = true;
    }

    if cancelled {
        ui.ctx().data_mut(|data| {
            data.remove_temp::<GoToLineState>(state_id);
        });
        ui.ctx()
            .memory_mut(|memory| memory.request_focus(editor_id));
        return None;
    }
    if accepted {
        match state.input.trim().parse::<usize>() {
            Ok(line) if (1..=line_count).contains(&line) => {
                ui.ctx().data_mut(|data| {
                    data.remove_temp::<GoToLineState>(state_id);
                });
                place_caret_at_line(ui.ctx(), editor_id, source, line);
                return Some(line);
            }
            _ => {
                state.error = Some(format!("Enter a line from 1 through {line_count}."));
                state.focus_input = true;
            }
        }
    }
    ui.ctx().data_mut(|data| data.insert_temp(state_id, state));
    None
}

pub(crate) fn place_caret_at_line(
    context: &egui::Context,
    editor_id: Id,
    source: &str,
    one_based_line: usize,
) {
    let char_index = char_index_for_line(source, one_based_line);
    let mut state = egui::text_edit::TextEditState::load(context, editor_id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(CCursorRange::one(CCursor::new(char_index))));
    state.store(context, editor_id);
    context.memory_mut(|memory| memory.request_focus(editor_id));
}

fn current_one_based_line(context: &egui::Context, editor_id: Id, source: &str) -> usize {
    let index = egui::text_edit::TextEditState::load(context, editor_id)
        .and_then(|state| state.cursor.char_range())
        .map_or(0, |range| range.primary.index.0);
    source
        .chars()
        .take(index)
        .filter(|character| *character == '\n')
        .count()
        + 1
}

fn char_index_for_line(source: &str, one_based_line: usize) -> usize {
    let target = one_based_line.max(1);
    let mut line = 1;
    for (index, character) in source.chars().enumerate() {
        if line == target {
            return index;
        }
        if character == '\n' {
            line += 1;
        }
    }
    source.chars().count()
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

fn char_to_byte(source: &str, index: usize) -> usize {
    source
        .char_indices()
        .nth(index)
        .map_or(source.len(), |(byte, _)| byte)
}

fn byte_to_char(source: &str, byte: usize) -> usize {
    source[..previous_char_boundary(source, byte.min(source.len()))]
        .chars()
        .count()
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

    #[test]
    fn go_to_line_character_offsets_are_unicode_safe_and_clamped() {
        let source = "α first\nβ second\nγ third";
        assert_eq!(char_index_for_line(source, 1), 0);
        assert_eq!(char_index_for_line(source, 2), 8);
        assert_eq!(char_index_for_line(source, 3), 17);
        assert_eq!(char_index_for_line(source, 99), source.chars().count());
    }

    #[test]
    fn menu_cut_and_undo_share_the_text_edit_history() {
        let context = egui::Context::default();
        let editor_id = egui::Id::new("standard-command-test");
        let mut source = "alpha beta".to_owned();
        let _ = context.run_ui(Default::default(), |context| {
            egui::CentralPanel::default().show(context, |ui| {
                let mut state = egui::text_edit::TextEditState::default();
                state
                    .cursor
                    .set_char_range(Some(CCursorRange::two(CCursor::new(0), CCursor::new(5))));
                state.store(ui.ctx(), editor_id);

                queue_standard_command(ui.ctx(), editor_id, StandardCommand::Cut);
                assert!(consume_standard_command(ui, editor_id, &mut source, true));
                assert_eq!(source, " beta");

                queue_standard_command(ui.ctx(), editor_id, StandardCommand::Undo);
                assert!(consume_standard_command(ui, editor_id, &mut source, true));
                assert_eq!(source, "alpha beta");
            });
        });
    }
}
