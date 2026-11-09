//! The virtual editor's AccessKit projection.
//!
//! A rope-backed editor cannot hand a screen reader the whole document, so it
//! publishes a bounded window around the caret, the caret's line and three
//! lines either side, and keeps that window's selection in sync with the
//! model's. Every string that reaches the accessibility tree is length-capped:
//! an assistive technology reading a 100k-character line is a hang, not a
//! feature.

use super::*;

/// The lines either side of the caret's that a screen reader is given.
const ACCESSIBLE_CONTEXT_LINES: usize = 3;

/// One document line's part of the window's text.
#[derive(Debug, Clone, Copy)]
struct AccessibleWindowLine {
    line: usize,
    first_column: usize,
    chars: usize,
    /// Where it starts in the window's text.
    offset: usize,
}

#[derive(Debug)]
struct AccessibleLineWindow {
    text: String,
    lines: Vec<AccessibleWindowLine>,
    line: usize,
    column: usize,
    total_lines: usize,
    total_columns: usize,
    first_column: usize,
    primary: usize,
    secondary: usize,
    selected_chars: usize,
    selection_is_complete: bool,
    cursor_count: usize,
}

impl AccessibleLineWindow {
    /// Where the document's `line` and `column` stand in the window's text,
    /// when the window shows them.
    fn offset_of(&self, line: usize, column: usize) -> Option<usize> {
        let entry = self.lines.iter().find(|entry| entry.line == line)?;
        (entry.first_column..=entry.first_column + entry.chars)
            .contains(&column)
            .then(|| entry.offset + column - entry.first_column)
    }

    /// The document line and column at `offset` in the window's text.
    fn place_of(&self, offset: usize) -> Option<(usize, usize)> {
        self.lines
            .iter()
            .find(|entry| (entry.offset..=entry.offset + entry.chars).contains(&offset))
            .map(|entry| (entry.line, entry.first_column + offset - entry.offset))
    }
}

fn accessible_line_window(model: &TextDocumentModel) -> AccessibleLineWindow {
    let selection = model
        .selections()
        .last()
        .copied()
        .unwrap_or_else(|| TextSelection::caret(0));
    let (line, column) = model.line_column_for_char(selection.cursor);
    let (anchor_line, anchor_column) = model.line_column_for_char(selection.anchor);
    let line_range = model
        .line_content_char_range(line)
        .unwrap_or_else(|| model.len_chars()..model.len_chars());
    let total_columns = line_range.len();
    let column = column.min(total_columns);
    let anchor_column = anchor_column.min(total_columns);
    let selected_chars = selection.range().len();

    let mut first_column = if anchor_line == line
        && column.abs_diff(anchor_column) <= MAX_ACCESSIBLE_LINE_CHARS
    {
        let selected_start = column.min(anchor_column);
        let selected_end = column.max(anchor_column);
        let surrounding = MAX_ACCESSIBLE_LINE_CHARS.saturating_sub(selected_end - selected_start);
        selected_start.saturating_sub(surrounding / 2)
    } else {
        column.saturating_sub(MAX_ACCESSIBLE_LINE_CHARS / 2)
    };
    first_column = first_column.min(total_columns.saturating_sub(MAX_ACCESSIBLE_LINE_CHARS));
    let last_column = (first_column + MAX_ACCESSIBLE_LINE_CHARS).min(total_columns);

    let total_lines = model.line_count();
    let mut text = String::new();
    let mut lines = Vec::new();
    let mut offset = 0;
    let window_end = (line + ACCESSIBLE_CONTEXT_LINES).min(total_lines.saturating_sub(1));
    for index in line.saturating_sub(ACCESSIBLE_CONTEXT_LINES)..=window_end {
        let range = if index == line {
            line_range.clone()
        } else {
            model
                .line_content_char_range(index)
                .unwrap_or_else(|| model.len_chars()..model.len_chars())
        };
        let (first, last) = if index == line {
            (first_column, last_column)
        } else {
            (0, range.len().min(MAX_ACCESSIBLE_LINE_CHARS))
        };
        if !lines.is_empty() {
            text.push('\n');
            offset += 1;
        }
        text.push_str(&model.text_range((range.start + first)..(range.start + last)));
        lines.push(AccessibleWindowLine {
            line: index,
            first_column: first,
            chars: last - first,
            offset,
        });
        offset += last - first;
    }

    let mut window = AccessibleLineWindow {
        text,
        lines,
        line,
        column,
        total_lines,
        total_columns,
        first_column,
        primary: 0,
        secondary: 0,
        selected_chars,
        selection_is_complete: false,
        cursor_count: model.selections().len(),
    };
    window.primary = window.offset_of(line, column).unwrap_or(0);
    let anchor = window.offset_of(anchor_line, anchor_column);
    window.selection_is_complete = anchor.is_some();
    window.secondary = anchor.unwrap_or(window.primary);
    window
}

fn accessible_cursor_range(window: &AccessibleLineWindow) -> egui::text::CCursorRange {
    egui::text::CCursorRange {
        primary: egui::text::CCursor::new(window.primary),
        secondary: egui::text::CCursor::new(window.secondary),
        h_pos: None,
    }
}

fn accessible_line_galley(
    ui: &Ui,
    window: &AccessibleLineWindow,
    style: &VirtualEditorStyle<'_>,
) -> Arc<egui::Galley> {
    // At the editor's own line pitch, so each line's bounds are where it is
    // drawn.
    let mut job = egui::text::LayoutJob::simple(
        window.text.clone(),
        style.font.clone(),
        style.text,
        f32::INFINITY,
    );
    if let Some(section) = job.sections.first_mut() {
        section.format.line_height = Some(style.line_height);
    }
    ui.fonts_mut(|fonts| fonts.layout_job(job))
}

pub(super) fn consume_accesskit_text_selection(
    ui: &Ui,
    editor_id: egui::Id,
    model: &mut TextDocumentModel,
    style: &VirtualEditorStyle<'_>,
) -> bool {
    if !ui.input(|input| {
        input.has_accesskit_action_request(editor_id, egui::accesskit::Action::SetTextSelection)
    }) {
        return false;
    }
    let window = accessible_line_window(model);
    let galley = accessible_line_galley(ui, &window, style);
    let mut range = accessible_cursor_range(&window);
    let events = ui.input(|input| input.events.clone());
    let changed = events
        .iter()
        .any(|event| range.on_event(ui.os(), event, &galley, editor_id));
    if changed {
        let accessible_chars = window.text.chars().count();
        let document_char = |index: usize| {
            window
                .place_of(index.min(accessible_chars))
                .and_then(|(line, column)| {
                    model
                        .line_content_char_range(line)
                        .map(|range| range.start + column)
                })
                .unwrap_or_else(|| model.len_chars())
        };
        let primary = document_char(range.primary.index.0);
        let secondary = document_char(range.secondary.index.0);
        model.set_selections(vec![TextSelection {
            anchor: secondary,
            cursor: primary,
        }]);
    }
    changed
}

pub(super) fn update_accesskit_text_state(
    ui: &Ui,
    editor_id: egui::Id,
    response: &Response,
    model: &TextDocumentModel,
    editable: bool,
    tab_moves_focus: bool,
    style: &VirtualEditorStyle<'_>,
    line_diagnostic_context: &mut impl FnMut(usize) -> Option<String>,
) {
    // This inexpensive probe avoids the bounded galley allocation altogether
    // when no native or web accessibility adapter is active.
    if ui.ctx().accesskit_node_builder(editor_id, |_| ()).is_none() {
        return;
    }

    let window = accessible_line_window(model);
    let galley = accessible_line_galley(ui, &window, style);
    let first_line = window.lines.first().map_or(window.line, |entry| entry.line);
    let visible_row = model
        .projected_visible_row_for_logical_line(first_line)
        .or_else(|| {
            model
                .projected_visible_row_for_logical_line(window.line)
                .map(|row| row.saturating_sub(window.line - first_line))
        })
        .unwrap_or(0);
    let char_width = monospace_advance(ui, &style.font).max(1.0);
    egui::text_selection::accesskit_text::update_accesskit_for_text_widget(
        ui.ctx(),
        editor_id,
        Some(accessible_cursor_range(&window)),
        egui::accesskit::Role::MultilineTextInput,
        egui::emath::TSTransform::from_translation(
            (response.rect.min
                + egui::vec2(
                    style.gutter_width
                        + style.code_left_padding
                        + window.first_column as f32 * char_width,
                    style.top_padding + visible_row as f32 * style.line_height,
                ))
            .to_vec2(),
        ),
        &galley,
    );

    let line = (window.line + 1).to_string();
    let total_lines = window.total_lines.to_string();
    let column = (window.column + 1).to_string();
    let mut description = style.messages.format(
        if editable {
            crate::workbench::MessageId::EditorAccessiblePositionEditable
        } else {
            crate::workbench::MessageId::EditorAccessiblePositionReadOnly
        },
        &[
            ("line", &line),
            ("total_lines", &total_lines),
            ("column", &column),
        ],
    );
    if window.selected_chars > 0 {
        let selected_chars = window.selected_chars.to_string();
        append_accessible_sentence(
            &mut description,
            &if window.selected_chars == 1 {
                style
                    .messages
                    .text(crate::workbench::MessageId::EditorAccessibleOneCharacterSelected)
            } else {
                style.messages.format(
                    crate::workbench::MessageId::EditorAccessibleCharactersSelected,
                    &[("count", &selected_chars)],
                )
            },
        );
        if !window.selection_is_complete {
            append_accessible_sentence(
                &mut description,
                &style
                    .messages
                    .text(crate::workbench::MessageId::EditorAccessibleSelectionBeyondExcerpt),
            );
        }
    }
    if window.cursor_count > 1 {
        let cursor_count = window.cursor_count.to_string();
        append_accessible_sentence(
            &mut description,
            &style.messages.format(
                crate::workbench::MessageId::EditorAccessibleMultipleCursors,
                &[("count", &cursor_count)],
            ),
        );
    }
    if editable && tab_moves_focus {
        append_accessible_sentence(
            &mut description,
            &tab_mode_sentence(style, ui.ctx().os(), true),
        );
    }
    if style.breakpoints_enabled {
        append_accessible_sentence(
            &mut description,
            &style
                .messages
                .text(crate::workbench::MessageId::EditorAccessibleBreakpointToggleHint),
        );
    }
    let one_based_line = window.line + 1;
    for message in [
        style
            .breakpoints
            .contains(&one_based_line)
            .then_some(crate::workbench::MessageId::EditorAccessibleBreakpoint),
        (style.current_line == Some(one_based_line))
            .then_some(crate::workbench::MessageId::EditorAccessibleCurrentStatement),
        (style.cross_probe_line == Some(window.line))
            .then_some(crate::workbench::MessageId::EditorAccessibleCrossProbe),
        style
            .edited_lines
            .contains(&window.line)
            .then_some(crate::workbench::MessageId::EditorAccessibleModifiedLine),
    ]
    .into_iter()
    .flatten()
    {
        append_accessible_sentence(&mut description, &style.messages.text(message));
    }
    if window.total_columns > MAX_ACCESSIBLE_LINE_CHARS {
        let first_column = (window.first_column + 1).to_string();
        let last_column = (window.first_column + window.text.chars().count()).to_string();
        let total_columns = window.total_columns.to_string();
        append_accessible_sentence(
            &mut description,
            &style.messages.format(
                crate::workbench::MessageId::EditorAccessibleSourceExcerpt,
                &[
                    ("first_column", &first_column),
                    ("last_column", &last_column),
                    ("total_columns", &total_columns),
                ],
            ),
        );
    }
    if let Some(context) = line_diagnostic_context(window.line) {
        let context = bounded_accessible_text(&context, MAX_ACCESSIBLE_DIAGNOSTIC_CHARS);
        if !context.is_empty() {
            append_accessible_sentence(&mut description, &context);
        }
    }
    ui.ctx().accesskit_node_builder(editor_id, |node| {
        node.set_label(style.accessible_label);
        node.set_description(description);
        if !editable {
            node.set_read_only();
        }
    });
    speak_caret_line(ui, editor_id, response, &window, style);
}

const SPEAK_CARET_LINES_ID: &str = "source-editor-speak-caret-lines";

/// Have every source editor say the line the caret arrives on. For a reader
/// with no text control to follow the caret through: the browser's mirror. A
/// desktop screen reader follows the editor's text itself and would hear it
/// twice.
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) fn speak_caret_lines(ctx: &egui::Context) {
    ctx.data_mut(|data| data.insert_temp(egui::Id::new(SPEAK_CARET_LINES_ID), true));
}

/// The line last said, its words as the caret found them, and a count that
/// tells one arrival from the next when two lines read the same.
#[derive(Debug, Clone)]
struct SpokenCaretLine {
    line: usize,
    arrival: u64,
    words: String,
}

/// A live status holding the caret's line, renewed each time the caret
/// reaches another line. Typing on a line leaves it alone: the words are what
/// the caret found there, so the region does not chatter.
fn speak_caret_line(
    ui: &Ui,
    editor_id: egui::Id,
    response: &Response,
    window: &AccessibleLineWindow,
    style: &VirtualEditorStyle<'_>,
) {
    if !response.has_focus()
        || !ui.ctx().data(|data| {
            data.get_temp::<bool>(egui::Id::new(SPEAK_CARET_LINES_ID))
                .unwrap_or(false)
        })
    {
        return;
    }
    let key = editor_id.with("spoken-caret-line");
    let last = ui.ctx().data(|data| data.get_temp::<SpokenCaretLine>(key));
    let spoken = match last {
        Some(last) if last.line == window.line => last,
        last => SpokenCaretLine {
            line: window.line,
            arrival: last.map_or(0, |last| last.arrival + 1),
            words: caret_line_words(window, style),
        },
    };
    ui.ctx()
        .data_mut(|data| data.insert_temp(key, spoken.clone()));
    let status = ui.interact(
        egui::Rect::from_min_size(response.rect.min, Vec2::splat(1.0)),
        key.with(spoken.arrival),
        Sense::hover(),
    );
    ui.ctx().accesskit_node_builder(status.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_live(egui::accesskit::Live::Polite);
        node.set_label(spoken.words);
    });
}

fn caret_line_words(window: &AccessibleLineWindow, style: &VirtualEditorStyle<'_>) -> String {
    let line = window
        .lines
        .iter()
        .find(|entry| entry.line == window.line)
        .map(|entry| {
            window
                .text
                .chars()
                .skip(entry.offset)
                .take(entry.chars)
                .collect::<String>()
        })
        .unwrap_or_default();
    let words = bounded_accessible_text(&line, MAX_ACCESSIBLE_LINE_CHARS);
    if words.is_empty() {
        style
            .messages
            .text(crate::workbench::MessageId::EditorAccessibleBlankLine)
    } else {
        words
    }
}

fn append_accessible_sentence(description: &mut String, sentence: &str) {
    if !description.is_empty() && !sentence.is_empty() {
        description.push(' ');
    }
    description.push_str(sentence);
}

pub(crate) fn bounded_accessible_text(source: &str, maximum_chars: usize) -> String {
    let mut output = String::with_capacity(source.len().min(maximum_chars));
    let mut output_chars = 0usize;
    let mut pending_space = false;
    let mut truncated = false;
    for character in source.chars() {
        if character.is_whitespace() {
            pending_space = !output.is_empty();
            continue;
        }
        if pending_space {
            if output_chars >= maximum_chars {
                truncated = true;
                break;
            }
            output.push(' ');
            output_chars += 1;
            pending_space = false;
        }
        if output_chars >= maximum_chars {
            truncated = true;
            break;
        }
        output.push(character);
        output_chars += 1;
    }
    if truncated && maximum_chars > 0 {
        while output_chars > maximum_chars.saturating_sub(1) {
            if output.pop().is_some() {
                output_chars -= 1;
            } else {
                break;
            }
        }
        output.push('…');
    }
    output
}
