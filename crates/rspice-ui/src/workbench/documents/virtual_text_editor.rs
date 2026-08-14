//! Virtualized, language-neutral source editor widget.
//!
//! The widget paints only a bounded row/column viewport backed by
//! [`TextDocumentModel`]. It owns multi-cursor/column selection, transactional
//! editing, folding, clipboard, keyboard navigation, and IME composition while
//! callers retain application-level document identity and persistence.

mod accessibility;

use accessibility::{consume_accesskit_text_selection, update_accesskit_text_state};

use std::ops::Range;
use std::sync::Arc;

use egui::{Color32, FontId, Key, Modifiers, Response, Sense, Stroke, Ui, Vec2};

use super::text_document_model::{TextDocumentModel, TextSelection};
use super::text_editor_commands::{
    ByteSelection, EditorSyntax, EditorViewCommand, StandardCommand, StructuralCommand,
    apply_structural_command, take_queued_caret_char_index, take_queued_standard_command,
    take_queued_structural_command, take_queued_view_command,
};

const ROW_OVERSCAN: usize = 4;
const COLUMN_OVERSCAN: usize = 24;
const MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME: usize = 16_384;
const RIGHT_PADDING: f32 = 24.0;
const BOTTOM_PADDING: f32 = 32.0;
/// Keep the accessibility tree proportional to what assistive technology can
/// use interactively. Exposing an entire release-scale source file would make
/// every caret move rebuild millions of characters of AccessKit state.
const MAX_ACCESSIBLE_LINE_CHARS: usize = 512;
const MAX_ACCESSIBLE_DIAGNOSTIC_CHARS: usize = 384;

#[derive(Clone)]
struct ImeComposition {
    before: TextDocumentModel,
    preedit_ranges: Vec<Range<usize>>,
    active_range: Option<Range<usize>>,
}

#[derive(Clone)]
struct VirtualEditorSession {
    model: TextDocumentModel,
    bound_revision: u64,
    maximum_columns: usize,
    maximum_column_scan: Option<MaximumColumnScan>,
    preferred_column: Option<usize>,
    drag_anchor: Option<(usize, usize)>,
    column_drag: bool,
    ime: Option<ImeComposition>,
}

#[derive(Clone, Copy)]
struct MaximumColumnScan {
    model_revision: u64,
    next_line: usize,
    maximum_columns: usize,
}

impl Default for VirtualEditorSession {
    fn default() -> Self {
        Self {
            model: TextDocumentModel::from_source(""),
            bound_revision: u64::MAX,
            maximum_columns: 0,
            maximum_column_scan: None,
            preferred_column: None,
            drag_anchor: None,
            column_drag: false,
            ime: None,
        }
    }
}

pub(crate) struct VirtualEditorStyle<'a> {
    pub accessible_label: &'a str,
    pub messages: crate::workbench::MessageCatalog,
    pub font: FontId,
    pub line_height: f32,
    pub gutter_width: f32,
    pub code_left_padding: f32,
    pub top_padding: f32,
    pub background: Color32,
    pub hover_background: Color32,
    pub active_background: Color32,
    pub selection_background: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub border: Color32,
    pub accent: Color32,
    pub error: Color32,
    pub warning: Color32,
    pub information: Color32,
    pub edited_lines: &'a std::collections::HashSet<usize>,
    pub breakpoints: &'a [usize],
    pub breakpoints_enabled: bool,
    pub current_line: Option<usize>,
    pub cross_probe_line: Option<usize>,
}

pub(crate) struct VirtualEditorOutput {
    pub changed: bool,
    pub cursor_line: usize,
    pub cursor_char_index: usize,
    /// Character under the pointer in the code well. Gutter, folded-away,
    /// and off-document positions report `None` so language hovers never
    /// guess at a token.
    pub hover_char_index: Option<usize>,
    pub selected_char_range: Option<(usize, usize)>,
    pub breakpoint_toggled: Option<usize>,
    /// Screen position just below the primary caret, for a popover that has to
    /// point at what is being typed. `None` when the caret's line is scrolled
    /// out of the viewport or folded away — an anchor is only offered when
    /// there is something on screen to anchor to, so a caller never has to
    /// guess a position. Independent of the blink phase.
    pub caret_anchor: Option<egui::Pos2>,
    pub response: Response,
}

/// Show a virtualized source document.
///
/// `source_revision` is the application document revision, not a frame counter.
/// `layout_line` receives a bounded horizontal slice and its logical line so a
/// language service can lay it out without constructing a whole-document
/// galley. `line_severity` returns the strongest current diagnostic color for
/// a zero-based logical line. `line_diagnostic_context` is called at most once
/// per frame, for the active line only, and should return a concise summary
/// from a publication-time index rather than scanning diagnostics.
#[allow(clippy::too_many_arguments)]
pub(crate) fn show_virtual_text_editor(
    ui: &mut Ui,
    editor_id: egui::Id,
    source: &mut String,
    source_revision: u64,
    editable: bool,
    syntax: EditorSyntax,
    requested_line: Option<usize>,
    style: &VirtualEditorStyle<'_>,
    mut layout_line: impl FnMut(&Ui, &str, usize, FontId) -> Arc<egui::Galley>,
    mut line_severity: impl FnMut(usize) -> Option<Color32>,
    mut line_diagnostic_context: impl FnMut(usize) -> Option<String>,
) -> VirtualEditorOutput {
    let session_id = editor_id.with("virtual-session");
    let mut session = ui
        .ctx()
        .data_mut(|data| data.remove_temp::<VirtualEditorSession>(session_id))
        .unwrap_or_default();
    if session.bound_revision != source_revision {
        if session.model.synchronize_source(source) {
            restart_maximum_column_scan(&mut session);
            session.ime = None;
        }
        session.bound_revision = source_revision;
    }

    if let Some(line) = requested_line {
        let logical_line = line
            .saturating_sub(1)
            .min(session.model.line_count().saturating_sub(1));
        session.model.unfold_at_line(logical_line);
        let index = session.model.char_at_line_column(logical_line, 0);
        session
            .model
            .set_selections(vec![TextSelection::caret(index)]);
        session.maximum_columns = session
            .maximum_columns
            .max(session.model.maximum_selected_line_columns());
    }

    // After source synchronization, so a completion that just spliced text
    // lands its caret on the document the editor now holds.
    if let Some(char_index) = take_queued_caret_char_index(ui.ctx(), editor_id) {
        let char_index = char_index.min(session.model.len_chars());
        let (line, _) = session.model.line_column_for_char(char_index);
        session.model.unfold_at_line(line);
        session
            .model
            .set_selections(vec![TextSelection::caret(char_index)]);
        session.maximum_columns = session
            .maximum_columns
            .max(session.model.maximum_selected_line_columns());
    }

    advance_maximum_column_scan(&mut session);

    let initial_model_revision = session.model.revision();
    if let Some(command) = take_queued_standard_command(ui.ctx(), editor_id) {
        handle_standard_command(ui, editor_id, &mut session, command, editable);
    }
    if let Some(command) = take_queued_structural_command(ui.ctx(), editor_id)
        && (!command.mutates_source() || editable)
    {
        apply_model_structural(&mut session.model, syntax, command);
    }
    if let Some(command) = take_queued_view_command(ui.ctx(), editor_id) {
        handle_view_command(&mut session, syntax, command);
    }
    consume_accesskit_text_selection(ui, editor_id, &mut session.model, style);
    let mut breakpoint_toggled = None;
    let mut caret_anchor = None;
    let mut hover_char_index = None;
    let mut requested_scroll = requested_line;
    let available = Vec2::new(ui.available_width(), ui.available_height().max(120.0));
    let scroll_output = ui.allocate_ui(available, |ui| {
        egui::ScrollArea::both()
            .id_salt(editor_id.with("scroll"))
            .auto_shrink([false, false])
            .show_viewport(ui, |ui, viewport| {
                let char_width = monospace_advance(ui, &style.font).max(1.0);
                let visible_rows = session.model.visible_line_count().max(1);
                let content_width = (style.gutter_width
                    + style.code_left_padding
                    + session.maximum_columns as f32 * char_width
                    + RIGHT_PADDING)
                    .max(viewport.width());
                let content_height =
                    (style.top_padding + visible_rows as f32 * style.line_height + BOTTOM_PADDING)
                        .max(viewport.height());
                ui.set_min_size(Vec2::new(content_width, content_height));
                let content_rect = egui::Rect::from_min_size(
                    ui.min_rect().min,
                    Vec2::new(content_width, content_height),
                );
                ui.painter()
                    .rect_filled(content_rect, 0.0, style.background);
                let response = ui
                    .interact(content_rect, editor_id, Sense::click_and_drag())
                    .on_hover_cursor(egui::CursorIcon::Text);
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::TextEdit,
                        ui.is_enabled(),
                        style.accessible_label,
                    )
                });
                if response.clicked() {
                    response.request_focus();
                }
                crate::ui::theme::paint_focus_ring(ui, &response, content_rect);

                let origin = content_rect.min;
                let first_row = ((viewport.top() - style.top_padding).max(0.0) / style.line_height)
                    .floor() as usize;
                let row_count = (viewport.height() / style.line_height).ceil() as usize + 1;
                let code_origin_x = origin.x + style.gutter_width + style.code_left_padding;
                let first_column = ((viewport.left() + style.gutter_width - code_origin_x).max(0.0)
                    / char_width)
                    .floor() as usize;
                let column_count = (viewport.width() / char_width).ceil() as usize + 1;

                let pointer_position = response.interact_pointer_pos();
                if response.drag_started()
                    && let Some(pointer) = pointer_position
                {
                    let (line, column) =
                        hit_test(&session.model, pointer, origin, viewport, style, char_width);
                    let modifiers = ui.input(|input| input.modifiers);
                    session.drag_anchor = Some((line, column));
                    session.column_drag = modifiers.alt && modifiers.shift;
                    let index = session.model.char_at_line_column(line, column);
                    if modifiers.alt && !modifiers.shift {
                        session.model.add_cursor(index);
                    } else if modifiers.shift {
                        let anchor = session
                            .model
                            .selections()
                            .last()
                            .map_or(index, |selection| selection.anchor);
                        session.model.set_selections(vec![TextSelection {
                            anchor,
                            cursor: index,
                        }]);
                    } else {
                        session
                            .model
                            .set_selections(vec![TextSelection::caret(index)]);
                    }
                    session.preferred_column = None;
                }
                if response.dragged()
                    && let Some(pointer) = pointer_position
                    && let Some((anchor_line, anchor_column)) = session.drag_anchor
                {
                    let (line, column) =
                        hit_test(&session.model, pointer, origin, viewport, style, char_width);
                    if session.column_drag {
                        session.model.set_column_selection(
                            anchor_line,
                            anchor_column,
                            line,
                            column,
                        );
                    } else {
                        session.model.set_selections(vec![TextSelection {
                            anchor: session
                                .model
                                .char_at_line_column(anchor_line, anchor_column),
                            cursor: session.model.char_at_line_column(line, column),
                        }]);
                    }
                }
                if response.drag_stopped() {
                    session.drag_anchor = None;
                    session.column_drag = false;
                }

                let events = if response.has_focus() {
                    ui.input(|input| input.events.clone())
                } else {
                    Vec::new()
                };
                for event in events {
                    handle_event(ui, &mut session, event, editable, syntax);
                }
                if response.has_focus()
                    && style.breakpoints_enabled
                    && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::F9))
                {
                    let cursor = session
                        .model
                        .selections()
                        .last()
                        .map_or(0, |selection| selection.cursor);
                    breakpoint_toggled = Some(
                        session
                            .model
                            .line_column_for_char(cursor)
                            .0
                            .saturating_add(1),
                    );
                }

                let viewport_lines = session.model.viewport(
                    first_row,
                    row_count,
                    first_column,
                    column_count,
                    ROW_OVERSCAN,
                    COLUMN_OVERSCAN,
                );
                let sticky_gutter_left = origin.x + viewport.left();
                let sticky_separator = sticky_gutter_left + style.gutter_width;
                let code_clip = ui.clip_rect().intersect(egui::Rect::from_min_max(
                    egui::pos2(sticky_separator + 1.0, ui.clip_rect().top()),
                    ui.clip_rect().right_bottom(),
                ));
                let code_painter = ui.painter().with_clip_rect(code_clip);
                let gutter_rect = egui::Rect::from_min_max(
                    egui::pos2(sticky_gutter_left, ui.clip_rect().top()),
                    egui::pos2(sticky_separator, ui.clip_rect().bottom()),
                );
                ui.painter().rect_filled(gutter_rect, 0.0, style.background);
                ui.painter().vline(
                    sticky_separator,
                    gutter_rect.y_range(),
                    Stroke::new(1.0, style.border),
                );

                let pointer_hover = ui.input(|input| input.pointer.hover_pos());
                if let Some(pointer) = pointer_hover.filter(|pointer| {
                    ui.clip_rect().contains(*pointer) && pointer.x > sticky_separator
                }) {
                    let (line, column) =
                        hit_test(&session.model, pointer, origin, viewport, style, char_width);
                    hover_char_index = Some(session.model.char_at_line_column(line, column));
                }
                for line in viewport_lines {
                    let row_top =
                        origin.y + style.top_padding + line.visible_row as f32 * style.line_height;
                    let row_rect = egui::Rect::from_min_size(
                        egui::pos2(origin.x + viewport.left(), row_top),
                        Vec2::new(viewport.width(), style.line_height),
                    );
                    if !row_rect.intersects(ui.clip_rect()) {
                        continue;
                    }
                    let one_based_line = line.logical_line + 1;
                    let is_active = session.model.selections().iter().any(|selection| {
                        session.model.line_column_for_char(selection.cursor).0 == line.logical_line
                    });
                    if style.current_line == Some(one_based_line) {
                        code_painter.rect_filled(
                            row_rect,
                            0.0,
                            style.information.linear_multiply(0.12),
                        );
                    } else if style.cross_probe_line == Some(line.logical_line) {
                        code_painter.rect_filled(
                            row_rect,
                            0.0,
                            style.information.linear_multiply(0.08),
                        );
                    } else if is_active {
                        code_painter.rect_filled(row_rect, 0.0, style.active_background);
                    } else if pointer_hover.is_some_and(|pointer| row_rect.contains(pointer)) {
                        code_painter.rect_filled(row_rect, 0.0, style.hover_background);
                    }

                    paint_selections(
                        &code_painter,
                        &session.model,
                        line.logical_line,
                        row_rect,
                        code_origin_x,
                        char_width,
                        style.selection_background,
                    );
                    let galley = layout_line(ui, &line.text, line.logical_line, style.font.clone());
                    code_painter.galley(
                        egui::pos2(
                            code_origin_x + line.first_column as f32 * char_width,
                            row_top + (style.line_height - galley.size().y) * 0.5,
                        ),
                        galley,
                        style.text,
                    );

                    let severity = line_severity(line.logical_line);
                    let number_color = severity.unwrap_or(if is_active {
                        style.text_dim
                    } else {
                        style.text_faint
                    });
                    let number_right = sticky_separator - 10.0;
                    ui.painter().text(
                        egui::pos2(number_right, row_rect.center().y),
                        egui::Align2::RIGHT_CENTER,
                        one_based_line.to_string(),
                        style.font.clone(),
                        number_color,
                    );
                    if let Some(color) = severity {
                        ui.painter().circle_filled(
                            egui::pos2(sticky_separator - 4.5, row_rect.center().y),
                            2.4,
                            color,
                        );
                    } else if style.edited_lines.contains(&line.logical_line) {
                        ui.painter().circle_filled(
                            egui::pos2(sticky_separator - 4.5, row_rect.center().y),
                            2.4,
                            style.accent,
                        );
                    }
                    if style.breakpoints.contains(&one_based_line) {
                        ui.painter().circle_filled(
                            egui::pos2(sticky_gutter_left + 7.0, row_rect.center().y),
                            4.0,
                            style.error,
                        );
                    }
                    if style.current_line == Some(one_based_line) {
                        ui.painter().circle_filled(
                            egui::pos2(sticky_gutter_left + 18.0, row_rect.center().y),
                            4.0,
                            style.warning,
                        );
                    }
                    if line.folded_line_count > 0 {
                        let marker = egui::Rect::from_center_size(
                            egui::pos2(sticky_gutter_left + 28.0, row_rect.center().y),
                            Vec2::splat(9.0),
                        );
                        ui.painter().rect_stroke(
                            marker,
                            1.0,
                            Stroke::new(1.0, style.text_dim),
                            egui::StrokeKind::Inside,
                        );
                        ui.painter().hline(
                            (marker.left() + 2.0)..=(marker.right() - 2.0),
                            marker.center().y,
                            Stroke::new(1.0, style.text_dim),
                        );
                    }
                }

                if response.clicked_by(egui::PointerButton::Primary)
                    && let Some(pointer) = response.interact_pointer_pos()
                {
                    let (line, _) =
                        hit_test(&session.model, pointer, origin, viewport, style, char_width);
                    let local_x = pointer.x - sticky_gutter_left;
                    if local_x <= 14.0 && style.breakpoints_enabled {
                        breakpoint_toggled = Some(line + 1);
                    } else if (20.0..=36.0).contains(&local_x) {
                        session.model.unfold_at_line(line);
                    }
                }

                caret_anchor =
                    primary_caret_anchor(&session, origin, code_origin_x, char_width, style);
                paint_carets_and_ime(
                    ui,
                    &session,
                    response.has_focus(),
                    origin,
                    viewport,
                    code_origin_x,
                    char_width,
                    style,
                );

                if let Some(line) = requested_scroll.take() {
                    let logical_line = line
                        .saturating_sub(1)
                        .min(session.model.line_count().saturating_sub(1));
                    if let Some(visible_row) =
                        session.model.visible_row_for_logical_line(logical_line)
                    {
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(
                                origin.x,
                                origin.y
                                    + style.top_padding
                                    + visible_row as f32 * style.line_height,
                            ),
                            Vec2::new(content_width, style.line_height),
                        );
                        ui.scroll_to_rect(rect, Some(egui::Align::Center));
                    }
                }
                response
            })
            .inner
    });
    let response = scroll_output.inner;
    let changed = session.model.revision() != initial_model_revision;
    if changed {
        *source = session.model.to_source();
        session.maximum_columns = session
            .maximum_columns
            .max(session.model.maximum_selected_line_columns());
        restart_maximum_column_scan(&mut session);
    }
    let cursor_line = session.model.selections().last().map_or(0, |selection| {
        session.model.line_column_for_char(selection.cursor).0
    });
    let cursor_char_index = session
        .model
        .selections()
        .last()
        .map_or(0, |selection| selection.cursor);
    let selected_char_range = session.model.selections().last().and_then(|selection| {
        let range = selection.range();
        (range.start != range.end).then_some((range.start, range.end))
    });
    update_accesskit_text_state(
        ui,
        editor_id,
        &response,
        &session.model,
        editable,
        style,
        &mut line_diagnostic_context,
    );
    ui.ctx()
        .data_mut(|data| data.insert_temp(session_id, session));
    VirtualEditorOutput {
        changed,
        cursor_line,
        cursor_char_index,
        hover_char_index,
        selected_char_range,
        breakpoint_toggled,
        caret_anchor,
        response,
    }
}

/// Where a popover should sit so it points at the primary caret.
///
/// This repeats the caret's own geometry deliberately: the painter skips the
/// caret on half of every blink cycle, and a popover that moved with the blink
/// would be unusable. `None` means the caret's line is folded or scrolled out
/// of the viewport, which is the one case where there is no honest position to
/// offer.
fn primary_caret_anchor(
    session: &VirtualEditorSession,
    origin: egui::Pos2,
    code_origin_x: f32,
    char_width: f32,
    style: &VirtualEditorStyle<'_>,
) -> Option<egui::Pos2> {
    let cursor = session.model.selections().last()?.cursor;
    let (line, column) = session.model.line_column_for_char(cursor);
    let visible_row = session.model.visible_row_for_logical_line(line)?;
    Some(egui::pos2(
        code_origin_x + column as f32 * char_width,
        origin.y + style.top_padding + (visible_row + 1) as f32 * style.line_height + 4.0,
    ))
}

fn restart_maximum_column_scan(session: &mut VirtualEditorSession) {
    session.maximum_column_scan = Some(MaximumColumnScan {
        model_revision: session.model.revision(),
        next_line: 0,
        maximum_columns: 0,
    });
}

fn advance_maximum_column_scan(session: &mut VirtualEditorSession) {
    if session
        .maximum_column_scan
        .is_some_and(|scan| scan.model_revision != session.model.revision())
    {
        restart_maximum_column_scan(session);
    }
    let Some(scan) = &mut session.maximum_column_scan else {
        return;
    };
    let line_count = session.model.line_count();
    let scanned = (line_count - scan.next_line).min(MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME);
    scan.maximum_columns = scan.maximum_columns.max(
        session
            .model
            .maximum_line_columns_in_range(scan.next_line, scanned),
    );
    scan.next_line += scanned;
    session.maximum_columns = session.maximum_columns.max(scan.maximum_columns);
    if scan.next_line >= line_count {
        session.maximum_columns = scan.maximum_columns;
        session.maximum_column_scan = None;
    }
}

fn monospace_advance(ui: &Ui, font: &FontId) -> f32 {
    ui.fonts_mut(|fonts| {
        fonts
            .layout_no_wrap("M".to_owned(), font.clone(), Color32::WHITE)
            .size()
            .x
    })
}

fn hit_test(
    model: &TextDocumentModel,
    pointer: egui::Pos2,
    origin: egui::Pos2,
    viewport: egui::Rect,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) -> (usize, usize) {
    let visible_row = ((pointer.y - origin.y - style.top_padding) / style.line_height)
        .floor()
        .max(0.0) as usize;
    let logical_line = model
        .logical_line_for_visible_row(visible_row)
        .unwrap_or_else(|| model.line_count().saturating_sub(1));
    let code_origin_x = origin.x + style.gutter_width + style.code_left_padding;
    let column = ((pointer.x - code_origin_x + viewport.left().min(0.0)) / char_width)
        .round()
        .max(0.0) as usize;
    (logical_line, column)
}

fn handle_event(
    ui: &Ui,
    session: &mut VirtualEditorSession,
    event: egui::Event,
    editable: bool,
    syntax: EditorSyntax,
) {
    match event {
        egui::Event::Copy => copy_selection(ui, &session.model),
        egui::Event::Cut if editable => {
            copy_selection(ui, &session.model);
            session.model.insert_at_selections("");
        }
        egui::Event::Paste(text) if editable && !text.is_empty() => {
            let text = normalize_newlines(&text, session.model.preferred_newline());
            grow_maximum_columns_for_insertion(session, &text);
            session.model.insert_at_selections(&text);
            session.preferred_column = None;
        }
        egui::Event::Text(text)
            if editable && !text.is_empty() && !text.chars().any(char::is_control) =>
        {
            grow_maximum_columns_for_insertion(session, &text);
            session.model.insert_at_selections(&text);
            session.preferred_column = None;
        }
        egui::Event::Key {
            key,
            pressed: true,
            modifiers,
            ..
        } => handle_key(ui, session, key, modifiers, editable, syntax),
        egui::Event::Ime(event) if editable => handle_ime(session, event),
        _ => {}
    }
}

fn grow_maximum_columns_for_insertion(session: &mut VirtualEditorSession, inserted: &str) {
    let inserted_columns = inserted
        .split(['\r', '\n'])
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);
    let existing_columns = session.model.maximum_selected_line_columns();
    session.maximum_columns = session
        .maximum_columns
        .max(existing_columns.saturating_add(inserted_columns));
}

fn handle_key(
    ui: &Ui,
    session: &mut VirtualEditorSession,
    key: Key,
    modifiers: Modifiers,
    editable: bool,
    syntax: EditorSyntax,
) {
    let command = modifiers.command;
    if command {
        match key {
            Key::A => {
                session.model.set_selections(vec![TextSelection {
                    anchor: 0,
                    cursor: session.model.len_chars(),
                }]);
                return;
            }
            Key::Z if editable && modifiers.shift => {
                session.model.redo();
                return;
            }
            Key::Z if editable => {
                session.model.undo();
                return;
            }
            Key::Y if editable => {
                session.model.redo();
                return;
            }
            Key::C => {
                copy_selection(ui, &session.model);
                return;
            }
            Key::X if editable => {
                copy_selection(ui, &session.model);
                session.model.insert_at_selections("");
                return;
            }
            Key::D if editable => {
                apply_model_structural(
                    &mut session.model,
                    syntax,
                    StructuralCommand::DuplicateLine,
                );
                return;
            }
            Key::Slash if editable => {
                apply_model_structural(
                    &mut session.model,
                    syntax,
                    StructuralCommand::ToggleComment,
                );
                return;
            }
            Key::K if editable && modifiers.shift => {
                apply_model_structural(&mut session.model, syntax, StructuralCommand::DeleteLine);
                return;
            }
            Key::Backslash if modifiers.shift => {
                apply_model_structural(&mut session.model, syntax, StructuralCommand::MatchBracket);
                return;
            }
            Key::ArrowLeft | Key::ArrowRight => {
                move_horizontal(
                    &mut session.model,
                    key == Key::ArrowRight,
                    modifiers.shift,
                    true,
                );
                session.preferred_column = None;
                return;
            }
            _ => {}
        }
    }
    match key {
        Key::ArrowLeft => {
            move_horizontal(&mut session.model, false, modifiers.shift, false);
            session.preferred_column = None;
        }
        Key::ArrowRight => {
            move_horizontal(&mut session.model, true, modifiers.shift, false);
            session.preferred_column = None;
        }
        Key::ArrowUp if !modifiers.alt => move_vertical(session, -1, modifiers.shift),
        Key::ArrowDown if !modifiers.alt => move_vertical(session, 1, modifiers.shift),
        Key::Home => move_line_edge(&mut session.model, false, modifiers.shift, command),
        Key::End => move_line_edge(&mut session.model, true, modifiers.shift, command),
        Key::Backspace if editable => {
            session.model.delete_backward();
            session.preferred_column = None;
        }
        Key::Delete if editable => {
            session.model.delete_forward();
            session.preferred_column = None;
        }
        Key::Enter if editable => {
            insert_smart_newline(&mut session.model, syntax);
            session.preferred_column = None;
        }
        Key::Tab if editable && !modifiers.shift => {
            apply_model_structural(&mut session.model, syntax, StructuralCommand::Indent);
            session.preferred_column = None;
        }
        Key::Tab if editable && modifiers.shift => {
            apply_model_structural(&mut session.model, syntax, StructuralCommand::Outdent);
            session.preferred_column = None;
        }
        Key::ArrowUp if editable && modifiers.alt => {
            apply_model_structural(&mut session.model, syntax, StructuralCommand::MoveLineUp);
        }
        Key::ArrowDown if editable && modifiers.alt => {
            apply_model_structural(&mut session.model, syntax, StructuralCommand::MoveLineDown);
        }
        Key::Escape => {
            if session.model.selections().len() > 1 {
                let primary = session
                    .model
                    .selections()
                    .last()
                    .copied()
                    .unwrap_or(TextSelection::caret(0));
                session.model.set_selections(vec![primary]);
            }
        }
        Key::OpenBracket if editable && command && modifiers.shift => fold_selection(session),
        Key::CloseBracket if command && modifiers.shift => {
            let cursor = session
                .model
                .selections()
                .last()
                .map_or(0, |selection| selection.cursor);
            let line = session.model.line_column_for_char(cursor).0;
            session.model.unfold_at_line(line);
        }
        _ => {}
    }
}

fn handle_standard_command(
    ui: &Ui,
    editor_id: egui::Id,
    session: &mut VirtualEditorSession,
    command: StandardCommand,
    editable: bool,
) {
    match command {
        StandardCommand::Undo if editable => {
            session.model.undo();
        }
        StandardCommand::Redo if editable => {
            session.model.redo();
        }
        StandardCommand::Copy => copy_selection(ui, &session.model),
        StandardCommand::Cut if editable => {
            copy_selection(ui, &session.model);
            session.model.insert_at_selections("");
        }
        StandardCommand::Paste if editable => ui
            .ctx()
            .send_viewport_cmd(egui::ViewportCommand::RequestPaste),
        StandardCommand::SelectAll => session.model.set_selections(vec![TextSelection {
            anchor: 0,
            cursor: session.model.len_chars(),
        }]),
        _ => {}
    }
    ui.ctx()
        .memory_mut(|memory| memory.request_focus(editor_id));
}

fn apply_model_structural(
    model: &mut TextDocumentModel,
    syntax: EditorSyntax,
    command: StructuralCommand,
) -> bool {
    if command == StructuralCommand::MatchBracket {
        return apply_primary_structural(model, syntax, command);
    }
    let mut groups = structural_line_groups(model);
    if groups.is_empty() {
        return false;
    }
    if matches!(
        command,
        StructuralCommand::MoveLineUp | StructuralCommand::MoveLineDown
    ) && groups.len() > 1
    {
        let combined = groups[0].start..groups.last().unwrap().end;
        groups.clear();
        groups.push(combined);
    }
    let mut source = model.to_source();
    let mut mapped_byte_selections: Vec<ByteSelection> = Vec::with_capacity(groups.len());
    let mut any_changed = false;
    for group in groups.into_iter().rev() {
        let anchor_char = model.char_at_line_column(group.start, 0);
        let cursor_char = if group.end < model.line_count() {
            model.char_at_line_column(group.end, 0)
        } else {
            model.len_chars()
        };
        let selection = ByteSelection {
            anchor: model.char_to_byte(anchor_char),
            cursor: model.char_to_byte(cursor_char),
        };
        let before_len = source.len();
        let result = apply_structural_command(&mut source, selection, syntax, command);
        let delta = source.len() as isize - before_len as isize;
        let changed_from = selection
            .anchor
            .min(selection.cursor)
            .min(result.selection.anchor.min(result.selection.cursor));
        if delta != 0 {
            for mapped in &mut mapped_byte_selections {
                mapped.anchor = shift_byte_after(mapped.anchor, changed_from, delta);
                mapped.cursor = shift_byte_after(mapped.cursor, changed_from, delta);
            }
        }
        mapped_byte_selections.push(result.selection);
        any_changed |= result.changed;
    }
    mapped_byte_selections.reverse();
    let mapped = mapped_byte_selections
        .into_iter()
        .map(|selection| TextSelection {
            anchor: byte_to_char_index(&source, selection.anchor),
            cursor: byte_to_char_index(&source, selection.cursor),
        })
        .collect::<Vec<_>>();
    if any_changed {
        model.replace_source_transaction(&source, mapped)
    } else {
        model.set_selections(mapped);
        false
    }
}

fn apply_primary_structural(
    model: &mut TextDocumentModel,
    syntax: EditorSyntax,
    command: StructuralCommand,
) -> bool {
    let selection = model
        .selections()
        .last()
        .copied()
        .unwrap_or(TextSelection::caret(model.len_chars()));
    let mut source = model.to_source();
    let result = apply_structural_command(
        &mut source,
        ByteSelection {
            anchor: model.char_to_byte(selection.anchor),
            cursor: model.char_to_byte(selection.cursor),
        },
        syntax,
        command,
    );
    let mapped = TextSelection {
        anchor: byte_to_char_index(&source, result.selection.anchor),
        cursor: byte_to_char_index(&source, result.selection.cursor),
    };
    if result.changed {
        model.replace_source_transaction(&source, vec![mapped])
    } else {
        model.set_selections(vec![mapped]);
        false
    }
}

fn structural_line_groups(model: &TextDocumentModel) -> Vec<Range<usize>> {
    let mut groups = model
        .selections()
        .iter()
        .copied()
        .map(|selection| {
            let range = selection.range();
            let start_line = model.line_column_for_char(range.start).0;
            let (end_line, end_column) = model.line_column_for_char(range.end);
            let end = if range.end > range.start && end_column == 0 {
                end_line.max(start_line + 1)
            } else {
                end_line.saturating_add(1)
            };
            start_line..end.min(model.line_count())
        })
        .collect::<Vec<_>>();
    groups.sort_by_key(|range| (range.start, range.end));
    let mut merged: Vec<Range<usize>> = Vec::with_capacity(groups.len());
    for group in groups {
        if let Some(previous) = merged.last_mut()
            && group.start <= previous.end
        {
            previous.end = previous.end.max(group.end);
        } else {
            merged.push(group);
        }
    }
    merged
}

fn shift_byte_after(value: usize, threshold: usize, delta: isize) -> usize {
    if value < threshold {
        value
    } else {
        value.saturating_add_signed(delta)
    }
}

fn byte_to_char_index(source: &str, byte_index: usize) -> usize {
    let mut index = byte_index.min(source.len());
    while index > 0 && !source.is_char_boundary(index) {
        index -= 1;
    }
    source[..index].chars().count()
}

fn copy_selection(ui: &Ui, model: &TextDocumentModel) {
    let selected = model
        .selections()
        .iter()
        .copied()
        .filter(|selection| !selection.range().is_empty())
        .map(|selection| model.selected_text(selection))
        .collect::<Vec<_>>();
    if !selected.is_empty() {
        ui.copy_text(selected.join(model.preferred_newline()));
    }
}

fn normalize_newlines(text: &str, newline: &str) -> String {
    if newline == "\n" {
        return text.replace("\r\n", "\n").replace('\r', "\n");
    }
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
        .replace('\n', newline)
}

fn insert_smart_newline(model: &mut TextDocumentModel, syntax: EditorSyntax) -> bool {
    let newline = model.preferred_newline();
    let replacements = model
        .selections()
        .iter()
        .copied()
        .map(|selection| {
            let insertion = selection.range().start;
            let line = model.line_column_for_char(insertion).0;
            let line_start = model
                .line_content_char_range(line)
                .map_or(insertion, |range| range.start);
            let prefix = model.text_range(line_start..insertion);
            let indentation = prefix
                .chars()
                .take_while(|character| matches!(character, ' ' | '\t'))
                .collect::<String>();
            let trimmed = prefix.trim_end();
            let opens_block = match syntax {
                EditorSyntax::Python | EditorSyntax::Yaml => trimmed.ends_with(':'),
                EditorSyntax::VerilogA => trimmed.ends_with('{'),
                EditorSyntax::Spice => {
                    trimmed.split_ascii_whitespace().next().is_some_and(|head| {
                        matches!(
                            head.to_ascii_lowercase().as_str(),
                            ".subckt" | ".if" | ".control" | ".lib"
                        )
                    })
                }
                EditorSyntax::Toml => false,
            };
            format!(
                "{newline}{indentation}{}",
                if opens_block {
                    syntax.indentation()
                } else {
                    ""
                }
            )
        })
        .collect::<Vec<_>>();
    model.replace_at_selections(&replacements)
}

fn move_horizontal(model: &mut TextDocumentModel, forward: bool, extend: bool, by_word: bool) {
    let length = model.len_chars();
    let selections = model
        .selections()
        .iter()
        .copied()
        .map(|selection| {
            let range = selection.range();
            let cursor = if !extend && !range.is_empty() {
                if forward { range.end } else { range.start }
            } else if forward {
                next_boundary(model, selection.cursor, length, by_word)
            } else {
                previous_boundary(model, selection.cursor, by_word)
            };
            TextSelection {
                anchor: if extend { selection.anchor } else { cursor },
                cursor,
            }
        })
        .collect();
    model.set_selections(selections);
}

fn next_boundary(
    model: &TextDocumentModel,
    mut cursor: usize,
    length: usize,
    by_word: bool,
) -> usize {
    if !by_word {
        return (cursor + 1).min(length);
    }
    while cursor < length && model.char_at(cursor).is_some_and(char::is_whitespace) {
        cursor += 1;
    }
    while cursor < length && model.char_at(cursor).is_some_and(is_word_character) {
        cursor += 1;
    }
    if cursor < length && !model.char_at(cursor).is_some_and(char::is_whitespace) {
        cursor += 1;
    }
    cursor
}

fn previous_boundary(model: &TextDocumentModel, mut cursor: usize, by_word: bool) -> usize {
    if !by_word {
        return cursor.saturating_sub(1);
    }
    while cursor > 0 && model.char_at(cursor - 1).is_some_and(char::is_whitespace) {
        cursor -= 1;
    }
    while cursor > 0 && model.char_at(cursor - 1).is_some_and(is_word_character) {
        cursor -= 1;
    }
    if cursor > 0 && !model.char_at(cursor - 1).is_some_and(char::is_whitespace) {
        cursor -= 1;
    }
    cursor
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || matches!(character, '_' | '$')
}

fn move_vertical(session: &mut VirtualEditorSession, delta: isize, extend: bool) {
    let selections = session.model.selections().to_vec();
    let mut moved = Vec::with_capacity(selections.len());
    for selection in selections {
        let (line, column) = session.model.line_column_for_char(selection.cursor);
        let preferred = *session.preferred_column.get_or_insert(column);
        let target_line = line
            .saturating_add_signed(delta)
            .min(session.model.line_count().saturating_sub(1));
        let cursor = session.model.char_at_line_column(target_line, preferred);
        moved.push(TextSelection {
            anchor: if extend { selection.anchor } else { cursor },
            cursor,
        });
    }
    session.model.set_selections(moved);
}

fn move_line_edge(model: &mut TextDocumentModel, end: bool, extend: bool, document: bool) {
    let selections = model
        .selections()
        .iter()
        .copied()
        .map(|selection| {
            let cursor = if document {
                if end { model.len_chars() } else { 0 }
            } else {
                let line = model.line_column_for_char(selection.cursor).0;
                model
                    .line_content_char_range(line)
                    .map_or(
                        selection.cursor,
                        |range| if end { range.end } else { range.start },
                    )
            };
            TextSelection {
                anchor: if extend { selection.anchor } else { cursor },
                cursor,
            }
        })
        .collect();
    model.set_selections(selections);
}

fn fold_selection(session: &mut VirtualEditorSession) {
    let Some(selection) = session.model.selections().last().copied() else {
        return;
    };
    let start = session
        .model
        .line_column_for_char(selection.range().start)
        .0;
    let end = session
        .model
        .line_column_for_char(selection.range().end)
        .0
        .saturating_add(1);
    session.model.fold_lines(start, end);
}

fn handle_view_command(
    session: &mut VirtualEditorSession,
    syntax: EditorSyntax,
    command: EditorViewCommand,
) {
    match command {
        EditorViewCommand::FoldSelection => fold_selection(session),
        EditorViewCommand::UnfoldAtCaret => {
            let cursor = session
                .model
                .selections()
                .last()
                .map_or(0, |selection| selection.cursor);
            let line = session.model.line_column_for_char(cursor).0;
            session.model.unfold_at_line(line);
        }
        EditorViewCommand::FoldAll => {
            session.model.unfold_all();
            for (start, end) in discover_fold_regions(&session.model, syntax) {
                session.model.fold_lines(start, end);
            }
        }
        EditorViewCommand::UnfoldAll => {
            session.model.unfold_all();
        }
    }
}

fn discover_fold_regions(model: &TextDocumentModel, syntax: EditorSyntax) -> Vec<(usize, usize)> {
    match syntax {
        EditorSyntax::Python | EditorSyntax::Yaml => indentation_fold_regions(model),
        EditorSyntax::Toml => toml_fold_regions(model),
        EditorSyntax::Spice => delimited_fold_regions(
            model,
            &[
                (".subckt", ".ends"),
                (".control", ".endc"),
                (".if", ".endif"),
            ],
        ),
        EditorSyntax::VerilogA => delimited_fold_regions(
            model,
            &[
                ("module", "endmodule"),
                ("function", "endfunction"),
                ("discipline", "enddiscipline"),
                ("nature", "endnature"),
            ],
        ),
    }
}

fn indentation_fold_regions(model: &TextDocumentModel) -> Vec<(usize, usize)> {
    let indent = |line: &str| {
        line.chars()
            .take_while(|character| matches!(character, ' ' | '\t'))
            .map(|character| if character == '\t' { 4 } else { 1 })
            .sum::<usize>()
    };
    let mut regions = Vec::new();
    let mut open = Vec::<(usize, usize)>::new();
    let mut previous = None::<(usize, usize, bool)>;
    for (line_index, line) in model.line_slices().enumerate() {
        let line = line.to_string();
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let current_indent = indent(&line);
        while open
            .last()
            .is_some_and(|(_, base_indent)| current_indent <= *base_indent)
        {
            let (start, _) = open.pop().expect("checked non-empty fold stack");
            if line_index > start + 1 {
                regions.push((start, line_index));
            }
        }
        if let Some((previous_line, previous_indent, previous_can_open)) = previous
            && previous_can_open
            && current_indent > previous_indent
        {
            open.push((previous_line, previous_indent));
        }
        previous = Some((line_index, current_indent, !trimmed.starts_with('#')));
    }
    for (start, _) in open {
        if model.line_count() > start + 1 {
            regions.push((start, model.line_count()));
        }
    }
    regions.sort_unstable();
    regions
}

fn toml_fold_regions(model: &TextDocumentModel) -> Vec<(usize, usize)> {
    let mut regions = Vec::new();
    let mut previous_heading = None;
    for (line, text) in model.line_slices().enumerate() {
        if !text.to_string().trim_start().starts_with('[') {
            continue;
        }
        if let Some(start) = previous_heading
            && line > start + 1
        {
            regions.push((start, line));
        }
        previous_heading = Some(line);
    }
    if let Some(start) = previous_heading
        && model.line_count() > start + 1
    {
        regions.push((start, model.line_count()));
    }
    regions
}

fn delimited_fold_regions(
    model: &TextDocumentModel,
    delimiters: &[(&str, &str)],
) -> Vec<(usize, usize)> {
    let mut stack = Vec::<(usize, usize)>::new();
    let mut regions = Vec::new();
    for (line, text) in model.line_slices().enumerate() {
        let text = text.to_string();
        let head = text
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if let Some((kind, _)) = delimiters
            .iter()
            .enumerate()
            .find(|(_, (open, _))| head == *open)
        {
            stack.push((kind, line));
            continue;
        }
        if let Some((kind, _)) = delimiters
            .iter()
            .enumerate()
            .find(|(_, (_, close))| head == *close)
            && let Some(position) = stack.iter().rposition(|(opened, _)| *opened == kind)
        {
            let (_, start) = stack.remove(position);
            if line + 1 > start + 1 {
                regions.push((start, line + 1));
            }
        }
    }
    regions.sort_unstable();
    regions
}

fn handle_ime(session: &mut VirtualEditorSession, event: egui::ImeEvent) {
    match event {
        egui::ImeEvent::Preedit {
            text,
            active_range_chars,
        } => {
            let before = session.ime.as_ref().map_or_else(
                || session.model.clone(),
                |composition| composition.before.clone(),
            );
            session.model = before.clone();
            if text.is_empty() {
                session.ime = None;
                return;
            }
            grow_maximum_columns_for_insertion(session, &text);
            let starts = session
                .model
                .selections()
                .iter()
                .map(|selection| selection.range().start)
                .collect::<Vec<_>>();
            session.model.insert_at_selections(&text);
            let length = text.chars().count();
            let preedit_ranges = starts
                .into_iter()
                .map(|start| start..start + length)
                .collect();
            session.ime = Some(ImeComposition {
                before,
                preedit_ranges,
                active_range: active_range_chars,
            });
        }
        egui::ImeEvent::Commit(text) => {
            if let Some(composition) = session.ime.take() {
                session.model = composition.before;
            }
            if !text.is_empty() {
                grow_maximum_columns_for_insertion(session, &text);
                session.model.insert_at_selections(&text);
            }
        }
        #[allow(deprecated)]
        egui::ImeEvent::Enabled | egui::ImeEvent::Disabled => {}
    }
}

fn paint_selections(
    painter: &egui::Painter,
    model: &TextDocumentModel,
    line: usize,
    row_rect: egui::Rect,
    code_origin_x: f32,
    char_width: f32,
    color: Color32,
) {
    let Some(line_range) = model.line_content_char_range(line) else {
        return;
    };
    for selection in model.selections() {
        let selection = selection.range();
        let start = selection.start.max(line_range.start);
        let end = selection.end.min(line_range.end);
        if start >= end {
            continue;
        }
        let x0 = code_origin_x + (start - line_range.start) as f32 * char_width;
        let x1 = code_origin_x + (end - line_range.start) as f32 * char_width;
        painter.rect_filled(
            egui::Rect::from_min_max(
                egui::pos2(x0, row_rect.top()),
                egui::pos2(x1.max(x0 + 1.0), row_rect.bottom()),
            ),
            0.0,
            color,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_carets_and_ime(
    ui: &Ui,
    session: &VirtualEditorSession,
    focused: bool,
    origin: egui::Pos2,
    viewport: egui::Rect,
    code_origin_x: f32,
    char_width: f32,
    style: &VirtualEditorStyle<'_>,
) {
    if !focused {
        return;
    }
    let time = ui.input(|input| input.time);
    let caret_visible = (time * 2.0).fract() < 0.62;
    let mut primary_caret_rect = None;
    if caret_visible {
        for selection in session.model.selections() {
            let (line, column) = session.model.line_column_for_char(selection.cursor);
            let Some(visible_row) = session.model.visible_row_for_logical_line(line) else {
                continue;
            };
            let x = code_origin_x + column as f32 * char_width;
            let y = origin.y + style.top_padding + visible_row as f32 * style.line_height;
            let rect = egui::Rect::from_min_size(
                egui::pos2(x, y + 1.0),
                Vec2::new(1.25, style.line_height - 2.0),
            );
            ui.painter().rect_filled(rect, 0.0, style.text);
            primary_caret_rect = Some(rect);
        }
    }
    if let Some(composition) = &session.ime {
        for range in &composition.preedit_ranges {
            let (line, start_column) = session.model.line_column_for_char(range.start);
            let (_, end_column) = session.model.line_column_for_char(range.end);
            let Some(visible_row) = session.model.visible_row_for_logical_line(line) else {
                continue;
            };
            let y =
                origin.y + style.top_padding + (visible_row + 1) as f32 * style.line_height - 1.0;
            ui.painter().hline(
                (code_origin_x + start_column as f32 * char_width)
                    ..=(code_origin_x + end_column as f32 * char_width),
                y,
                Stroke::new(1.0, style.accent),
            );
            if let Some(active) = &composition.active_range {
                let start = start_column + active.start.min(end_column - start_column);
                let end = start_column + active.end.min(end_column - start_column);
                ui.painter().hline(
                    (code_origin_x + start as f32 * char_width)
                        ..=(code_origin_x + end as f32 * char_width),
                    y - 1.0,
                    Stroke::new(2.0, style.accent),
                );
            }
        }
    }
    if let Some(cursor_rect) = primary_caret_rect
        && cursor_rect.intersects(ui.clip_rect())
    {
        let to_global = ui
            .ctx()
            .layer_transform_to_global(ui.layer_id())
            .unwrap_or_default();
        ui.output_mut(|output| {
            output.ime = Some(egui::output::IMEOutput {
                rect: to_global * ui.clip_rect(),
                cursor_rect: to_global * cursor_rect,
                should_interrupt_composition: false,
            });
        });
    }
    let _ = viewport;
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(250));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_style<'a>(edited: &'a std::collections::HashSet<usize>) -> VirtualEditorStyle<'a> {
        VirtualEditorStyle {
            accessible_label: "Test source editor",
            messages: crate::workbench::MessageCatalog::new(
                crate::workbench::UiTextLocale::EnglishUnitedStates,
            ),
            font: FontId::monospace(11.0),
            line_height: 17.0,
            gutter_width: 47.0,
            code_left_padding: 12.0,
            top_padding: 8.0,
            background: Color32::BLACK,
            hover_background: Color32::from_gray(24),
            active_background: Color32::from_gray(28),
            selection_background: Color32::from_rgb(32, 64, 96),
            text: Color32::WHITE,
            text_dim: Color32::LIGHT_GRAY,
            text_faint: Color32::GRAY,
            border: Color32::DARK_GRAY,
            accent: Color32::LIGHT_BLUE,
            error: Color32::RED,
            warning: Color32::YELLOW,
            information: Color32::LIGHT_BLUE,
            edited_lines: edited,
            breakpoints: &[],
            breakpoints_enabled: false,
            current_line: None,
            cross_probe_line: None,
        }
    }

    #[test]
    fn newline_normalization_respects_document_policy() {
        assert_eq!(normalize_newlines("a\r\nb\rc\n", "\n"), "a\nb\nc\n");
        assert_eq!(normalize_newlines("a\r\nb\rc\n", "\r\n"), "a\r\nb\r\nc\r\n");
    }

    #[test]
    fn accessible_text_is_whitespace_normalized_and_strictly_bounded() {
        assert_eq!(
            accessibility::bounded_accessible_text(" alpha\n\tbeta ", 32),
            "alpha beta"
        );
        let bounded = accessibility::bounded_accessible_text(&"x".repeat(10_000), 64);
        assert_eq!(bounded.chars().count(), 64);
        assert!(bounded.ends_with('…'));
    }

    #[test]
    fn ime_preedit_is_replaced_instead_of_accumulated() {
        let mut session = VirtualEditorSession::default();
        handle_ime(
            &mut session,
            egui::ImeEvent::Preedit {
                text: "k".to_owned(),
                active_range_chars: Some(0..1),
            },
        );
        handle_ime(
            &mut session,
            egui::ImeEvent::Preedit {
                text: "ka".to_owned(),
                active_range_chars: Some(0..2),
            },
        );
        assert_eq!(session.model.to_source(), "ka");
        handle_ime(&mut session, egui::ImeEvent::Commit("か".to_owned()));
        assert_eq!(session.model.to_source(), "か");
        assert!(session.model.undo());
        assert_eq!(session.model.to_source(), "");
    }

    #[test]
    fn release_text_fidelity_matrix_preserves_large_unicode_ime_clipboard_and_paste_content() {
        let mut large_bytes = Vec::with_capacity(1_000_010);
        large_bytes.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
        large_bytes.extend_from_slice(b"* ");
        large_bytes.extend(std::iter::repeat_n(b'R', 1_000_000));
        large_bytes.extend_from_slice(b"\r\n.end\r\n");
        let large = TextDocumentModel::from_bytes(&large_bytes).expect("large source is valid");
        assert_eq!(
            large.encoding(),
            super::super::text_document_model::TextEncoding::Utf8Bom
        );
        assert_eq!(large.preferred_newline(), "\r\n");
        assert_eq!(large.encoded_bytes(), large_bytes);
        let far_right = large.viewport(0, 1, 999_980, 32, 0, 0);
        assert_eq!(far_right.len(), 1);
        assert!(far_right[0].text.chars().count() <= 32);
        assert_eq!(far_right[0].first_column, 999_980);

        let alpha = '\u{03B1}';
        let beta = '\u{03B2}';
        let mut selected = TextDocumentModel::from_source(&format!("{alpha}\r\n{beta}\r\n"));
        selected.set_selections(vec![
            TextSelection {
                anchor: 0,
                cursor: 1,
            },
            TextSelection {
                anchor: 3,
                cursor: 4,
            },
        ]);
        let context = egui::Context::default();
        let output = context.run_ui(Default::default(), |ui| copy_selection(ui, &selected));
        let expected_copy = format!("{alpha}\r\n{beta}");
        assert!(output.platform_output.commands.iter().any(|command| {
            matches!(
                command,
                egui::OutputCommand::CopyText(text) if text.as_str() == expected_copy
            )
        }));

        let mut session = VirtualEditorSession {
            model: TextDocumentModel::from_source("prefix\r\n"),
            ..VirtualEditorSession::default()
        };
        session
            .model
            .set_selections(vec![TextSelection::caret(session.model.len_chars())]);
        let paste = "\u{4E00}\n\u{4E8C}\r\u{4E09}";
        let _ = context.run_ui(Default::default(), |ui| {
            handle_event(
                ui,
                &mut session,
                egui::Event::Paste(paste.to_owned()),
                true,
                EditorSyntax::Python,
            );
        });
        assert_eq!(
            session.model.to_source(),
            "prefix\r\n\u{4E00}\r\n\u{4E8C}\r\n\u{4E09}"
        );

        let before_ime = session.model.to_source();
        handle_ime(
            &mut session,
            egui::ImeEvent::Preedit {
                text: "\u{65E5}".to_owned(),
                active_range_chars: Some(0..1),
            },
        );
        handle_ime(
            &mut session,
            egui::ImeEvent::Preedit {
                text: "\u{65E5}\u{672C}".to_owned(),
                active_range_chars: Some(0..2),
            },
        );
        handle_ime(
            &mut session,
            egui::ImeEvent::Commit("\u{65E5}\u{672C}\u{8A9E}\u{1F9EA}".to_owned()),
        );
        assert_eq!(
            session.model.to_source(),
            format!("{before_ime}\u{65E5}\u{672C}\u{8A9E}\u{1F9EA}")
        );
        assert!(session.model.undo());
        assert_eq!(session.model.to_source(), before_ime);

        let invalid = [b'o', b'k', 0xF0, 0x28, 0x8C, 0x28];
        assert_eq!(
            TextDocumentModel::from_bytes(&invalid).unwrap_err(),
            super::super::text_document_model::SourceTextError::InvalidUtf8 { valid_up_to: 2 }
        );
    }

    #[test]
    fn word_navigation_is_unicode_safe() {
        let mut model = TextDocumentModel::from_source("αβ gamma");
        model.set_selections(vec![TextSelection::caret(model.len_chars())]);
        move_horizontal(&mut model, false, false, true);
        assert_eq!(model.selections(), &[TextSelection::caret(3)]);
        move_horizontal(&mut model, false, false, true);
        assert_eq!(model.selections(), &[TextSelection::caret(0)]);
    }

    #[test]
    fn disjoint_multicursor_structural_edit_is_one_transaction() {
        let mut model = TextDocumentModel::from_source("a\nb\nc\n");
        model.set_selections(vec![TextSelection::caret(0), TextSelection::caret(4)]);
        assert!(apply_model_structural(
            &mut model,
            EditorSyntax::Spice,
            StructuralCommand::ToggleComment,
        ));
        assert_eq!(model.to_source(), "* a\nb\n* c\n");
        assert_eq!(model.selections().len(), 2);
        assert!(model.undo());
        assert_eq!(model.to_source(), "a\nb\nc\n");
    }

    #[test]
    fn smart_newline_preserves_line_endings_and_indents_language_blocks() {
        let mut model = TextDocumentModel::from_source("    if ready:\r\n");
        model.set_selections(vec![TextSelection::caret(13)]);
        assert!(insert_smart_newline(&mut model, EditorSyntax::Python));
        assert_eq!(model.to_source(), "    if ready:\r\n        \r\n");
    }

    #[test]
    fn fold_all_discovers_language_regions_and_unfold_all_restores_every_line() {
        let mut session = VirtualEditorSession {
            model: TextDocumentModel::from_source(
                "module gain(in, out);\n  analog begin\n    V(out) <+ V(in);\n  end\nendmodule\nmodule other;\nendmodule\n",
            ),
            ..VirtualEditorSession::default()
        };
        handle_view_command(
            &mut session,
            EditorSyntax::VerilogA,
            EditorViewCommand::FoldAll,
        );
        assert_eq!(session.model.folds().len(), 2);
        assert!(session.model.visible_line_count() < session.model.line_count());
        handle_view_command(
            &mut session,
            EditorSyntax::VerilogA,
            EditorViewCommand::UnfoldAll,
        );
        assert!(session.model.folds().is_empty());
        assert_eq!(
            session.model.visible_line_count(),
            session.model.line_count()
        );
    }

    #[test]
    fn requested_line_unfolds_and_reveals_a_collapsed_target() {
        let context = egui::Context::default();
        let editor_id = egui::Id::new("folded-reveal-editor-test");
        let mut source = ".subckt amp in out\nR1 in out 1k\n.ends\n.end\n".to_owned();
        let mut session = VirtualEditorSession {
            model: TextDocumentModel::from_source(&source),
            ..Default::default()
        };
        assert!(session.model.fold_lines(0, 3));
        session.bound_revision = 1;
        session.maximum_columns = session.model.maximum_line_columns();
        context.data_mut(|data| {
            data.insert_temp(editor_id.with("virtual-session"), session);
        });
        let edited = std::collections::HashSet::new();
        let style = test_style(&edited);
        let _ = context.run_ui(Default::default(), |ui| {
            let _ = show_virtual_text_editor(
                ui,
                editor_id,
                &mut source,
                1,
                true,
                EditorSyntax::Spice,
                Some(2),
                &style,
                |ui, text, _line, font| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                    })
                },
                |_| None,
                |_| None,
            );
        });
        let session = context
            .data_mut(|data| {
                data.remove_temp::<VirtualEditorSession>(editor_id.with("virtual-session"))
            })
            .expect("retained virtual-editor session");
        assert!(session.model.folds().is_empty());
        assert_eq!(
            session
                .model
                .line_column_for_char(session.model.selections()[0].cursor),
            (1, 0)
        );
    }

    #[test]
    fn indentation_and_toml_fold_discovery_are_deterministic() {
        let python = TextDocumentModel::from_source(
            "def outer():\n    if ready:\n        return 1\n    return 0\nnext_value = 2\n",
        );
        assert_eq!(
            discover_fold_regions(&python, EditorSyntax::Python),
            vec![(0, 4), (1, 3)]
        );
        let toml =
            TextDocumentModel::from_source("title = 'x'\n[first]\na = 1\nb = 2\n[second]\nc = 3\n");
        assert_eq!(
            discover_fold_regions(&toml, EditorSyntax::Toml),
            vec![(1, 4), (4, 7)]
        );
    }

    #[test]
    fn five_million_line_fold_discovery_streams_without_a_line_vector() {
        let source = "x\n".repeat(5_000_000);
        let model = TextDocumentModel::from_source(&source);
        assert!(discover_fold_regions(&model, EditorSyntax::Python).is_empty());
        assert!(discover_fold_regions(&model, EditorSyntax::Spice).is_empty());
    }

    #[test]
    fn widget_lays_out_only_a_bounded_large_document_viewport() {
        let context = egui::Context::default();
        let mut source = (0..100_000)
            .map(|line| format!("R{line} n{line} 0 1k\n"))
            .collect::<String>();
        let original = source.clone();
        let edited = std::collections::HashSet::new();
        let style = test_style(&edited);
        let mut laid_out = 0_usize;
        let mut changed = true;
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            ..Default::default()
        };
        let _ = context.run_ui(raw_input, |ui| {
            let output = show_virtual_text_editor(
                ui,
                egui::Id::new("large-editor-test"),
                &mut source,
                1,
                true,
                EditorSyntax::Spice,
                None,
                &style,
                |ui, text, _line, font| {
                    laid_out += 1;
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                    })
                },
                |_| None,
                |_| None,
            );
            changed = output.changed;
        });

        assert!(!changed);
        assert_eq!(source, original);
        assert!(laid_out > 0);
        assert!(laid_out < 80, "laid out {laid_out} rows");
    }

    #[test]
    fn maximum_column_reconciliation_is_revision_bound_and_chunked() {
        let mut source = "x\n".repeat(MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME + 2);
        source.push_str(&"z".repeat(1_000));
        let mut session = VirtualEditorSession {
            model: TextDocumentModel::from_source(&source),
            maximum_columns: 0,
            ..VirtualEditorSession::default()
        };
        restart_maximum_column_scan(&mut session);
        advance_maximum_column_scan(&mut session);
        let first = session
            .maximum_column_scan
            .expect("large document scan remains pending");
        assert_eq!(first.next_line, MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME);
        assert_eq!(session.maximum_columns, 1);
        advance_maximum_column_scan(&mut session);
        assert!(session.maximum_column_scan.is_none());
        assert_eq!(session.maximum_columns, 1_000);

        assert!(session.model.synchronize_source("a\n"));
        restart_maximum_column_scan(&mut session);
        assert_eq!(
            session.maximum_columns, 1_000,
            "the old extent remains usable until the bounded scan completes"
        );
        advance_maximum_column_scan(&mut session);
        assert!(session.maximum_column_scan.is_none());
        assert_eq!(session.maximum_columns, 1);

        let inserted = format!("short\n{}\nend", "q".repeat(2_000));
        grow_maximum_columns_for_insertion(&mut session, &inserted);
        assert!(
            session.maximum_columns >= 2_000,
            "a long intermediate pasted line must grow the canvas immediately"
        );
    }

    #[test]
    fn focused_debug_editor_exposes_f9_breakpoint_toggle() {
        let context = egui::Context::default();
        let mut source = "print('ready')\n".to_owned();
        let edited = std::collections::HashSet::new();
        let mut style = test_style(&edited);
        style.breakpoints_enabled = true;
        let editor_id = egui::Id::new("breakpoint-shortcut-editor-test");
        let raw_input = || egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            ..Default::default()
        };
        let _ = context.run_ui(raw_input(), |ui| {
            let output = show_virtual_text_editor(
                ui,
                editor_id,
                &mut source,
                1,
                true,
                EditorSyntax::Python,
                None,
                &style,
                |ui, text, _line, font| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                    })
                },
                |_| None,
                |_| None,
            );
            output.response.request_focus();
        });

        let mut input = raw_input();
        input.events.push(egui::Event::Key {
            key: Key::F9,
            physical_key: Some(Key::F9),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        });
        let mut toggled = None;
        let _ = context.run_ui(input, |ui| {
            toggled = show_virtual_text_editor(
                ui,
                editor_id,
                &mut source,
                1,
                true,
                EditorSyntax::Python,
                None,
                &style,
                |ui, text, _line, font| {
                    ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                    })
                },
                |_| None,
                |_| None,
            )
            .breakpoint_toggled;
        });
        assert_eq!(toggled, Some(1));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn widget_exposes_active_line_read_only_and_diagnostic_context_to_accesskit() {
        let context = egui::Context::default();
        context.enable_accesskit();
        crate::ui::Theme::default().apply(&context);
        let mut source = "R1 input output 1k\nV1 input 0 1\n".to_owned();
        let mut edited = std::collections::HashSet::new();
        edited.insert(1);
        let breakpoints = [2];
        let mut style = test_style(&edited);
        style.accessible_label = "Generated SPICE netlist viewer";
        style.breakpoints = &breakpoints;
        style.current_line = Some(2);
        let raw_input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                Vec2::new(900.0, 600.0),
            )),
            ..Default::default()
        };
        let nodes = context
            .run_ui(raw_input, |ui| {
                let _ = show_virtual_text_editor(
                    ui,
                    egui::Id::new("accessible-editor-test"),
                    &mut source,
                    1,
                    false,
                    EditorSyntax::Spice,
                    Some(2),
                    &style,
                    |ui, text, _line, font| {
                        ui.fonts_mut(|fonts| {
                            fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                        })
                    },
                    |_| None,
                    |line| {
                        (line == 1).then(|| {
                            "One warning diagnostic on this line, TEST-001: Check the source."
                                .to_owned()
                        })
                    },
                );
            })
            .platform_output
            .accesskit_update
            .expect("AccessKit editor tree")
            .nodes;

        let editor = nodes
            .iter()
            .find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::MultilineTextInput
                    && node.label() == Some("Generated SPICE netlist viewer"))
                .then_some(node)
            })
            .expect("named multiline editor node");
        let description = editor.description().expect("editor description");
        assert!(description.contains("Line 2 of 3, column 1. Read-only."));
        assert!(description.contains("One warning diagnostic on this line"));
        assert!(description.contains("TEST-001"));
        assert!(description.contains("Breakpoint on this line"));
        assert!(description.contains("Current debugger statement"));
        assert!(description.contains("Modified line"));
        assert!(editor.is_read_only());
        assert!(editor.text_selection().is_some());
        let active_text = nodes
            .iter()
            .find_map(|(_, node)| {
                (node.role() == egui::accesskit::Role::TextRun
                    && node.value() == Some("V1 input 0 1"))
                .then_some(node)
            })
            .expect("active-line TextRun");
        let bounds = active_text.bounds().expect("active-line bounds");
        assert!(
            bounds.y0 >= f64::from(style.top_padding + style.line_height),
            "line-two accessibility geometry must not overlap line one: {bounds:?}"
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn accesskit_active_line_payload_is_bounded_for_extreme_source_lines() {
        let context = egui::Context::default();
        context.enable_accesskit();
        crate::ui::Theme::default().apply(&context);
        let mut source = "x".repeat(50_000);
        let edited = std::collections::HashSet::new();
        let style = test_style(&edited);
        let nodes = context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        Vec2::new(900.0, 600.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    let _ = show_virtual_text_editor(
                        ui,
                        egui::Id::new("bounded-accessible-editor-test"),
                        &mut source,
                        1,
                        true,
                        EditorSyntax::Spice,
                        None,
                        &style,
                        |ui, text, _line, font| {
                            ui.fonts_mut(|fonts| {
                                fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                            })
                        },
                        |_| None,
                        |_| None,
                    );
                },
            )
            .platform_output
            .accesskit_update
            .expect("AccessKit editor tree")
            .nodes;
        let exposed_characters = nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::TextRun)
            .map(|(_, node)| node.value().unwrap_or_default().chars().count())
            .sum::<usize>();
        assert_eq!(exposed_characters, MAX_ACCESSIBLE_LINE_CHARS);
        assert!(nodes.iter().any(|(_, node)| {
            node.description()
                .is_some_and(|description| description.contains("of 50000"))
        }));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn accesskit_excerpt_geometry_tracks_the_real_horizontal_source_column() {
        let context = egui::Context::default();
        context.enable_accesskit();
        crate::ui::Theme::default().apply(&context);
        let editor_id = egui::Id::new("horizontal-accessible-editor-test");
        let mut source = "x".repeat(2_000);
        let mut session = VirtualEditorSession {
            model: TextDocumentModel::from_source(&source),
            ..Default::default()
        };
        session
            .model
            .set_selections(vec![TextSelection::caret(1_500)]);
        session.bound_revision = 1;
        session.maximum_columns = session.model.maximum_line_columns();
        context.data_mut(|data| {
            data.insert_temp(editor_id.with("virtual-session"), session);
        });
        let edited = std::collections::HashSet::new();
        let style = test_style(&edited);
        let nodes = context
            .run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        Vec2::new(900.0, 600.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    let _ = show_virtual_text_editor(
                        ui,
                        editor_id,
                        &mut source,
                        1,
                        true,
                        EditorSyntax::Spice,
                        None,
                        &style,
                        |ui, text, _line, font| {
                            ui.fonts_mut(|fonts| {
                                fonts.layout_no_wrap(text.to_owned(), font, Color32::WHITE)
                            })
                        },
                        |_| None,
                        |_| None,
                    );
                },
            )
            .platform_output
            .accesskit_update
            .expect("AccessKit editor tree")
            .nodes;
        let text_run = nodes
            .iter()
            .find_map(|(_, node)| (node.role() == egui::accesskit::Role::TextRun).then_some(node))
            .expect("active-line excerpt TextRun");
        let bounds = text_run.bounds().expect("active-line excerpt bounds");
        assert!(
            bounds.x0 > 1_000.0,
            "far-right source text must retain far-right geometry: {bounds:?}"
        );
        assert!(nodes.iter().any(|(_, node)| {
            node.description()
                .is_some_and(|description| description.contains("Source excerpt columns"))
        }));
    }
}
