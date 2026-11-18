//! The find bar over a source editor. One row holds the search, the match
//! case, whole word and regular expression toggles, where the selected match
//! stands among them ("3 of 17"), Previous, Next and Close; where the text
//! may change, a second row holds the replacement, Replace and Replace all.
//! Every match is marked in the text and on the scroll bar.

use std::ops::Range;

use egui::{Align, Id, Key, Layout, Sense, Stroke, Ui, UiBuilder, Vec2};

use super::super::text_document_model::{TextDocumentModel, TextSelection};
use super::{VirtualEditorSession, VirtualEditorStyle};
use crate::state::{FindError, FindOptions, find_all_in_source_bounded, replace_source_ranges};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Button, compact_height};
use crate::workbench::MessageId;
use crate::workbench::design_system::{WorkbenchIcon, icon_button};

/// Air around the bar's controls, above and below and at its ends.
const BAR_PADDING: f32 = 3.0;
/// Room between the bar's controls.
const BAR_GAP: f32 = 4.0;
/// The search and replacement fields' width, at most; a narrow editor
/// gives them less.
const FIELD_WIDTH: f32 = 260.0;
/// How much of the accent a match's fill keeps, and the selected match's.
const MATCH_FILL: f32 = 0.22;
const SELECTED_MATCH_FILL: f32 = 0.45;
/// The scroll bar's marks: their width in from the editor's right edge,
/// their height, and the least room between two before the second is
/// left out.
const TICK_WIDTH: f32 = 4.0;
const TICK_HEIGHT: f32 = 3.0;
const TICK_SPACING: f32 = 1.0;
/// The longest selection Find takes as its search when it opens.
const SEEDED_QUERY_LIMIT: usize = 256;

/// What the bar keeps between frames. Closed, it keeps its search and
/// options for the next time it opens.
#[derive(Clone, Debug, Default)]
pub(super) struct FindBar {
    pub(super) open: bool,
    replacing: bool,
    query: String,
    replacement: String,
    match_case: bool,
    whole_word: bool,
    regular_expression: bool,
    /// The search field takes the keyboard on the next frame.
    focus_query: bool,
    found: Found,
}

/// The matches of one search in one revision of the text.
#[derive(Clone, Debug, Default)]
struct Found {
    key: Option<(u64, String, FindOptions)>,
    /// Each match as a byte range of the text and as a character range.
    bytes: Vec<Range<usize>>,
    chars: Vec<Range<usize>>,
    /// Each matched line once, as its line and the column of its first
    /// match, for the scroll bar's marks.
    places: Vec<(usize, usize)>,
    invalid: bool,
}

impl FindBar {
    fn options(&self) -> FindOptions {
        FindOptions {
            match_case: self.match_case,
            whole_word: self.whole_word,
            regular_expression: self.regular_expression,
            ..FindOptions::default()
        }
    }

    /// The matches, as character ranges in text order.
    pub(super) fn matches(&self) -> &[Range<usize>] {
        if self.open { &self.found.chars } else { &[] }
    }

    /// Find the matches again when the text, the search or an option has
    /// changed since they were found.
    fn refresh(&mut self, model: &TextDocumentModel) {
        let options = self.options();
        if self
            .found
            .key
            .as_ref()
            .is_some_and(|(revision, query, found_with)| {
                *revision == model.revision() && *query == self.query && *found_with == options
            })
        {
            return;
        }
        self.found = Found {
            key: Some((model.revision(), self.query.clone(), options)),
            ..Found::default()
        };
        if self.query.is_empty() {
            return;
        }
        let text = model.rope().to_string();
        let matches = match find_all_in_source_bounded(&text, &self.query, options, usize::MAX) {
            Ok(found) => found,
            Err(FindError::InvalidRegularExpression(_)) => {
                self.found.invalid = true;
                return;
            }
            Err(_) => return,
        };
        let (mut byte, mut char) = (0, 0);
        for found in matches.matches() {
            let bytes = found.byte_range();
            if bytes.start < byte {
                continue;
            }
            char += text[byte..bytes.start].chars().count();
            let start = char;
            char += text[bytes.clone()].chars().count();
            byte = bytes.end;
            let place = model.line_column_for_char(start);
            if self.found.places.last().map(|(line, _)| *line) != Some(place.0) {
                self.found.places.push(place);
            }
            self.found.bytes.push(bytes);
            self.found.chars.push(start..char);
        }
    }

    /// The match the primary selection is, if it is one.
    fn selected(&self, model: &TextDocumentModel) -> Option<usize> {
        let selected = model.selections().last()?.range();
        self.found
            .chars
            .binary_search_by(|found| (found.start, found.end).cmp(&(selected.start, selected.end)))
            .ok()
    }
}

fn open_request_id(editor_id: Id) -> Id {
    editor_id.with("find-bar-open")
}

/// Open the find bar of the editor `editor_id` the next time it draws, with
/// its replace row when `replace`.
pub(crate) fn queue_open(context: &egui::Context, editor_id: Id, replace: bool) {
    context.data_mut(|data| data.insert_temp(open_request_id(editor_id), replace));
    context.request_repaint();
}

/// Take an open the editor was asked for: the bar opens with the keyboard in
/// its search, which a selection within one line becomes.
pub(super) fn take_open_request(
    context: &egui::Context,
    editor_id: Id,
    session: &mut VirtualEditorSession,
    editable: bool,
) {
    let Some(replace) =
        context.data_mut(|data| data.remove_temp::<bool>(open_request_id(editor_id)))
    else {
        return;
    };
    let bar = &mut session.find;
    bar.open = true;
    bar.replacing = replace && editable;
    bar.focus_query = true;
    if let Some(selection) = session.model.selections().last() {
        let selected = session.model.selected_text(*selection);
        if !selected.is_empty()
            && !selected.contains(['\n', '\r'])
            && selected.chars().count() <= SEEDED_QUERY_LIMIT
        {
            bar.query = selected;
        }
    }
}

/// What a press in the bar asks of the text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    Next,
    Previous,
    Replace,
    ReplaceAll,
    Close,
}

/// Draw the bar at the top of `ui` and carry out what was pressed in it.
/// Says whether the selection moved to a match, so the editor brings it
/// into view.
pub(super) fn show(
    ui: &mut Ui,
    editor_id: Id,
    session: &mut VirtualEditorSession,
    editable: bool,
    style: &VirtualEditorStyle<'_>,
) -> bool {
    let tokens = Tokens::get(ui.ctx());
    let control = compact_height(&tokens.metrics);
    let row = control + 2.0 * BAR_PADDING;
    let replacing = session.find.replacing && editable;
    let height = if replacing { 2.0 * row } else { row };
    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, tokens.color.bg_panel);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(1.0, style.border),
    );
    let mut bar_ui = ui.new_child(
        UiBuilder::new()
            .max_rect(rect.shrink2(Vec2::new(BAR_PADDING * 2.0, BAR_PADDING)))
            .layout(Layout::top_down(Align::Min)),
    );
    bar_ui.spacing_mut().item_spacing = Vec2::new(BAR_GAP, 2.0 * BAR_PADDING);
    bar_ui.spacing_mut().interact_size.y = control;
    let step = bar_rows(
        &mut bar_ui,
        editor_id,
        &mut session.find,
        &session.model,
        editable,
        style,
        control,
    );
    match step {
        Some(Step::Close) => {
            session.find.open = false;
            ui.memory_mut(|memory| memory.request_focus(editor_id));
            false
        }
        Some(Step::Next) => step_to_match(&session.find, &mut session.model, true),
        Some(Step::Previous) => step_to_match(&session.find, &mut session.model, false),
        Some(Step::Replace) if editable => replace_selected(&mut session.find, &mut session.model),
        Some(Step::ReplaceAll) if editable => {
            replace_all(&mut session.find, &mut session.model);
            false
        }
        _ => false,
    }
}

/// The bar's rows; what was pressed.
fn bar_rows(
    ui: &mut Ui,
    editor_id: Id,
    bar: &mut FindBar,
    model: &TextDocumentModel,
    editable: bool,
    style: &VirtualEditorStyle<'_>,
    control: f32,
) -> Option<Step> {
    let messages = style.messages;
    let square = Vec2::splat(control);
    let mut step = None;
    let field_width = FIELD_WIDTH.min((ui.available_width() * 0.45).max(120.0));
    ui.horizontal(|ui| {
        if editable {
            let (icon, label) = if bar.replacing {
                (WorkbenchIcon::ChevronDown, MessageId::EditorFindHideReplace)
            } else {
                (
                    WorkbenchIcon::ChevronRight,
                    MessageId::EditorFindShowReplace,
                )
            };
            let label = messages.text(label);
            if icon_button(ui, icon, &label, false, square)
                .on_hover_text(&label)
                .clicked()
            {
                bar.replacing = !bar.replacing;
            }
        }
        let query_id = super::find_field_id(editor_id);
        let hint = messages.text(MessageId::EditorFindField);
        let field = compact_field(
            ui,
            egui::TextEdit::singleline(&mut bar.query)
                .id(query_id)
                .hint_text(hint.as_str())
                .font(style.font.clone())
                .margin(egui::Margin::symmetric(6, 2)),
            Vec2::new(field_width, control),
        );
        if bar.focus_query {
            bar.focus_query = false;
            field.request_focus();
            select_all(ui.ctx(), query_id, &bar.query);
        }
        if field.lost_focus() {
            // Shift as the Enter press carried it, whatever the frame's
            // modifiers say.
            let (enter, shift, escape) = ui.input(|input| {
                let enter = input.events.iter().find_map(|event| match event {
                    egui::Event::Key {
                        key: Key::Enter,
                        pressed: true,
                        modifiers,
                        ..
                    } => Some(modifiers.shift),
                    _ => None,
                });
                (
                    enter.is_some(),
                    enter == Some(true),
                    input.key_pressed(Key::Escape),
                )
            });
            if escape {
                step = Some(Step::Close);
            } else if enter {
                step = Some(if shift { Step::Previous } else { Step::Next });
                field.request_focus();
            }
        }
        for (on, icon, label) in [
            (
                &mut bar.match_case,
                WorkbenchIcon::MatchCase,
                MessageId::EditorFindMatchCase,
            ),
            (
                &mut bar.whole_word,
                WorkbenchIcon::WholeWord,
                MessageId::EditorFindWholeWord,
            ),
            (
                &mut bar.regular_expression,
                WorkbenchIcon::RegularExpression,
                MessageId::EditorFindRegularExpression,
            ),
        ] {
            let label = messages.text(label);
            if icon_button(ui, icon, &label, *on, square)
                .on_hover_text(&label)
                .clicked()
            {
                *on = !*on;
            }
        }
        // The search as edited this frame, so what the bar says and what a
        // step acts on are its matches.
        bar.refresh(model);
        let selected = bar.selected(model);
        let count = bar.found.chars.len();
        let (summary, color) = if bar.found.invalid {
            (
                messages.text(MessageId::EditorFindInvalidExpression),
                style.error,
            )
        } else if bar.query.is_empty() {
            (String::new(), style.text_dim)
        } else if let Some(index) = selected {
            let current = (index + 1).to_string();
            let total = count.to_string();
            (
                messages.format(
                    MessageId::EditorFindPosition,
                    &[("current", current.as_str()), ("count", total.as_str())],
                ),
                style.text_dim,
            )
        } else {
            let total = count.to_string();
            (
                match count {
                    0 => messages.text(MessageId::EditorFindNoMatches),
                    1 => messages.text(MessageId::EditorFindOneMatch),
                    _ => {
                        messages.format(MessageId::EditorFindMatches, &[("count", total.as_str())])
                    }
                },
                if count == 0 {
                    style.warning
                } else {
                    style.text_dim
                },
            )
        };
        ui.add(egui::Label::new(egui::RichText::new(summary).color(color)).selectable(false));
        ui.add_enabled_ui(count > 0, |ui| {
            for (icon, label, pressed) in [
                (
                    WorkbenchIcon::ChevronUp,
                    MessageId::EditorFindPrevious,
                    Step::Previous,
                ),
                (
                    WorkbenchIcon::ChevronDown,
                    MessageId::EditorFindNext,
                    Step::Next,
                ),
            ] {
                let label = messages.text(label);
                if icon_button(ui, icon, &label, false, square)
                    .on_hover_text(&label)
                    .clicked()
                {
                    step = Some(pressed);
                }
            }
        });
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let label = messages.text(MessageId::EditorFindClose);
            if icon_button(ui, WorkbenchIcon::Close, &label, false, square)
                .on_hover_text(&label)
                .clicked()
            {
                step = Some(Step::Close);
            }
        });
    });
    if bar.replacing && editable {
        ui.horizontal(|ui| {
            ui.add_space(control + BAR_GAP);
            let hint = messages.text(MessageId::EditorFindReplaceField);
            let field = compact_field(
                ui,
                egui::TextEdit::singleline(&mut bar.replacement)
                    .id(editor_id.with("find-bar-replacement"))
                    .hint_text(hint.as_str())
                    .font(style.font.clone())
                    .margin(egui::Margin::symmetric(6, 2)),
                Vec2::new(field_width, control),
            );
            if field.lost_focus() {
                let (enter, escape) = ui.input(|input| {
                    (
                        input.key_pressed(Key::Enter),
                        input.key_pressed(Key::Escape),
                    )
                });
                if escape {
                    step = Some(Step::Close);
                } else if enter {
                    step = Some(Step::Replace);
                    field.request_focus();
                }
            }
            let any = !bar.found.chars.is_empty();
            let replace = messages.text(MessageId::EditorFindReplace);
            if Button::new(&replace)
                .compact()
                .enabled(any)
                .show(ui)
                .clicked()
            {
                step = Some(Step::Replace);
            }
            let replace_all = messages.text(MessageId::EditorFindReplaceAll);
            if Button::new(&replace_all)
                .compact()
                .enabled(any)
                .show(ui)
                .clicked()
            {
                step = Some(Step::ReplaceAll);
            }
        });
    }
    step
}

/// A field of the bar, `size` big: as tall as the bar's other controls
/// rather than a form's, so the replace row fits under the search.
fn compact_field(ui: &mut Ui, edit: egui::TextEdit<'_>, size: Vec2) -> egui::Response {
    ui.add_sized(size, edit.vertical_align(Align::Center))
}

/// Select the whole of the search field's text, so typing replaces it.
fn select_all(context: &egui::Context, field_id: Id, text: &str) {
    let mut state = egui::text_edit::TextEditState::load(context, field_id).unwrap_or_default();
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            egui::text::CCursor::new(text.chars().count()),
        )));
    state.store(context, field_id);
}

/// Select the next match after the selection, or the one before it,
/// wrapping at either end of the text. Says whether one was selected.
fn step_to_match(bar: &FindBar, model: &mut TextDocumentModel, forward: bool) -> bool {
    let matches = &bar.found.chars;
    if matches.is_empty() {
        return false;
    }
    let Some(selected) = model.selections().last().map(|selection| selection.range()) else {
        return false;
    };
    let index = if forward {
        let next = matches.partition_point(|found| found.start < selected.end);
        if next == matches.len() { 0 } else { next }
    } else {
        matches
            .partition_point(|found| found.start < selected.start)
            .checked_sub(1)
            .unwrap_or(matches.len() - 1)
    };
    select_match(model, matches[index].clone());
    true
}

fn select_match(model: &mut TextDocumentModel, found: Range<usize>) {
    let line = model.line_column_for_char(found.start).0;
    model.unfold_at_line(line);
    model.set_selections(vec![TextSelection {
        anchor: found.start,
        cursor: found.end,
    }]);
}

/// Replace the selected match and select the next; with no match selected,
/// select the next. Says whether the selection moved to a match.
fn replace_selected(bar: &mut FindBar, model: &mut TextDocumentModel) -> bool {
    let Some(index) = bar.selected(model) else {
        return step_to_match(bar, model, true);
    };
    let Some(replacement) = replacements(bar, model, &[bar.found.bytes[index].clone()])
        .and_then(|mut replaced| replaced.pop())
    else {
        return false;
    };
    if !model.replace_at_selections(&[replacement]) {
        return false;
    }
    bar.refresh(model);
    step_to_match(bar, model, true);
    true
}

/// Replace every match as one step of the history, the caret left after
/// the first replacement.
fn replace_all(bar: &mut FindBar, model: &mut TextDocumentModel) {
    let Some(replaced) = replacements(bar, model, &bar.found.bytes) else {
        return;
    };
    if replaced.len() != bar.found.chars.len() {
        return;
    }
    model.set_selections(
        bar.found
            .chars
            .iter()
            .map(|found| TextSelection {
                anchor: found.start,
                cursor: found.end,
            })
            .collect(),
    );
    if model.replace_at_selections(&replaced)
        && let Some(first) = model.selections().first().copied()
    {
        model.set_selections(vec![TextSelection::caret(first.cursor)]);
    }
    bar.refresh(model);
}

/// What each of the matches at `bytes` becomes, a regular expression's
/// groups filled in.
fn replacements(
    bar: &FindBar,
    model: &TextDocumentModel,
    bytes: &[Range<usize>],
) -> Option<Vec<String>> {
    let text = model.rope().to_string();
    let outcome = replace_source_ranges(
        &text,
        &bar.query,
        &bar.replacement,
        bar.options(),
        bytes,
        false,
    )
    .ok()?;
    Some(
        outcome
            .replaced_ranges()
            .iter()
            .map(|range| outcome.source()[range.clone()].to_owned())
            .collect(),
    )
}

/// Mark the matches on `line`: each one a wash of the accent, the selected
/// one a stronger wash with an edge.
#[allow(clippy::too_many_arguments)]
pub(super) fn paint_matches(
    painter: &egui::Painter,
    model: &TextDocumentModel,
    bar: &FindBar,
    line: usize,
    row_rect: egui::Rect,
    code_origin_x: f32,
    char_width: f32,
    style: &VirtualEditorStyle<'_>,
) {
    let matches = bar.matches();
    if matches.is_empty() {
        return;
    }
    let Some(span) = model.line_content_char_range(line) else {
        return;
    };
    let selected = model.selections().last().map(|selection| selection.range());
    let first = matches.partition_point(|found| found.end <= span.start);
    for found in matches[first..]
        .iter()
        .take_while(|found| found.start < span.end.max(span.start + 1))
    {
        let start = found.start.max(span.start) - span.start;
        let end = found.end.min(span.end).max(found.start.max(span.start)) - span.start;
        let x0 = code_origin_x + model.cell_at(line, start) as f32 * char_width;
        let x1 = code_origin_x + model.cell_at(line, end) as f32 * char_width;
        let rect = egui::Rect::from_min_max(
            egui::pos2(x0, row_rect.top() + 1.0),
            egui::pos2(x1.max(x0 + 2.0), row_rect.bottom() - 1.0),
        );
        if selected.as_ref() == Some(found) {
            painter.rect_filled(rect, 2.0, style.accent.gamma_multiply(SELECTED_MATCH_FILL));
            painter.rect_stroke(
                rect,
                2.0,
                Stroke::new(1.0, style.accent),
                egui::StrokeKind::Inside,
            );
        } else {
            painter.rect_filled(rect, 2.0, style.accent.gamma_multiply(MATCH_FILL));
        }
    }
}

/// Mark on the scroll bar, along `track`'s right edge, the rows the
/// matches are on: where they stand in the view while the whole text fits
/// it, and in proportion down the scroll bar once it does not. A match
/// inside a fold marks the fold's row.
pub(super) fn paint_ticks(
    painter: &egui::Painter,
    session: &VirtualEditorSession,
    track: egui::Rect,
    style: &VirtualEditorStyle<'_>,
) {
    let bar = &session.find;
    if !bar.open || bar.found.places.is_empty() {
        return;
    }
    let content_height = super::content_size(session, style, 0.0).y;
    let scale = track.height() / content_height.max(track.height());
    let x = (track.right() - TICK_WIDTH - 1.0)..=(track.right() - 1.0);
    let mut last_top = f32::NEG_INFINITY;
    for &(line, column) in &bar.found.places {
        let row = session
            .rows
            .row_of(&session.model, line, column)
            .map(|(row, _)| row)
            .or_else(|| {
                session
                    .model
                    .projected_visible_row_for_logical_line(line)
                    .map(|visible_row| session.rows.first_row_of(visible_row))
            });
        let Some(row) = row else {
            continue;
        };
        let centre =
            track.top() + (style.top_padding + (row as f32 + 0.5) * style.line_height) * scale;
        let top = (centre - TICK_HEIGHT * 0.5)
            .clamp(track.top(), (track.bottom() - TICK_HEIGHT).max(track.top()));
        if top - last_top < TICK_SPACING {
            continue;
        }
        last_top = top;
        painter.rect_filled(
            egui::Rect::from_x_y_ranges(x.clone(), top..=(top + TICK_HEIGHT)),
            0.0,
            style.accent,
        );
    }
}
