//! Virtualized, language-neutral source editor widget.
//!
//! The widget paints only a bounded row/column viewport backed by
//! [`TextDocumentModel`]. It owns multi-cursor/column selection, transactional
//! editing, folding, clipboard, keyboard navigation, and IME composition while
//! callers retain application-level document identity and persistence.

mod accessibility;
mod keymap;
mod touch;

pub(crate) use accessibility::bounded_accessible_text;
use accessibility::{consume_accesskit_text_selection, update_accesskit_text_state};
use keymap::{KeyAction, Motion, Reach};

use std::ops::Range;
use std::sync::Arc;

use egui::{Color32, FontId, Key, Modifiers, Response, Sense, Stroke, Ui, Vec2};

use crate::ui::widgets::{Problem, paint_problem_mark};

use super::text_document_model::{TextDocumentModel, TextSelection};
use super::text_editor_commands::{
    ByteSelection, EditorSyntax, apply_structural_command, publish_caret_line,
    take_queued_caret_char_index, take_queued_caret_position, take_queued_standard_command,
    take_queued_structural_command, take_queued_text_replacement, take_queued_view_command,
    veriloga_blocks::{
        dedent_after_typing, dedent_closing_word_at_caret, leaves_block_open, veriloga_fold_regions,
    },
};
use crate::workbench::framework::requests::{
    EditorViewCommand, StandardCommand, StructuralCommand,
};

const ROW_OVERSCAN: usize = 4;
const COLUMN_OVERSCAN: usize = 24;
const MAXIMUM_COLUMN_SCAN_LINES_PER_FRAME: usize = 16_384;
const RIGHT_PADDING: f32 = 24.0;
const BOTTOM_PADDING: f32 = 32.0;
/// Side of a diagnostic's mark in the gutter, between the line number and the
/// code: small enough to sit in the number's right padding.
const GUTTER_MARK_SIDE: f32 = 8.0;
/// Width of the change bar an edited line carries at the gutter's edge.
const CHANGE_BAR_WIDTH: f32 = 2.0;
/// Keep the accessibility tree proportional to what assistive technology can
/// use interactively. Exposing an entire release-scale source file would make
/// every caret move rebuild millions of characters of AccessKit state.
const MAX_ACCESSIBLE_LINE_CHARS: usize = 512;
const MAX_ACCESSIBLE_DIAGNOSTIC_CHARS: usize = 384;
/// Rows and columns kept between a caret that moved and the viewport's edge.
const REVEAL_MARGIN_ROWS: f32 = 2.0;
const REVEAL_MARGIN_COLUMNS: f32 = 4.0;
/// How long the announcement of a Tab mode change stays in the tree.
const TAB_MODE_ANNOUNCEMENT_SECONDS: f64 = 5.0;

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
    /// An undo or redo this frame asked for that the editor's own history
    /// could not answer, for the owner to step a history of its own.
    unanswered_history: Option<StandardCommand>,
    /// The scroll area drawn last frame and its viewport's size, so a key
    /// can move the view in the same frame it moves the caret.
    scroll_id: Option<egui::Id>,
    viewport_size: Option<Vec2>,
    /// When the primary caret last moved. The blink restarts from it, so a
    /// caret is drawn the moment it arrives.
    caret_moved_at: f64,
    touch: touch::TouchGesture,
}

/// How a caret that moved is brought into view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Reveal {
    /// Scroll as little as keeps the caret a margin inside the viewport.
    Nearest,
    /// Leave a caret already in view alone; centre one that is not.
    CenterIfOutside,
}

/// What the keyboard and text input of one frame did besides editing.
#[derive(Default)]
struct InputOutcome {
    caret_moved: bool,
    /// Rows the view scrolls by with the caret, for PageUp and PageDown.
    page_rows: isize,
    release_focus: bool,
    toggle_tab_moves_focus: bool,
}

/// What a key press needs to know about the editor it lands in.
struct KeyContext {
    os: egui::os::OperatingSystem,
    editable: bool,
    syntax: EditorSyntax,
    page_rows: usize,
    tab_moves_focus: bool,
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
            unanswered_history: None,
            scroll_id: None,
            viewport_size: None,
            caret_moved_at: 0.0,
            touch: touch::TouchGesture::default(),
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
    pub edited_lines: &'a std::collections::HashSet<usize>,
    pub breakpoints: &'a [usize],
    pub breakpoints_enabled: bool,
    pub current_line: Option<usize>,
    pub cross_probe_line: Option<usize>,
    /// The stretches of text findings cover, by line, each underlined in
    /// the shape of its severity.
    pub marks: &'a [SourceMark],
    /// The findings were made before the latest edits: their marks are
    /// drawn dimmed.
    pub marks_outdated: bool,
}

/// The stretch of one line a finding covers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SourceMark {
    /// Zero-based logical line.
    pub line: usize,
    /// Character columns on the line. An empty stretch, a finding at the
    /// end of a line, is marked one character wide.
    pub columns: std::ops::Range<usize>,
    pub severity: rspice_vocabulary::diagnostics::DiagnosticSeverity,
}

/// How much of its ink an outdated finding's mark keeps.
const OUTDATED_MARK_OPACITY: f32 = 0.45;

/// A row the viewport is about to paint, for the language service that lays
/// it out.
pub(crate) struct LayoutRow<'a> {
    /// The row's text from `first_column`, as wide as the viewport needs.
    pub text: &'a str,
    pub logical_line: usize,
    /// The character column `text` starts at.
    pub first_column: usize,
    /// The whole document, for a reading that carries from line to line.
    pub document: &'a TextDocumentModel,
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
    /// Undo or Redo, when it was asked for and the editor's own history had
    /// no step to take. An owner with an older history of its own (a bulk
    /// replace, a rename) steps that one, so the request is never lost.
    pub unanswered_history: Option<StandardCommand>,
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
/// `layout_line` receives a bounded horizontal slice of a row, where it starts
/// and the document it belongs to, so a language service can lay it out
/// without constructing a whole-document galley. `line_problem` returns the mark the strongest current diagnostic on
/// a zero-based logical line earns: a warning or an error, or none for a note
/// or a hint. `line_diagnostic_context` is called at most once
/// per frame, for the active line only, and should return a concise summary
/// from a publication-time index rather than scanning diagnostics.
pub(crate) fn show_virtual_text_editor(
    ui: &mut Ui,
    editor_id: egui::Id,
    source: &mut String,
    source_revision: u64,
    editable: bool,
    syntax: EditorSyntax,
    requested_line: Option<usize>,
    style: &VirtualEditorStyle<'_>,
    mut layout_line: impl FnMut(&Ui, &LayoutRow<'_>, FontId) -> Arc<egui::Galley>,
    mut line_problem: impl FnMut(usize) -> Option<Problem>,
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
    let mut reveal = None;
    if let Some(char_index) = take_queued_caret_char_index(ui.ctx(), editor_id) {
        let char_index = char_index.min(session.model.len_chars());
        place_caret(&mut session, char_index);
        reveal = Some(Reveal::Nearest);
    }
    if let Some((line, column)) = take_queued_caret_position(ui.ctx(), editor_id) {
        let logical_line = line
            .saturating_sub(1)
            .min(session.model.line_count().saturating_sub(1));
        let index = session
            .model
            .char_at_line_column(logical_line, column.saturating_sub(1));
        place_caret(&mut session, index);
        reveal = Some(Reveal::CenterIfOutside);
    }

    advance_maximum_column_scan(&mut session);

    let initial_model_revision = session.model.revision();
    if let Some(command) = take_queued_standard_command(ui.ctx(), editor_id) {
        handle_standard_command(ui, editor_id, &mut session, command, editable);
        reveal = reveal.or(Some(Reveal::Nearest));
    }
    if let Some(command) = take_queued_structural_command(ui.ctx(), editor_id)
        && (!command.mutates_source() || editable)
    {
        apply_model_structural(&mut session.model, syntax, command);
        reveal = reveal.or(Some(Reveal::Nearest));
    }
    if let Some((range, text)) = take_queued_text_replacement(ui.ctx(), editor_id)
        && editable
    {
        replace_text_range(&mut session, range, &text, syntax);
        reveal = reveal.or(Some(Reveal::Nearest));
    }
    if let Some(command) = take_queued_view_command(ui.ctx(), editor_id) {
        handle_view_command(&mut session, syntax, command);
        reveal = reveal.or(Some(Reveal::Nearest));
    }
    if consume_accesskit_text_selection(ui, editor_id, &mut session.model, style) {
        reveal = reveal.or(Some(Reveal::Nearest));
    }

    // Keys act before the view is laid out, so the frame that moves the
    // caret is the frame that shows it.
    let char_width = monospace_advance(ui, &style.font).max(1.0);
    let tab_moves_focus = tab_moves_focus_mode(ui.ctx());
    let mut input = InputOutcome::default();
    if ui.memory(|memory| memory.has_focus(editor_id)) {
        let keys = KeyContext {
            os: ui.ctx().os(),
            editable,
            syntax,
            page_rows: page_rows(&session, ui, style),
            tab_moves_focus,
        };
        let events = ui.input(|input| input.events.clone());
        // egui moves focus on arrows and Tab before any widget runs, unless
        // the focused widget's lock filter claims them; that filter is only
        // in force from focus's second frame. A key this editor acts on must
        // never also move focus, so a direction egui took from it is undone.
        let claims_tab = editable && !tab_moves_focus;
        if events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key { key, pressed: true, .. }
                    if matches!(
                        key,
                        Key::ArrowLeft | Key::ArrowRight | Key::ArrowUp | Key::ArrowDown
                    ) || (claims_tab && *key == Key::Tab)
            )
        }) {
            ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
        }
        for event in events {
            handle_event(ui, &mut session, event, &keys, &mut input);
        }
    }
    if input.toggle_tab_moves_focus {
        set_tab_moves_focus(ui.ctx(), editor_id, !tab_moves_focus);
    }
    if input.caret_moved {
        reveal = reveal.or(Some(Reveal::Nearest));
    }
    // A page moves the view and the caret together, so the caret keeps its
    // place on screen; it is only brought back if the page could not move
    // the view as far as the caret went.
    if input.page_rows != 0 {
        reveal = reveal.map(|_| Reveal::CenterIfOutside);
    }
    if reveal.is_some() || input.page_rows != 0 {
        session.caret_moved_at = ui.input(|input| input.time);
    }
    if input.release_focus {
        ui.memory_mut(|memory| memory.surrender_focus(editor_id));
    }
    let scroll_offset = planned_scroll_offset(
        ui.ctx(),
        &session,
        style,
        char_width,
        reveal,
        input.page_rows,
    );

    let tab_moves_focus = tab_moves_focus_mode(ui.ctx());
    let mut breakpoint_toggled = None;
    let mut caret_anchor = None;
    let mut hover_char_index = None;
    let mut requested_scroll = requested_line;
    let available = Vec2::new(ui.available_width(), ui.available_height().max(120.0));
    let scroll_output = ui.allocate_ui(available, |ui| {
        let mut scroll_area = egui::ScrollArea::both()
            .id_salt(editor_id.with("scroll"))
            .auto_shrink([false, false]);
        if let Some(offset) = scroll_offset {
            scroll_area = scroll_area.scroll_offset(offset);
        }
        scroll_area.show_viewport(ui, |ui, viewport| {
            session.viewport_size = Some(viewport.size());
            let content_size = content_size(&session, style, char_width).max(viewport.size());
            let content_width = content_size.x;
            ui.set_min_size(content_size);
            let content_rect = egui::Rect::from_min_size(ui.min_rect().min, content_size);
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
            let pressed = response.is_pointer_button_down_on()
                && ui.input(|input| input.pointer.button_pressed(egui::PointerButton::Primary));
            if pressed {
                session
                    .touch
                    .begin_press(ui.input(egui::InputState::any_touches));
            }
            // A mouse press focuses the editor, so a drag that selects
            // leaves the keyboard with the selection; a finger focuses it
            // only with a tap, so scrolling never raises a keyboard.
            // Focus is asked for only when it is elsewhere: asking again
            // would drop the key filter below and interrupt a composition.
            if (response.clicked() || (pressed && !session.touch.is_touch()))
                && !response.has_focus()
            {
                response.request_focus();
            }

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
            let hit = |session: &VirtualEditorSession, pointer| {
                hit_test(&session.model, pointer, origin, viewport, style, char_width)
            };
            // A finger may be about to scroll, so only a mouse or a pen
            // places the caret as it goes down; a tap places it when it
            // lifts. The gutter's breakpoints and fold marks take their
            // own clicks, so a press there leaves the caret alone.
            let gutter_right = origin.x + viewport.left() + style.gutter_width;
            if pressed
                && !session.touch.is_touch()
                && let Some(pointer) = pointer_position
                && pointer.x > gutter_right
            {
                let (line, column) = hit(&session, pointer);
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
                session.caret_moved_at = ui.input(|input| input.time);
            }
            if session.touch.is_touch() {
                // One finger dragging scrolls the source under it.
                if response.dragged() {
                    ui.scroll_with_delta_animation(
                        response.drag_delta(),
                        egui::style::ScrollAnimation::none(),
                    );
                }
                if response.clicked()
                    && let Some(pointer) = pointer_position
                {
                    let (line, column) = hit(&session, pointer);
                    let index = session.model.char_at_line_column(line, column);
                    session
                        .model
                        .set_selections(vec![TextSelection::caret(index)]);
                    session.preferred_column = None;
                    session.touch.show_handles(false);
                }
            }
            if response.double_clicked()
                && let Some(pointer) = pointer_position
            {
                let (line, column) = hit(&session, pointer);
                let index = session.model.char_at_line_column(line, column);
                select_word_at(&mut session.model, index);
                session.drag_anchor = None;
                session.preferred_column = None;
                session.touch.show_handles(session.touch.is_touch());
            }
            if response.dragged()
                && !session.touch.is_touch()
                && let Some(pointer) = pointer_position
                && let Some((anchor_line, anchor_column)) = session.drag_anchor
            {
                let (line, column) =
                    hit_test(&session.model, pointer, origin, viewport, style, char_width);
                if session.column_drag {
                    session
                        .model
                        .set_column_selection(anchor_line, anchor_column, line, column);
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
            // A right-click (or a long press) opens the context menu on
            // what it points at: outside the selection it first moves the
            // caret there, inside it keeps the selection the menu acts on.
            // A long press selects the word under the finger, with the
            // handles that adjust it.
            let long_touched = response.long_touched();
            if (response.secondary_clicked() || long_touched)
                && let Some(pointer) = ui.input(|input| input.pointer.interact_pos())
            {
                let (line, column) = hit(&session, pointer);
                let index = session.model.char_at_line_column(line, column);
                let inside_selection = session.model.selections().iter().any(|selection| {
                    let range = selection.range();
                    range.start < range.end && range.start <= index && index <= range.end
                });
                if !inside_selection {
                    if long_touched {
                        select_word_at(&mut session.model, index);
                    } else {
                        session
                            .model
                            .set_selections(vec![TextSelection::caret(index)]);
                    }
                    session.preferred_column = None;
                }
                session.touch.show_handles(long_touched);
                if !response.has_focus() {
                    response.request_focus();
                }
            }
            if let Some(selection) = touch::drag_selection_handles(
                ui,
                editor_id,
                &mut session,
                response.has_focus(),
                origin,
                viewport,
                style,
                char_width,
            ) {
                session.model.set_selections(vec![selection]);
                session.preferred_column = None;
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
            // Marks of findings made before the latest edits are dimmed.
            let mut mark_painter = code_painter.clone();
            let mut gutter_mark_painter = ui.painter().clone();
            if style.marks_outdated {
                mark_painter.multiply_opacity(OUTDATED_MARK_OPACITY);
                gutter_mark_painter.multiply_opacity(OUTDATED_MARK_OPACITY);
            }
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
            if let Some(pointer) = pointer_hover
                .filter(|pointer| ui.clip_rect().contains(*pointer) && pointer.x > sticky_separator)
            {
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
                // The debugger's current line and the cross-probed line are
                // where the reader is, so they take the active-item accent.
                if style.current_line == Some(one_based_line) {
                    code_painter.rect_filled(row_rect, 0.0, style.accent.linear_multiply(0.12));
                } else if style.cross_probe_line == Some(line.logical_line) {
                    code_painter.rect_filled(row_rect, 0.0, style.accent.linear_multiply(0.08));
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
                let galley = layout_line(
                    ui,
                    &LayoutRow {
                        text: &line.text,
                        logical_line: line.logical_line,
                        first_column: line.first_column,
                        document: &session.model,
                    },
                    style.font.clone(),
                );
                code_painter.galley(
                    egui::pos2(
                        code_origin_x + line.first_column as f32 * char_width,
                        row_top + (style.line_height - galley.size().y) * 0.5,
                    ),
                    galley,
                    style.text,
                );
                let first_mark = style
                    .marks
                    .partition_point(|mark| mark.line < line.logical_line);
                let line_marks = style.marks[first_mark..]
                    .iter()
                    .take_while(|mark| mark.line == line.logical_line);
                for mark in line_marks {
                    paint_source_mark(
                        &mark_painter,
                        mark,
                        code_origin_x,
                        row_top + style.line_height - 2.5,
                        char_width,
                        style,
                    );
                }

                // The line number is neutral. A warning or an error on
                // the line is a mark between the number and the code; an
                // edited line is a change bar at the gutter's edge.
                let problem = line_problem(line.logical_line);
                let number_color = if is_active {
                    style.text_dim
                } else {
                    style.text_faint
                };
                let number_right = sticky_separator - 10.0;
                ui.painter().text(
                    egui::pos2(number_right, row_rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    one_based_line.to_string(),
                    style.font.clone(),
                    number_color,
                );
                if let Some(problem) = problem {
                    paint_problem_mark(
                        &gutter_mark_painter,
                        egui::Rect::from_center_size(
                            egui::pos2(sticky_separator - 4.5, row_rect.center().y),
                            Vec2::splat(GUTTER_MARK_SIDE),
                        ),
                        problem,
                    );
                } else if style.edited_lines.contains(&line.logical_line) {
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(sticky_separator - 1.0 - CHANGE_BAR_WIDTH, row_top),
                            egui::pos2(sticky_separator - 1.0, row_top + style.line_height),
                        ),
                        0.0,
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

            caret_anchor = primary_caret_anchor(&session, origin, code_origin_x, char_width, style);
            let focused = response.has_focus();
            paint_carets_and_ime(
                ui,
                &session,
                focused,
                editable,
                origin,
                code_origin_x,
                char_width,
                style,
            );
            touch::paint_selection_handles(
                &code_painter,
                &session,
                focused,
                origin,
                style,
                char_width,
            );
            if focused && tab_moves_focus {
                paint_tab_moves_focus_badge(ui, viewport, origin, style);
            }

            if let Some(line) = requested_scroll.take() {
                let logical_line = line
                    .saturating_sub(1)
                    .min(session.model.line_count().saturating_sub(1));
                if let Some(visible_row) = session.model.visible_row_for_logical_line(logical_line)
                {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(
                            origin.x,
                            origin.y + style.top_padding + visible_row as f32 * style.line_height,
                        ),
                        Vec2::new(content_width, style.line_height),
                    );
                    ui.scroll_to_rect(rect, Some(egui::Align::Center));
                }
            }
            // The editor takes focus on every press, so a press never
            // frames it: only focus that Tab moved here shows the ring,
            // and the caret says where typing goes the rest of the time.
            // Painted last, over the code, and around the visible part of
            // a document that scrolls past its viewport.
            if crate::ui::theme::focus_is_visible(ui.ctx(), &response) {
                crate::ui::theme::paint_focus_ring(
                    ui,
                    &response,
                    ui.clip_rect().intersect(content_rect),
                );
            }
            response
        })
    });
    session.scroll_id = Some(scroll_output.inner.id);
    let response = scroll_output.inner.inner;
    if response.has_focus() {
        claim_editing_keys(ui, editor_id, editable && !tab_moves_focus);
        announce_tab_mode_change(ui, editor_id, &response, tab_moves_focus, style);
    }
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
    publish_caret_line(ui.ctx(), editor_id, cursor_line);
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
        tab_moves_focus,
        style,
        &mut line_diagnostic_context,
    );
    let unanswered_history = session.unanswered_history.take();
    ui.ctx()
        .data_mut(|data| data.insert_temp(session_id, session));
    VirtualEditorOutput {
        changed,
        cursor_line,
        cursor_char_index,
        hover_char_index,
        selected_char_range,
        breakpoint_toggled,
        unanswered_history,
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

/// Replace characters `range` with `text` as one undoable edit, the caret
/// after it. A closing word put in this way takes its line back as a typed
/// one does.
fn replace_text_range(
    session: &mut VirtualEditorSession,
    range: Range<usize>,
    text: &str,
    syntax: EditorSyntax,
) {
    let length = session.model.len_chars();
    session.model.set_selections(vec![TextSelection {
        anchor: range.start.min(length),
        cursor: range.end.min(length),
    }]);
    grow_maximum_columns_for_insertion(session, text);
    session.model.insert_at_selections(text);
    if syntax == EditorSyntax::VerilogA {
        dedent_closing_word_at_caret(&mut session.model);
    }
    session.preferred_column = None;
}

/// Put a single caret at `char_index`, unfolding its line.
fn place_caret(session: &mut VirtualEditorSession, char_index: usize) {
    let (line, _) = session.model.line_column_for_char(char_index);
    session.model.unfold_at_line(line);
    session
        .model
        .set_selections(vec![TextSelection::caret(char_index)]);
    session.preferred_column = None;
    session.maximum_columns = session
        .maximum_columns
        .max(session.model.maximum_selected_line_columns());
}

/// Select the word at `char_index`, or the single character there when it
/// is not part of a word. A line break is never selected.
fn select_word_at(model: &mut TextDocumentModel, char_index: usize) {
    let length = model.len_chars();
    let is_word = |index: usize| model.char_at(index).is_some_and(is_word_character);
    let range = if is_word(char_index) || (char_index > 0 && is_word(char_index - 1)) {
        let mut start = char_index;
        while start > 0 && is_word(start - 1) {
            start -= 1;
        }
        let mut end = char_index;
        while end < length && is_word(end) {
            end += 1;
        }
        start..end
    } else if model
        .char_at(char_index)
        .is_some_and(|character| !matches!(character, '\r' | '\n'))
    {
        char_index..char_index + 1
    } else {
        char_index..char_index
    };
    model.set_selections(vec![TextSelection {
        anchor: range.start,
        cursor: range.end,
    }]);
}

/// Rows one PageUp or PageDown moves: the whole rows the viewport shows.
fn page_rows(session: &VirtualEditorSession, ui: &Ui, style: &VirtualEditorStyle<'_>) -> usize {
    let height = session
        .viewport_size
        .map_or_else(|| ui.available_height(), |size| size.y);
    ((height - style.top_padding).max(0.0) / style.line_height).floor() as usize
}

fn content_size(
    session: &VirtualEditorSession,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) -> Vec2 {
    let visible_rows = session.model.visible_line_count().max(1);
    Vec2::new(
        style.gutter_width
            + style.code_left_padding
            + session.maximum_columns as f32 * char_width
            + RIGHT_PADDING,
        style.top_padding + visible_rows as f32 * style.line_height + BOTTOM_PADDING,
    )
}

/// The primary caret's cell in content coordinates, or `None` when its line
/// is folded away.
fn caret_content_rect(
    session: &VirtualEditorSession,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
) -> Option<egui::Rect> {
    let cursor = session.model.selections().last()?.cursor;
    let (line, column) = session.model.line_column_for_char(cursor);
    let row = session.model.visible_row_for_logical_line(line)?;
    Some(egui::Rect::from_min_size(
        egui::pos2(
            style.gutter_width + style.code_left_padding + column as f32 * char_width,
            style.top_padding + row as f32 * style.line_height,
        ),
        Vec2::new(char_width, style.line_height),
    ))
}

/// The scroll offset this frame should start from, when a key or a command
/// moved the caret or paged the view. `None` leaves the scroll area where it
/// was. The offset is set before the view is laid out, so the caret is never
/// drawn outside it.
fn planned_scroll_offset(
    ctx: &egui::Context,
    session: &VirtualEditorSession,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
    reveal: Option<Reveal>,
    page_rows: isize,
) -> Option<Vec2> {
    if reveal.is_none() && page_rows == 0 {
        return None;
    }
    let viewport = session.viewport_size?;
    let mut offset = egui::scroll_area::State::load(ctx, session.scroll_id?)?.offset;
    offset.y += page_rows as f32 * style.line_height;
    if let Some(reveal) = reveal
        && let Some(caret) = caret_content_rect(session, style, char_width)
    {
        offset = revealed_offset(offset, viewport, caret, style, char_width, reveal);
    }
    let maximum = (content_size(session, style, char_width) - viewport).max(Vec2::ZERO);
    Some(offset.clamp(Vec2::ZERO, maximum))
}

/// `offset` moved as little as brings `caret` a margin inside the viewport,
/// or centred on it for [`Reveal::CenterIfOutside`]. The sticky gutter hides
/// the viewport's left edge, so the code's visible span starts after it.
fn revealed_offset(
    mut offset: Vec2,
    viewport: Vec2,
    caret: egui::Rect,
    style: &VirtualEditorStyle<'_>,
    char_width: f32,
    reveal: Reveal,
) -> Vec2 {
    let code_left = style.gutter_width + style.code_left_padding;
    let code_width = (viewport.x - code_left).max(char_width);
    let margin_x = (REVEAL_MARGIN_COLUMNS * char_width).min(code_width * 0.25);
    let margin_y = (REVEAL_MARGIN_ROWS * style.line_height)
        .min(((viewport.y - style.line_height) * 0.5).max(0.0));
    let outside_vertically = caret.top() < offset.y || caret.bottom() > offset.y + viewport.y;
    match reveal {
        Reveal::CenterIfOutside if outside_vertically => {
            offset.y = caret.center().y - viewport.y * 0.5;
        }
        Reveal::CenterIfOutside => {}
        Reveal::Nearest => {
            if caret.top() - margin_y < offset.y {
                offset.y = caret.top() - margin_y;
            } else if caret.bottom() + margin_y > offset.y + viewport.y {
                offset.y = caret.bottom() + margin_y - viewport.y;
            }
        }
    }
    if caret.left() - margin_x < offset.x + code_left {
        offset.x = caret.left() - margin_x - code_left;
    } else if caret.right() + margin_x > offset.x + viewport.x {
        offset.x = caret.right() + margin_x - viewport.x;
    }
    offset
}

/// Claim the keys this editor acts on from egui's focus navigation for the
/// next frame: arrows and Escape always, and Tab while it indents.
fn claim_editing_keys(ui: &Ui, editor_id: egui::Id, claim_tab: bool) {
    ui.memory_mut(|memory| {
        memory.set_focus_lock_filter(
            editor_id,
            egui::EventFilter {
                tab: claim_tab,
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: true,
            },
        );
    });
}

/// Whether Tab moves focus out of source editors instead of indenting. One
/// switch for every editor in the window, as a reader who turns it on to
/// leave one expects the next one to let them leave too.
fn tab_moves_focus_mode(ctx: &egui::Context) -> bool {
    ctx.data(|data| data.get_temp::<bool>(egui::Id::new(TAB_MOVES_FOCUS_ID)))
        .unwrap_or(false)
}

const TAB_MOVES_FOCUS_ID: &str = "source-editor-tab-moves-focus";

fn set_tab_moves_focus(ctx: &egui::Context, editor_id: egui::Id, enabled: bool) {
    let now = ctx.input(|input| input.time);
    ctx.data_mut(|data| {
        data.insert_temp(egui::Id::new(TAB_MOVES_FOCUS_ID), enabled);
        data.insert_temp(editor_id.with("tab-mode-announced-at"), now);
    });
}

/// Read the new Tab mode out once it changes, without taking focus: a live
/// status the editor keeps for a few seconds after the switch.
fn announce_tab_mode_change(
    ui: &Ui,
    editor_id: egui::Id,
    response: &Response,
    tab_moves_focus: bool,
    style: &VirtualEditorStyle<'_>,
) {
    let Some(announced_at) = ui
        .ctx()
        .data(|data| data.get_temp::<f64>(editor_id.with("tab-mode-announced-at")))
    else {
        return;
    };
    if ui.input(|input| input.time) - announced_at > TAB_MODE_ANNOUNCEMENT_SECONDS {
        return;
    }
    let status = ui.interact(
        egui::Rect::from_min_size(response.rect.min, Vec2::splat(1.0)),
        editor_id.with("tab-mode-status"),
        Sense::hover(),
    );
    let text = tab_mode_sentence(style, ui.ctx().os(), tab_moves_focus);
    ui.ctx().accesskit_node_builder(status.id, |node| {
        node.set_role(egui::accesskit::Role::Status);
        node.set_live(egui::accesskit::Live::Polite);
        node.set_label(text);
    });
}

/// What Tab does in source editors now, and the chord that switches it.
fn tab_mode_sentence(
    style: &VirtualEditorStyle<'_>,
    os: egui::os::OperatingSystem,
    tab_moves_focus: bool,
) -> String {
    style.messages.format(
        if tab_moves_focus {
            crate::workbench::MessageId::EditorAccessibleTabMovesFocus
        } else {
            crate::workbench::MessageId::EditorAccessibleTabIndents
        },
        &[("shortcut", keymap::tab_moves_focus_chord(os))],
    )
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
    keys: &KeyContext,
    outcome: &mut InputOutcome,
) {
    let editable = keys.editable;
    match event {
        egui::Event::Copy => copy_selection(ui, &session.model),
        egui::Event::Cut if editable => {
            copy_selection(ui, &session.model);
            session.model.insert_at_selections("");
            outcome.caret_moved = true;
        }
        egui::Event::Paste(text) if editable && !text.is_empty() => {
            let text = normalize_newlines(&text, session.model.preferred_newline());
            grow_maximum_columns_for_insertion(session, &text);
            session.model.insert_at_selections(&text);
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        egui::Event::Text(text)
            if editable && !text.is_empty() && !text.chars().any(char::is_control) =>
        {
            grow_maximum_columns_for_insertion(session, &text);
            session.model.insert_at_selections(&text);
            if keys.syntax == EditorSyntax::VerilogA {
                dedent_after_typing(&mut session.model, &text);
            }
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        egui::Event::Key {
            key,
            pressed: true,
            modifiers,
            ..
        } => {
            if let Some(action) = keymap::key_action(keys.os, key, modifiers) {
                session.touch.show_handles(false);
                apply_key_action(ui, session, action, keys, outcome);
            }
        }
        egui::Event::Ime(event) if editable => {
            handle_ime(session, event);
            outcome.caret_moved = true;
        }
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

fn apply_key_action(
    ui: &Ui,
    session: &mut VirtualEditorSession,
    action: KeyAction,
    keys: &KeyContext,
    outcome: &mut InputOutcome,
) {
    let editable = keys.editable;
    match action {
        KeyAction::SelectAll => {
            session.model.set_selections(vec![TextSelection {
                anchor: 0,
                cursor: session.model.len_chars(),
            }]);
        }
        KeyAction::Undo | KeyAction::Redo if editable => {
            step_history(
                session,
                if action == KeyAction::Undo {
                    StandardCommand::Undo
                } else {
                    StandardCommand::Redo
                },
            );
            outcome.caret_moved = true;
        }
        KeyAction::Copy => copy_selection(ui, &session.model),
        KeyAction::Cut if editable => {
            copy_selection(ui, &session.model);
            session.model.insert_at_selections("");
            outcome.caret_moved = true;
        }
        KeyAction::Move { motion, extend } => {
            let rows = keys.page_rows.max(1) as isize;
            match motion {
                Motion::Backward(reach) | Motion::Forward(reach) => {
                    let forward = matches!(motion, Motion::Forward(_));
                    match reach {
                        Reach::Character => {
                            move_horizontal(&mut session.model, forward, extend, false);
                        }
                        Reach::Word => move_horizontal(&mut session.model, forward, extend, true),
                        Reach::LineEdge => {
                            move_line_edge(&mut session.model, forward, extend, false);
                        }
                    }
                    session.preferred_column = None;
                }
                Motion::Up => move_vertical(session, -1, extend),
                Motion::Down => move_vertical(session, 1, extend),
                Motion::PageUp => {
                    move_vertical(session, -rows, extend);
                    outcome.page_rows -= rows;
                }
                Motion::PageDown => {
                    move_vertical(session, rows, extend);
                    outcome.page_rows += rows;
                }
                Motion::DocumentStart | Motion::DocumentEnd => {
                    let end = motion == Motion::DocumentEnd;
                    move_line_edge(&mut session.model, end, extend, true);
                    session.preferred_column = None;
                }
            }
            outcome.caret_moved = true;
        }
        KeyAction::Delete { forward, reach } if editable => {
            match (reach, forward) {
                (Reach::Character, false) => {
                    session.model.delete_backward();
                }
                (Reach::Character, true) => {
                    session.model.delete_forward();
                }
                (Reach::Word | Reach::LineEdge, _) => {
                    delete_to(&mut session.model, forward, reach);
                }
            }
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        KeyAction::Newline if editable => {
            if keys.syntax == EditorSyntax::VerilogA {
                dedent_closing_word_at_caret(&mut session.model);
            }
            insert_smart_newline(&mut session.model, keys.syntax);
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        KeyAction::Indent if editable && !keys.tab_moves_focus => {
            indent_at_selections(&mut session.model, keys.syntax);
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        KeyAction::Outdent if editable && !keys.tab_moves_focus => {
            apply_model_structural(&mut session.model, keys.syntax, StructuralCommand::Outdent);
            session.preferred_column = None;
            outcome.caret_moved = true;
        }
        KeyAction::Escape => {
            // Escape takes back the last thing a press added: extra carets,
            // then the selection. With nothing left to collapse it hands
            // the keyboard back, as it does everywhere else.
            let selections = session.model.selections();
            if selections.len() > 1 {
                let primary = selections
                    .last()
                    .copied()
                    .unwrap_or(TextSelection::caret(0));
                session.model.set_selections(vec![primary]);
            } else if let Some(primary) = selections.last().copied()
                && !primary.range().is_empty()
            {
                session
                    .model
                    .set_selections(vec![TextSelection::caret(primary.cursor)]);
            } else {
                outcome.release_focus = true;
            }
        }
        KeyAction::ToggleTabMovesFocus => {
            outcome.toggle_tab_moves_focus = !outcome.toggle_tab_moves_focus;
        }
        _ => {}
    }
}

/// Delete from each caret to the word boundary or the line edge in the
/// direction asked. A caret already on the line edge deletes the line break
/// instead, so the key keeps working across lines. Selections delete
/// themselves.
fn delete_to(model: &mut TextDocumentModel, forward: bool, reach: Reach) {
    let length = model.len_chars();
    let selections = model
        .selections()
        .iter()
        .copied()
        .map(|selection| {
            if !selection.range().is_empty() {
                return selection;
            }
            let cursor = selection.cursor;
            let line = model.line_column_for_char(cursor).0;
            let line_range = model
                .line_content_char_range(line)
                .unwrap_or(cursor..cursor);
            let target = if forward {
                if cursor >= line_range.end {
                    line_break_end(model, cursor, length)
                } else if reach == Reach::Word {
                    next_boundary(model, cursor, length, true).min(line_range.end)
                } else {
                    line_range.end
                }
            } else if cursor <= line_range.start {
                line_break_start(model, cursor)
            } else if reach == Reach::Word {
                previous_boundary(model, cursor, true).max(line_range.start)
            } else {
                line_range.start
            };
            TextSelection {
                anchor: target,
                cursor,
            }
        })
        .collect();
    model.set_selections(selections);
    model.insert_at_selections("");
}

/// The end of the line break that starts at `cursor`: past `\r\n` whole.
fn line_break_end(model: &TextDocumentModel, cursor: usize, length: usize) -> usize {
    if cursor >= length {
        return cursor;
    }
    if model.char_at(cursor) == Some('\r') && model.char_at(cursor + 1) == Some('\n') {
        cursor + 2
    } else {
        cursor + 1
    }
}

/// The start of the line break that ends at `cursor`: before `\r\n` whole.
fn line_break_start(model: &TextDocumentModel, cursor: usize) -> usize {
    if cursor == 0 {
        return 0;
    }
    if cursor >= 2
        && model.char_at(cursor - 1) == Some('\n')
        && model.char_at(cursor - 2) == Some('\r')
    {
        cursor - 2
    } else {
        cursor - 1
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
        StandardCommand::Undo | StandardCommand::Redo if editable => {
            step_history(session, command);
        }
        StandardCommand::Copy => copy_selection(ui, &session.model),
        StandardCommand::Cut if editable => {
            copy_selection(ui, &session.model);
            session.model.insert_at_selections("");
        }
        StandardCommand::Paste if editable => ui
            .ctx()
            .send_viewport_cmd(egui::ViewportCommand::RequestPaste),
        StandardCommand::Delete if editable => {
            session.model.delete_forward();
            session.preferred_column = None;
        }
        StandardCommand::SelectAll => session.model.set_selections(vec![TextSelection {
            anchor: 0,
            cursor: session.model.len_chars(),
        }]),
        _ => {}
    }
    // Asking again while focused would drop the key filter and interrupt a
    // composition.
    if !ui.memory(|memory| memory.has_focus(editor_id)) {
        ui.ctx()
            .memory_mut(|memory| memory.request_focus(editor_id));
    }
}

/// Undo or redo one step of the editor's own history, or say that it had
/// none so the owner can step its own.
fn step_history(session: &mut VirtualEditorSession, command: StandardCommand) {
    let stepped = if command == StandardCommand::Undo {
        session.model.undo()
    } else {
        session.model.redo()
    };
    if !stepped {
        session.unanswered_history = Some(command);
    }
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
    let mapped = if matches!(
        command,
        StructuralCommand::Indent | StructuralCommand::Outdent
    ) {
        selections_after_reindent(model, &source)
    } else {
        mapped_byte_selections
            .into_iter()
            .map(|selection| TextSelection {
                anchor: byte_to_char_index(&source, selection.anchor),
                cursor: byte_to_char_index(&source, selection.cursor),
            })
            .collect::<Vec<_>>()
    };
    if any_changed {
        model.replace_source_transaction(&source, mapped)
    } else {
        model.set_selections(mapped);
        false
    }
}

/// Tab in the source: while every selection lies on one line, indentation
/// at each caret up to the next indent stop, replacing what is selected;
/// once a selection spans lines, those whole lines indent.
fn indent_at_selections(model: &mut TextDocumentModel, syntax: EditorSyntax) {
    let spans_lines = model.selections().iter().any(|selection| {
        let range = selection.range();
        model.line_column_for_char(range.start).0 != model.line_column_for_char(range.end).0
    });
    if spans_lines {
        apply_model_structural(model, syntax, StructuralCommand::Indent);
        return;
    }
    let stop = syntax.indentation().chars().count().max(1);
    let padding = model
        .selections()
        .iter()
        .map(|selection| {
            let column = model.line_column_for_char(selection.range().start).1;
            " ".repeat(stop - column % stop)
        })
        .collect::<Vec<_>>();
    model.replace_at_selections(&padding);
}

/// The model's selections carried across a change of leading indentation to
/// `source`, which has the same lines. Each end keeps its line and the text
/// it sits before: a caret moves with its text, and a selection that starts
/// at a line's start keeps the indentation inside it.
fn selections_after_reindent(model: &TextDocumentModel, source: &str) -> Vec<TextSelection> {
    let mut starts = vec![0];
    let mut lengths = Vec::new();
    let mut length = 0;
    for (index, character) in source.chars().enumerate() {
        if character == '\n' {
            lengths.push(length);
            starts.push(index + 1);
            length = 0;
        } else if character != '\r' {
            length += 1;
        }
    }
    lengths.push(length);
    let carry = |index: usize, keep_line_start: bool| {
        let (line, column) = model.line_column_for_char(index);
        let (Some(&start), Some(&new_length)) = (starts.get(line), lengths.get(line)) else {
            return index;
        };
        let old_length = model
            .line_content_char_range(line)
            .map_or(new_length, |range| range.len());
        let column = if new_length >= old_length {
            if column == 0 && keep_line_start {
                0
            } else {
                column + (new_length - old_length)
            }
        } else {
            column.saturating_sub(old_length - new_length)
        };
        start + column.min(new_length)
    };
    model
        .selections()
        .iter()
        .map(|selection| {
            let range = selection.range();
            let selected = !range.is_empty();
            let start = carry(range.start, selected);
            let end = carry(range.end, false);
            if selection.anchor <= selection.cursor {
                TextSelection {
                    anchor: start,
                    cursor: end,
                }
            } else {
                TextSelection {
                    anchor: end,
                    cursor: start,
                }
            }
        })
        .collect()
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
                EditorSyntax::VerilogA => leaves_block_open(trimmed),
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

/// Move every caret `delta` rows on screen, so a folded region counts as the
/// one row it shows.
fn move_vertical(session: &mut VirtualEditorSession, delta: isize, extend: bool) {
    let selections = session.model.selections().to_vec();
    let mut moved = Vec::with_capacity(selections.len());
    let last_row = session.model.visible_line_count().saturating_sub(1);
    for selection in selections {
        let (line, column) = session.model.line_column_for_char(selection.cursor);
        let preferred = *session.preferred_column.get_or_insert(column);
        let target_line = session
            .model
            .visible_row_for_logical_line(line)
            .and_then(|row| {
                session
                    .model
                    .logical_line_for_visible_row(row.saturating_add_signed(delta).min(last_row))
            })
            .unwrap_or_else(|| {
                line.saturating_add_signed(delta)
                    .min(session.model.line_count().saturating_sub(1))
            });
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

/// Fold the selected lines. A caret, or a selection within one line, folds
/// the innermost block around its line that is not folded already.
fn fold_selection(session: &mut VirtualEditorSession, syntax: EditorSyntax) {
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
    if end > start + 1 {
        session.model.fold_lines(start, end);
        return;
    }
    let mut around = discover_fold_regions(&session.model, syntax)
        .into_iter()
        .filter(|(first, last)| *first <= start && start < *last)
        .collect::<Vec<_>>();
    around.sort_unstable_by_key(|&(first, last)| (std::cmp::Reverse(first), last));
    for (first, last) in around {
        if session.model.fold_lines(first, last) {
            return;
        }
    }
}

fn handle_view_command(
    session: &mut VirtualEditorSession,
    syntax: EditorSyntax,
    command: EditorViewCommand,
) {
    match command {
        EditorViewCommand::FoldSelection => fold_selection(session, syntax),
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
        EditorSyntax::VerilogA => veriloga_fold_regions(model),
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

/// Underline the stretch `mark` covers, along `underline`: an error with a
/// zigzag, a warning with a solid rule, and a note or a hint with a dotted
/// one, so the shape says the severity as well as the ink does.
fn paint_source_mark(
    painter: &egui::Painter,
    mark: &SourceMark,
    code_origin_x: f32,
    underline: f32,
    char_width: f32,
    style: &VirtualEditorStyle<'_>,
) {
    use rspice_vocabulary::diagnostics::DiagnosticSeverity;

    /// Half a zigzag's period and a dotted rule's pitch.
    const STEP: f32 = 2.0;
    let start = mark.columns.start;
    let end = mark.columns.end.max(start + 1);
    let clip = painter.clip_rect();
    let left = (code_origin_x + start as f32 * char_width).max(clip.left() - STEP);
    let right = (code_origin_x + end as f32 * char_width).min(clip.right() + STEP);
    if left >= right {
        return;
    }
    match mark.severity {
        DiagnosticSeverity::Error => {
            // The zigzag keeps its phase to the text, so it does not crawl
            // as the row scrolls sideways.
            let mut step = ((left - code_origin_x) / STEP).floor();
            let mut points = Vec::new();
            loop {
                let x = code_origin_x + step * STEP;
                let y = if step.rem_euclid(2.0) < 1.0 {
                    underline + 1.0
                } else {
                    underline - 1.0
                };
                points.push(egui::pos2(x.clamp(left, right), y));
                if x >= right {
                    break;
                }
                step += 1.0;
            }
            painter.add(egui::Shape::line(points, Stroke::new(1.0, style.error)));
        }
        DiagnosticSeverity::Warning => {
            painter.hline(left..=right, underline, Stroke::new(1.5, style.warning));
        }
        DiagnosticSeverity::Info | DiagnosticSeverity::Hint => {
            let mut x = code_origin_x + ((left - code_origin_x) / STEP).ceil() * STEP;
            while x <= right {
                painter.circle_filled(egui::pos2(x, underline), 0.7, style.text_dim);
                x += STEP * 1.5;
            }
        }
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

fn paint_carets_and_ime(
    ui: &Ui,
    session: &VirtualEditorSession,
    focused: bool,
    editable: bool,
    origin: egui::Pos2,
    code_origin_x: f32,
    char_width: f32,
    style: &VirtualEditorStyle<'_>,
) {
    if !focused {
        return;
    }
    // Reduced motion holds the caret steady; otherwise it blinks from the
    // moment it last moved.
    let blinking = ui.style().animation_time > 0.0;
    let since_moved = (ui.input(|input| input.time) - session.caret_moved_at).max(0.0);
    let caret_visible = !blinking || (since_moved * 2.0).fract() < 0.62;
    let mut primary_caret_rect = None;
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
        if caret_visible {
            ui.painter().rect_filled(rect, 0.0, style.text);
        }
        primary_caret_rect = Some(rect);
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
    // Text input is wanted on every frame an editable view holds focus,
    // whatever the blink shows: a frame without it ends a composition and
    // drops a tablet's keyboard. A caret scrolled or folded out of view
    // anchors the candidate window to the nearest edge of the view instead.
    if editable {
        let clip = ui.clip_rect();
        let cursor_rect = primary_caret_rect.map_or_else(
            || egui::Rect::from_min_size(clip.min, Vec2::new(1.25, style.line_height - 2.0)),
            |rect| {
                let min = rect
                    .min
                    .clamp(clip.min, (clip.max - rect.size()).max(clip.min));
                egui::Rect::from_min_size(min, rect.size())
            },
        );
        let to_global = ui
            .ctx()
            .layer_transform_to_global(ui.layer_id())
            .unwrap_or_default();
        ui.output_mut(|output| {
            output.ime = Some(egui::output::IMEOutput {
                rect: to_global * clip,
                cursor_rect: to_global * cursor_rect,
                should_interrupt_composition: false,
            });
        });
    }
    if blinking {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(250));
    }
}

/// A small mark in the view's lower right while Tab moves focus, so a
/// sighted keyboard user can tell why Tab no longer indents.
fn paint_tab_moves_focus_badge(
    ui: &Ui,
    viewport: egui::Rect,
    origin: egui::Pos2,
    style: &VirtualEditorStyle<'_>,
) {
    let text = style
        .messages
        .text(crate::workbench::MessageId::EditorTabMovesFocusBadge);
    let font = FontId::proportional(style.font.size * 0.9);
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(text, font, style.text_dim));
    let inset = Vec2::new(14.0, 14.0);
    let size = galley.size() + Vec2::new(12.0, 6.0);
    let bottom_right = origin + viewport.max.to_vec2() - inset;
    let rect = egui::Rect::from_min_max(bottom_right - size, bottom_right);
    let painter = ui.painter();
    painter.rect(
        rect,
        3.0,
        style.background,
        Stroke::new(1.0, style.border),
        egui::StrokeKind::Inside,
    );
    painter.galley(rect.min + Vec2::new(6.0, 3.0), galley, style.text_dim);
}

#[cfg(test)]
mod input_tests;
#[cfg(test)]
mod tests;
