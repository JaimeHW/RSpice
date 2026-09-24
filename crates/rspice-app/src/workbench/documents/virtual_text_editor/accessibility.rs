//! The virtual editor's AccessKit projection.
//!
//! A rope-backed editor cannot hand a screen reader the whole document, so it
//! publishes a bounded window around the caret and keeps that window's
//! selection in sync with the model's. Every string that reaches the
//! accessibility tree is length-capped: an assistive technology reading a
//! 100k-character line is a hang, not a feature.

use super::*;

#[derive(Debug)]
struct AccessibleLineWindow {
    text: String,
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
    let selection_is_complete = anchor_line == line
        && (first_column..=last_column).contains(&column)
        && (first_column..=last_column).contains(&anchor_column);
    let primary = column
        .saturating_sub(first_column)
        .min(last_column - first_column);
    let secondary = if selection_is_complete {
        anchor_column
            .saturating_sub(first_column)
            .min(last_column - first_column)
    } else {
        primary
    };

    AccessibleLineWindow {
        text: model.text_range((line_range.start + first_column)..(line_range.start + last_column)),
        line,
        column,
        total_lines: model.line_count(),
        total_columns,
        first_column,
        primary,
        secondary,
        selected_chars,
        selection_is_complete,
        cursor_count: model.selections().len(),
    }
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
    ui.fonts_mut(|fonts| fonts.layout_no_wrap(window.text.clone(), style.font.clone(), style.text))
}

pub(super) fn consume_accesskit_text_selection(
    ui: &Ui,
    editor_id: egui::Id,
    model: &mut TextDocumentModel,
    style: &VirtualEditorStyle<'_>,
) {
    if !ui.input(|input| {
        input.has_accesskit_action_request(editor_id, egui::accesskit::Action::SetTextSelection)
    }) {
        return;
    }
    let window = accessible_line_window(model);
    let galley = accessible_line_galley(ui, &window, style);
    let mut range = accessible_cursor_range(&window);
    let events = ui.input(|input| input.events.clone());
    let changed = events
        .iter()
        .any(|event| range.on_event(ui.os(), event, &galley, editor_id));
    if changed {
        let line_start = model
            .line_content_char_range(window.line)
            .map_or(model.len_chars(), |range| range.start);
        let accessible_chars = window.text.chars().count();
        let primary =
            line_start + window.first_column + range.primary.index.0.min(accessible_chars);
        let secondary =
            line_start + window.first_column + range.secondary.index.0.min(accessible_chars);
        model.set_selections(vec![TextSelection {
            anchor: secondary,
            cursor: primary,
        }]);
    }
}

pub(super) fn update_accesskit_text_state(
    ui: &Ui,
    editor_id: egui::Id,
    response: &Response,
    model: &TextDocumentModel,
    editable: bool,
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
    let visible_row = model
        .projected_visible_row_for_logical_line(window.line)
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
}

fn append_accessible_sentence(description: &mut String, sentence: &str) {
    if !description.is_empty() && !sentence.is_empty() {
        description.push(' ');
    }
    description.push_str(sentence);
}

pub(super) fn bounded_accessible_text(source: &str, maximum_chars: usize) -> String {
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
