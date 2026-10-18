//! The Console page: one channel's log, read the way a simulator's log is.
//!
//! A row is three columns: the severity mark, the time on the reader's clock
//! and the message, which wraps at the column rather than cutting a path
//! short. The log is one keyboard stop. The arrows move the selection inside
//! it, and the reader copies, saves or follows what is selected.
//!
//! Only the rows in view are laid out ([`lines`] keeps the rest as heights),
//! and the body follows new lines only while the reader is at its end.

mod lines;
mod toolbar;

pub(super) use toolbar::toolbar;

use std::sync::Arc;

use egui::{Align, Align2, Key, Modifiers, Rect, ScrollArea, Sense, Ui, pos2, vec2};

use super::log_anchor;
use super::{
    CONSOLE_COLUMN_GAP, CONSOLE_FONT_SIZE, CONSOLE_MARK_GUTTER, CONSOLE_ROW_MIN_HEIGHT,
    log_message_color, log_problem, muted, paint_row_mark,
};
use crate::diagnostics::{LogAnchor, LogSeverity};
use crate::io::file_exchange::{self, FileKind};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::workbench::AppState;
use crate::workbench::app_state::session::shortcuts::report_escape_owner;
use crate::workbench::state::{ConsoleChannel, ConsoleReveal, ConsoleSelection, ConsoleView};
use lines::{Line, Metrics, Rows, Source};

/// The widest time the clock column prints, measured in the row's face to
/// size the column.
const CLOCK_WIDEST: &str = "00:00:00";

/// What a saved log is offered as.
const LOG_FILE: FileKind = FileKind {
    label: "Log file",
    extensions: &["log"],
    subject: "The log",
    fallback_name: "console.log",
};

/// The identity of the log's one keyboard stop.
pub(super) fn body_id() -> egui::Id {
    egui::Id::new("workbench.console.log")
}

/// The identity of the find field.
pub(super) fn find_id() -> egui::Id {
    egui::Id::new("workbench.console.find")
}

fn save_id() -> egui::Id {
    egui::Id::new("workbench.console.save")
}

fn row_font() -> egui::FontId {
    theme::mono(CONSOLE_FONT_SIZE, FontWeight::Regular)
}

/// The log `view` reads, falling back to the session's when the run it was
/// reading is no longer held.
fn source<'a>(state: &'a AppState, view: &mut ConsoleView) -> Source<'a> {
    Source::of(
        &state.log_buffer,
        &state.simulation.retained.runs,
        view.channel,
    )
    .unwrap_or_else(|| {
        view.open(ConsoleChannel::Session);
        Source::Session(&state.log_buffer)
    })
}

/// What the page asked for that it cannot do while it draws from the session
/// it would change.
#[derive(Default)]
struct Intent {
    follow: Option<LogAnchor>,
    clipboard: Option<String>,
    /// A file name and the text to save under it.
    save: Option<(String, String)>,
}

pub(super) fn console(ui: &mut Ui, state: &mut AppState) {
    let mut view = std::mem::take(&mut state.workbench.console_view);
    let intent = body(ui, state, &mut view);
    state.workbench.console_view = view;
    if let Some(text) = intent.clipboard {
        ui.ctx().copy_text(text);
    }
    if let Some((name, text)) = intent.save
        && let Err(error) =
            file_exchange::save_file(ui.ctx(), save_id(), LOG_FILE, name, text.into_bytes())
    {
        state.prompt(error);
    }
    if let Some(anchor) = intent.follow {
        state.jump_to_log_anchor(anchor);
    }
}

/// Say how a save the page started went, once the picker has answered.
pub(super) fn collect_save(ctx: &egui::Context, state: &mut AppState) {
    let Some(outcome) = file_exchange::take_saved(ctx, save_id()) else {
        return;
    };
    match outcome {
        Ok(Some(file)) => state.prompt(format!("Saved {}.", file.name)),
        Ok(None) => {}
        Err(error) => state
            .ui
            .toasts
            .error_with_title(ctx, "Log not saved", error),
    }
}

/// Where a row's columns stand, measured from the face they are set in.
#[derive(Debug, Clone, Copy)]
struct Columns {
    /// Where the message starts.
    message: f32,
    width: f32,
    metrics: Metrics,
}

impl Columns {
    fn measure(ui: &Ui, width: f32) -> Self {
        let clock = ui.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                CLOCK_WIDEST.to_owned(),
                row_font(),
                egui::Color32::PLACEHOLDER,
            )
        });
        let time = clock.size().x.ceil();
        let message = CONSOLE_MARK_GUTTER + time + CONSOLE_COLUMN_GAP;
        Self {
            message,
            width,
            metrics: Metrics {
                wrap: (width - message).max(time),
                advance: clock.size().x / CLOCK_WIDEST.len() as f32,
                line_height: clock.size().y,
                row_height: CONSOLE_ROW_MIN_HEIGHT,
            },
        }
    }
}

fn body(ui: &mut Ui, state: &AppState, view: &mut ConsoleView) -> Intent {
    let mut intent = Intent::default();
    let source = source(state, view);
    let mut rows = Rows::take(ui.ctx());
    rows.sync(source, view);
    // The bar's width is kept whether or not it shows, so a log that grows
    // past the body does not rewrap every row the frame its bar appears.
    let width = (ui.available_width() - ui.spacing().scroll.allocated_width()).max(1.0);
    let columns = Columns::measure(ui, width);
    rows.sync_heights(source, columns.metrics);
    if rows.is_empty() {
        muted(ui, &empty_state(source, view, &rows));
        rows.store(ui.ctx());
        return intent;
    }
    keyboard(ui, state, source, &rows, view, &mut intent);

    let tokens = Tokens::get(ui.ctx());
    let notice = eviction_notice(source).map(|text| {
        ui.painter().layout(
            text,
            row_font(),
            tokens.color.text_faint,
            columns.metrics.wrap,
        )
    });
    let lead = notice.as_ref().map_or(0.0, |galley| {
        columns.metrics.row_height_for(galley.size().y)
    });
    let reveal = view.reveal.take();
    let mut pressed = false;
    let mut left = false;
    let mut cursor = None;
    let mut remeasured = false;
    let output = ScrollArea::vertical()
        .id_salt(("workbench.console.body", view.channel))
        .auto_shrink(false)
        // egui otherwise keeps a scrolled area at least 64 points tall, past
        // the bottom edge of a dock dragged shorter than that.
        .min_scrolled_height(0.0)
        .stick_to_bottom(true)
        .show_viewport(ui, |ui, visible| {
            ui.set_height(lead + rows.height());
            let origin = ui.max_rect().min;
            let span = |top: f32, bottom: f32| {
                Rect::from_min_max(
                    pos2(origin.x, origin.y + lead + top),
                    pos2(origin.x + columns.width, origin.y + lead + bottom),
                )
            };
            match reveal {
                Some(ConsoleReveal::Line(key)) => {
                    if let Ok(index) = rows.position(key) {
                        ui.scroll_to_rect(span(rows.top(index), rows.bottom(index)), None);
                    }
                }
                Some(ConsoleReveal::End) => {
                    ui.scroll_to_rect(span(rows.height(), rows.height()), Some(Align::BOTTOM));
                }
                None => {}
            }
            if let Some(galley) = notice {
                ui.painter().galley(
                    pos2(
                        origin.x + columns.message,
                        origin.y + columns.metrics.inset(),
                    ),
                    galley,
                    tokens.color.text_faint,
                );
            }
            let range = rows.between(visible.min.y - lead, visible.max.y - lead);
            // Rows are set end to end from the first one in view, each at the
            // height it lays out at this frame, so a row taller than its
            // estimate pushes the next one down rather than painting over it.
            let mut top = origin.y + lead + rows.top(range.start);
            for index in range {
                let Some(line) = source.line(rows.keys()[index]) else {
                    continue;
                };
                let galley = ui.painter().layout(
                    line.message.to_owned(),
                    row_font(),
                    log_message_color(&tokens, line.severity),
                    columns.metrics.wrap,
                );
                let height = columns.metrics.row_height_for(galley.size().y);
                remeasured |= rows.measure(index, height);
                let rect = Rect::from_min_size(pos2(origin.x, top), vec2(columns.width, height));
                top += height;
                let outcome = row(
                    ui,
                    &RowContext {
                        state,
                        source,
                        rows: &rows,
                        columns: &columns,
                    },
                    &line,
                    galley,
                    rect,
                    view,
                    &mut intent,
                );
                pressed |= outcome.pressed;
                left |= outcome.left;
                if outcome.cursor {
                    cursor = Some((outcome.id, rect));
                }
            }
        });
    if remeasured {
        ui.ctx().request_repaint();
    }
    let response = focus_stop(ui, output.inner_rect, cursor);
    if left {
        response.surrender_focus();
    } else if pressed && !response.has_focus() {
        response.request_focus();
    }
    // After the press, so the frame a click gave the log focus records it: an
    // Escape on the very next frame is still the log's.
    report_escape_owner(&response);
    rows.store(ui.ctx());
    intent
}

/// The line a channel shows instead of rows.
fn empty_state(source: Source<'_>, view: &ConsoleView, rows: &Rows) -> String {
    if source.len() == 0 {
        return match source {
            Source::Session(_) => "No studio activity yet.",
            Source::Run(_) => "This run wrote no log.",
        }
        .to_owned();
    }
    if !view.find.is_empty() && rows.found() == 0 {
        return format!("No line contains '{}'.", view.find);
    }
    "The severity filters hide every line.".to_owned()
}

/// The first row of a session whose ring has dropped its oldest entries.
fn eviction_notice(source: Source<'_>) -> Option<String> {
    let Source::Session(log) = source else {
        return None;
    };
    (log.evicted() > 0).then(|| {
        format!(
            "Earlier entries were removed (the Console keeps the latest {})",
            super::super::navigator::grouped_count(log.capacity())
        )
    })
}

/// The log's one keyboard stop, laid over the rows it moves through.
///
/// One stop rather than one per row: Tab passes the log in one press, and the
/// arrows are for moving inside it. `cursor` is the row the selection was last
/// moved to, where it is drawn: it is announced as the list's active item and
/// carries the ring that Tab shows.
fn focus_stop(ui: &mut Ui, rect: Rect, cursor: Option<(egui::Id, Rect)>) -> egui::Response {
    let id = body_id();
    let response = ui.interact(rect, id, Sense::FOCUSABLE);
    // The log owns the vertical arrows and Escape while it holds focus;
    // without this egui takes them first, as moves to the widget above or
    // below and as a release of focus. Tab is left to egui: it is how a
    // reader leaves the log.
    if response.has_focus() {
        ui.memory_mut(|memory| {
            memory.set_focus_lock_filter(
                id,
                egui::EventFilter {
                    tab: false,
                    horizontal_arrows: false,
                    vertical_arrows: true,
                    escape: true,
                },
            );
        });
    }
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Other, ui.is_enabled(), "Console log")
    });
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::ListBox);
        if let Some((row, _)) = cursor {
            node.set_active_descendant(row.accesskit_id());
        }
    });
    // Focus a pointer press gave frames nothing; focus Tab gave frames the
    // row the keys move from, or the log while none is selected.
    if theme::focus_is_visible(ui.ctx(), &response) {
        let framed = cursor
            .map(|(_, row)| row.intersect(rect))
            .filter(|row| row.is_positive())
            .unwrap_or(rect);
        theme::paint_focus_ring(ui, &response, framed);
    }
    response
}

/// What a row reads beyond its own line.
struct RowContext<'a, 'b> {
    state: &'a AppState,
    source: Source<'a>,
    rows: &'b Rows,
    columns: &'b Columns,
}

/// What a press on a row did.
struct RowOutcome {
    id: egui::Id,
    /// The row was pressed, and the log takes the keyboard.
    pressed: bool,
    /// The row's anchor was followed off the page, and the log lets go of it.
    left: bool,
    /// The row is the one the selection was last moved to.
    cursor: bool,
}

fn row(
    ui: &mut Ui,
    context: &RowContext<'_, '_>,
    line: &Line<'_>,
    galley: Arc<egui::Galley>,
    rect: Rect,
    view: &mut ConsoleView,
    intent: &mut Intent,
) -> RowOutcome {
    let tokens = Tokens::get(ui.ctx());
    let id = egui::Id::new(("workbench.console.row", view.channel, line.key));
    // Every row senses a press, because every row can be selected, and none
    // takes focus: the log is one keyboard stop.
    let response = ui.interact(rect, id, Sense::CLICK);
    let selected = view
        .selection
        .is_some_and(|selection| selection.contains(line.key));
    let time = clock(line.unix_ms);
    let announcement = format!("{}, {time}, {}", severity_word(line.severity), line.message);
    // Declared as a selectable label so the line's words are its name, then
    // given the role a list option has.
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            announcement.clone(),
        )
    });
    ui.ctx().accesskit_node_builder(id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_selected(selected);
    });
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, tokens.color.accent.linear_multiply(0.14));
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, tokens.color.bg_hover);
    }
    paint_row_mark(ui, rect.left(), rect.top(), log_problem(line.severity));
    ui.painter().text(
        pos2(
            rect.left() + CONSOLE_MARK_GUTTER,
            rect.top() + context.columns.metrics.row_height * 0.5,
        ),
        Align2::LEFT_CENTER,
        time,
        row_font(),
        tokens.color.text_faint,
    );
    ui.painter().galley(
        pos2(
            rect.left() + context.columns.message,
            rect.top() + context.columns.metrics.inset(),
        ),
        galley,
        log_message_color(&tokens, line.severity),
    );

    let mut outcome = RowOutcome {
        id,
        pressed: false,
        left: false,
        cursor: false,
    };
    if response.clicked() {
        let extend = ui.input(|input| input.modifiers.shift);
        view.selection = Some(match view.selection {
            Some(selection) if extend => ConsoleSelection {
                anchor: selection.anchor,
                cursor: line.key,
            },
            _ => ConsoleSelection::single(line.key),
        });
        outcome.pressed = true;
    }
    // A menu acts on the selection, so a row opened outside it is selected
    // first.
    if response.secondary_clicked() && !selected {
        view.selection = Some(ConsoleSelection::single(line.key));
        outcome.pressed = true;
    }
    if response.double_clicked()
        && let Some(anchor) = line.anchor
    {
        outcome.left = log_anchor::follow(context.state, view, anchor, &mut intent.follow);
    }
    response.context_menu(|ui| {
        if ui.button("Copy").clicked() {
            let selection = view.selection;
            intent.clipboard = Some(copy_of(context.source, context.rows, |key| {
                selection.is_some_and(|selection| selection.contains(key))
            }));
            ui.close();
        }
        if ui.button("Copy all shown").clicked() {
            intent.clipboard = Some(copy_of(context.source, context.rows, |_| true));
            ui.close();
        }
        if ui.button("Save shown\u{2026}").clicked() {
            let mut text = copy_of(context.source, context.rows, |_| true);
            text.push('\n');
            intent.save = Some((file_name(view.channel), text));
            ui.close();
        }
        if let Some(anchor) = line.anchor {
            ui.separator();
            outcome.left |= log_anchor::menu(ui, context.state, view, anchor, &mut intent.follow);
        }
    });
    outcome.cursor = view
        .selection
        .is_some_and(|selection| selection.cursor == line.key);
    if let Some(anchor) = line.anchor
        && response.hovered()
    {
        response.on_hover_text(
            context
                .state
                .log_anchor_refusal(anchor)
                .unwrap_or_else(|| log_anchor::hint(anchor)),
        );
    }
    outcome
}

/// The keys the log takes while it holds focus.
#[derive(Debug, Default)]
struct Keys {
    up: bool,
    down: bool,
    home: bool,
    end: bool,
    enter: bool,
    escape: bool,
    all: bool,
    copy: bool,
    find: bool,
}

/// Move the selection, select every shown row, copy, follow, find or let go.
///
/// The keys are taken only while the log holds focus, so the page never takes
/// a key from the surface a reader is working in, and they are consumed rather
/// than read, so nothing behind the dock acts on them too.
fn keyboard(
    ui: &Ui,
    state: &AppState,
    source: Source<'_>,
    rows: &Rows,
    view: &mut ConsoleView,
    intent: &mut Intent,
) {
    let id = body_id();
    let (focused, let_go) = ui.memory(|memory| {
        (
            memory.has_focus(id),
            memory.had_focus_last_frame(id) && memory.focused().is_none(),
        )
    });
    if !focused {
        // egui lets go of a focused widget on Escape before any page runs,
        // unless its lock filter claims the key, and a filter is in force only
        // from the second frame a widget holds focus. That Escape was still
        // the reader's, and still clears the selection.
        if let_go && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
            view.selection = None;
        }
        return;
    }
    // Read off the key itself as well as the held modifiers: a key event
    // carries the modifiers it was pressed with.
    let extend = ui.input(|input| {
        input.modifiers.shift
            || input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Key {
                        key: Key::ArrowUp | Key::ArrowDown | Key::Home | Key::End,
                        pressed: true,
                        modifiers,
                        ..
                    } if modifiers.shift
                )
            })
    });
    let keys = ui.input_mut(|input| {
        // A platform copy arrives as its own event rather than as a key.
        let copied = input
            .events
            .iter()
            .any(|event| matches!(event, egui::Event::Copy));
        input
            .events
            .retain(|event| !matches!(event, egui::Event::Copy));
        Keys {
            up: input.consume_key(Modifiers::NONE, Key::ArrowUp),
            down: input.consume_key(Modifiers::NONE, Key::ArrowDown),
            home: input.consume_key(Modifiers::NONE, Key::Home),
            end: input.consume_key(Modifiers::NONE, Key::End),
            enter: input.consume_key(Modifiers::NONE, Key::Enter),
            escape: input.consume_key(Modifiers::NONE, Key::Escape),
            all: input.consume_key(Modifiers::COMMAND, Key::A),
            copy: input.consume_key(Modifiers::COMMAND, Key::C) || copied,
            find: input.consume_key(Modifiers::COMMAND, Key::F),
        }
    });
    if keys.up || keys.down {
        // On the first frame the log holds focus its lock filter is not yet
        // in force, and egui has read the same arrow as a move to the widget
        // above or below. The log keeps the key, so the move is cancelled.
        ui.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
    }
    if keys.escape {
        ui.memory_mut(|memory| memory.surrender_focus(id));
        view.selection = None;
        return;
    }
    if keys.find {
        ui.memory_mut(|memory| memory.request_focus(find_id()));
        return;
    }
    let shown = rows.keys();
    if keys.all
        && let (Some(first), Some(last)) = (shown.first(), shown.last())
    {
        view.selection = Some(ConsoleSelection {
            anchor: *first,
            cursor: *last,
        });
    }
    let target = if keys.home {
        shown.first()
    } else if keys.end {
        shown.last()
    } else if keys.up || keys.down {
        // With nothing selected the keys start where a log is read from, its
        // newest line. A cursor the filters now hide moves to the shown row
        // beside it.
        match view
            .selection
            .map(|selection| rows.position(selection.cursor))
        {
            None => shown.last(),
            Some(Ok(index)) if keys.down => shown.get(index + 1).or(shown.last()),
            Some(Ok(index)) => shown.get(index.saturating_sub(1)),
            Some(Err(index)) if keys.down => shown.get(index).or(shown.last()),
            Some(Err(index)) => shown.get(index.saturating_sub(1)),
        }
    } else {
        None
    }
    .copied();
    if let Some(key) = target {
        view.selection = Some(match view.selection {
            Some(selection) if extend => ConsoleSelection {
                anchor: selection.anchor,
                cursor: key,
            },
            _ => ConsoleSelection::single(key),
        });
        view.reveal = Some(ConsoleReveal::Line(key));
    }
    if keys.copy
        && let Some(selection) = view.selection
    {
        intent.clipboard = Some(copy_of(source, rows, |key| selection.contains(key)));
    }
    if keys.enter
        && let Some(selection) = view.selection
        && let Some(anchor) = source.line(selection.cursor).and_then(|line| line.anchor)
        && log_anchor::follow(state, view, anchor, &mut intent.follow)
    {
        ui.memory_mut(|memory| memory.surrender_focus(id));
    }
}

/// The shown lines `taken` keeps, one per text line, in log order.
fn copy_of(source: Source<'_>, rows: &Rows, taken: impl Fn(u64) -> bool) -> String {
    let mut text = String::new();
    for line in rows
        .keys()
        .iter()
        .copied()
        .filter(|key| taken(*key))
        .filter_map(|key| source.line(key))
    {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&copied_line(&line));
    }
    text
}

/// A line as it is copied and saved: `HH:MM:SS  Severity  message`, the
/// severity padded so the messages of a block line up.
fn copied_line(line: &Line<'_>) -> String {
    format!(
        "{}  {:<7}  {}",
        clock(line.unix_ms),
        severity_word(line.severity),
        line.message
    )
}

/// The name a saved channel is offered under.
fn file_name(channel: ConsoleChannel) -> String {
    match channel {
        ConsoleChannel::Session => "session.log".to_owned(),
        ConsoleChannel::Run(sequence) => format!("run-{sequence}.log"),
    }
}

/// A line's severity as a word, for the reader who cannot see its mark.
fn severity_word(severity: LogSeverity) -> &'static str {
    match severity {
        LogSeverity::Error => "Error",
        LogSeverity::Warning => "Warning",
        LogSeverity::Info => "Info",
        LogSeverity::Debug => "Debug",
        LogSeverity::Trace => "Trace",
    }
}

/// The time a line was written, as `HH:MM:SS` on the reader's clock.
fn clock(unix_ms: u64) -> String {
    crate::time_compat::local_date_time(unix_ms)
        .map(|local| {
            format!(
                "{:02}:{:02}:{:02}",
                local.hour(),
                local.minute(),
                local.second()
            )
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests;
