//! The in-canvas entry: what a command wears when the only thing it still
//! needs is a name, and the reader has already said where.
//!
//! A net label, an off-sheet connector and a bus tap are placed at a click.
//! The click carries the anchor, the sheet and the object under the pointer,
//! so a modal that opens in the middle of the screen to ask for coordinates is
//! asking for what it was already given, and takes the drawing out from under
//! the reader while it does. This is the other shape: a small card at the
//! click, one or two fields in it, Enter to place and Escape to leave.
//!
//! It is deliberately the same vocabulary as [`super::command_form`] — the
//! same row grammar, the same reserved status slot, the same failure mark in
//! front of neutral words — so the two read as one family and a command can
//! move between them without the reader learning a second set of rules.

use egui::{Align, Context, Id, Key, Layout, Modifiers, Order, Response, Ui, vec2};

use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

use super::form;
use super::select::select;
use super::status_mark::{PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark};

/// The card's width. Wide enough for a bus slice and its resolved bits, narrow
/// enough to sit beside the pointer without covering the drawing it names.
const CARD_WIDTH: f32 = 248.0;
/// Air inside the card.
const CARD_PADDING: i8 = 8;
/// The label column. Every control in the card starts at one x.
const LABEL_COL: f32 = 52.0;
/// Air between a row's label and its control.
const LABEL_GAP: f32 = 6.0;
/// Air between two rows.
const ROW_GAP: f32 = 6.0;
/// One derived or status line, at `FS_0`.
const LINE_H: f32 = 14.0;
/// How many lines the status slot reserves. A refusal in a card this narrow
/// reaches a second line often enough that reserving one would make the card
/// resize while the reader types.
const STATUS_LINES: usize = 2;
/// Air between the last row and the status slot.
const STATUS_GAP: f32 = 6.0;
/// How far the card sits from the click, so the pointer never covers the
/// first field.
const ANCHOR_OFFSET: f32 = 12.0;
/// Air kept between the card and the edge of the screen.
const SCREEN_MARGIN: f32 = 8.0;

/// What the reader did to an open entry this frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanvasEntryChoice {
    /// Still typing.
    None,
    /// Enter, on a draft the caller said it would accept.
    Commit,
    /// Escape, or a click outside the card.
    Cancelled,
}

/// The rows of one entry, on the card's label column.
pub struct CanvasEntryRows<'u> {
    ui: &'u mut Ui,
    started: bool,
}

impl<'u> CanvasEntryRows<'u> {
    fn new(ui: &'u mut Ui) -> Self {
        ui.spacing_mut().item_spacing.y = 0.0;
        Self { ui, started: false }
    }

    fn row<R>(&mut self, label: &str, control: impl FnOnce(&mut Ui, f32) -> R) -> R {
        if self.started {
            self.ui.add_space(ROW_GAP);
        }
        self.started = true;
        let t = Tokens::get(self.ui.ctx());
        let row_h = t.metrics.ctl_h;
        let width = self.ui.available_width();
        self.ui
            .allocate_ui_with_layout(
                vec2(width, row_h),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let (rect, _) =
                        ui.allocate_exact_size(vec2(LABEL_COL, row_h), egui::Sense::hover());
                    ui.painter().text(
                        egui::pos2(rect.left(), rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        label,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        t.color.text_dim,
                    );
                    let width = ui.available_width();
                    control(ui, width)
                },
            )
            .inner
    }

    /// One single-line mono field.
    pub fn text(&mut self, label: &str, id: Id, value: &mut String, hint: &str) -> Response {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let response = ui.add_sized(
                vec2(width, t.metrics.ctl_h),
                egui::TextEdit::singleline(value)
                    .id(id)
                    .font(egui::TextStyle::Monospace)
                    .hint_text(hint)
                    .margin(egui::Margin::symmetric(6, 4)),
            );
            form::name_control(ui, &response, label);
            response
        })
    }

    /// One dropdown. Returns the picked index when the reader picked one.
    pub fn select(
        &mut self,
        label: &str,
        id_salt: &str,
        selected: &str,
        options: &[String],
    ) -> Option<usize> {
        self.row(label, |ui, width| {
            select(ui, id_salt, label, selected, options, width)
        })
    }

    /// A computed line under the control it reads off, indented to the control
    /// column: what the draft declares, never what is true of every draft.
    pub fn derived(&mut self, text: &str) {
        let t = Tokens::get(self.ui.ctx());
        self.ui.add_space(4.0);
        let full_width = self.ui.available_width();
        let indent = LABEL_COL + LABEL_GAP;
        let width = (full_width - indent).max(1.0);
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat {
                font_id: font,
                color: t.color.text_faint,
                ..Default::default()
            },
        );
        job.wrap = egui::text::TextWrapping {
            max_width: width,
            max_rows: 2,
            overflow_character: Some('\u{2026}'),
            ..Default::default()
        };
        let galley = self.ui.fonts_mut(|fonts| fonts.layout_job(job));
        let height = galley.size().y.max(LINE_H);
        let (rect, response) = self
            .ui
            .allocate_exact_size(vec2(full_width, height), egui::Sense::hover());
        self.ui.painter().galley(
            egui::pos2(rect.left() + indent, rect.top()),
            galley,
            t.color.text_faint,
        );
        let value = text.to_owned();
        self.ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Label);
            node.set_value(value.clone());
        });
    }
}

/// One in-canvas entry, anchored at the click that opened it.
///
/// Built, shown and discarded in a frame: the draft it edits lives in the
/// caller's state.
pub struct CanvasEntry<'a> {
    id_salt: &'a str,
    label: &'a str,
    anchor: egui::Pos2,
    status: Result<Option<&'a str>, &'a str>,
    commit_enabled: bool,
}

impl<'a> CanvasEntry<'a> {
    /// `label` names the whole card to assistive technology — "Net label",
    /// "Bus tap" — and is never painted: the fields say what they are.
    pub fn new(id_salt: &'a str, label: &'a str, anchor: egui::Pos2) -> Self {
        Self {
            id_salt,
            label,
            anchor,
            status: Ok(None),
            commit_enabled: true,
        }
    }

    /// The validation slot: `Ok(None)` says nothing, `Ok(Some(_))` states a
    /// neutral fact, `Err(_)` states why Enter is refused, behind a failure
    /// mark. The slot is one line tall whatever it says, so the card does not
    /// change size under the pointer as a name is typed.
    pub fn status(mut self, status: Result<Option<&'a str>, &'a str>) -> Self {
        self.status = status;
        self
    }

    /// Whether Enter commits. A refused draft keeps the entry open.
    pub fn commit_enabled(mut self, enabled: bool) -> Self {
        self.commit_enabled = enabled;
        self
    }

    /// Show the entry. `body` returns the control that takes the caret.
    pub fn show(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut CanvasEntryRows<'_>) -> Option<Id>,
    ) -> CanvasEntryChoice {
        let t = Tokens::get(ctx);
        let area_id = Id::new(("rspice.canvas-entry", self.id_salt));
        let position = placed_at(ctx, self.anchor);
        let status = self.status;
        let label = self.label;

        let mut focus_target = None;
        let area = egui::Area::new(area_id)
            .order(Order::Foreground)
            .fixed_pos(position)
            .interactable(true)
            .show(ctx, |ui| {
                ui.set_width(CARD_WIDTH);
                egui::Frame::new()
                    .fill(t.color.bg_panel)
                    .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                    .corner_radius(6.0)
                    .inner_margin(egui::Margin::same(CARD_PADDING))
                    .shadow(t.shadow())
                    .show(ui, |ui| {
                        ui.set_width(CARD_WIDTH - f32::from(CARD_PADDING) * 2.0);
                        ui.spacing_mut().item_spacing.y = 0.0;
                        focus_target = {
                            let mut rows = CanvasEntryRows::new(&mut *ui);
                            body(&mut rows)
                        };
                        paint_status(ui, status);
                    });
            });
        ctx.accesskit_node_builder(area_id, |node| {
            node.set_label(label.to_owned());
        });

        // The caret belongs in the first field from the frame the entry opens,
        // so the reader types a name rather than hunting for where to.
        if let Some(field) = focus_target
            && ctx.memory(|memory| memory.focused()).is_none()
        {
            ctx.memory_mut(|memory| memory.request_focus(field));
        }
        if let (Some(field), Err(message)) = (focus_target, status) {
            let message = message.to_owned();
            ctx.accesskit_node_builder(field, |node| {
                node.set_description(message.clone());
                node.set_invalid(egui::accesskit::Invalid::True);
            });
        }

        // Escape leaves at once: this draft owns nothing, so there is nothing
        // to confirm the discard of. While a dropdown is open both keys are
        // its own — Escape closes it, Enter picks from it — and neither has
        // anything to say about the entry behind it.
        let dropdown_open = ctx.any_popup_open();
        if !dropdown_open && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape))
        {
            return CanvasEntryChoice::Cancelled;
        }
        let entered =
            !dropdown_open && ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter));
        if entered && self.commit_enabled {
            return CanvasEntryChoice::Commit;
        }
        // A press anywhere else is the reader going back to the drawing. A
        // press inside an open dropdown is not: the dropdown is a layer of
        // its own, and dismissing on it would close the entry whenever an
        // option was picked.
        let dismissed = ctx.input(|input| input.pointer.any_pressed())
            && !area.response.contains_pointer()
            && !dropdown_open;
        if dismissed {
            return CanvasEntryChoice::Cancelled;
        }
        CanvasEntryChoice::None
    }
}

/// Where the card sits: beside the click, pulled back inside the screen when
/// the click was near an edge.
fn placed_at(ctx: &Context, anchor: egui::Pos2) -> egui::Pos2 {
    let screen = ctx.viewport_rect();
    // The height is not known until the card is laid out, so the vertical
    // clamp uses the tallest card this widget builds: two rows, a derived
    // line and the status slot.
    let height = 3.0 * (Tokens::get(ctx).metrics.ctl_h + ROW_GAP)
        + STATUS_GAP
        + LINE_H * STATUS_LINES as f32
        + f32::from(CARD_PADDING) * 2.0;
    let x = (anchor.x + ANCHOR_OFFSET).min(screen.right() - CARD_WIDTH - SCREEN_MARGIN);
    let y = (anchor.y + ANCHOR_OFFSET).min(screen.bottom() - height - SCREEN_MARGIN);
    egui::pos2(
        x.max(screen.left() + SCREEN_MARGIN),
        y.max(screen.top() + SCREEN_MARGIN),
    )
}

/// The status slot: up to [`STATUS_LINES`] lines, and no band at all when it
/// says nothing.
///
/// A refusal in a card this narrow runs to a second line more often than not,
/// so it wraps rather than eliding. Unlike a dialog's, this slot is not
/// reserved: the card has no footer under it, so a refusal appearing grows
/// the card downwards and moves nothing the reader is pointing at — and a
/// card that reserved it would be a card with a permanent empty band.
fn paint_status(ui: &mut Ui, status: Result<Option<&str>, &str>) {
    let t = Tokens::get(ui.ctx());
    let (message, color, problem) = match status {
        Ok(None) => return,
        Ok(Some(message)) => (message, t.color.text_dim, None),
        Err(message) => (message, t.color.text, Some(Problem::Failure)),
    };
    ui.add_space(STATUS_GAP);
    let width = ui.available_width();
    let (rect, response) = ui.allocate_exact_size(
        vec2(width, LINE_H * STATUS_LINES as f32),
        egui::Sense::hover(),
    );
    let text_left = if problem.is_some() {
        PROBLEM_MARK_ADVANCE
    } else {
        0.0
    };
    let mut job = egui::text::LayoutJob::single_section(
        message.to_owned(),
        egui::TextFormat {
            font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
            color,
            ..Default::default()
        },
    );
    job.wrap = egui::text::TextWrapping {
        max_width: (width - text_left).max(1.0),
        max_rows: STATUS_LINES,
        overflow_character: Some('\u{2026}'),
        ..Default::default()
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    if let Some(problem) = problem {
        let first_line = galley
            .rows
            .first()
            .map_or(galley.size().y * 0.5, |row| row.rect().center().y);
        paint_problem_mark(
            ui.painter(),
            egui::Rect::from_center_size(
                egui::pos2(
                    rect.left() + PROBLEM_MARK_SIDE * 0.5,
                    rect.top() + first_line,
                ),
                egui::Vec2::splat(PROBLEM_MARK_SIDE),
            ),
            problem,
        );
    }
    ui.painter().galley(
        egui::pos2(rect.left() + text_left, rect.top()),
        galley,
        color,
    );
    let message = message.to_owned();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_value(message.clone());
        if let Some(problem) = problem {
            node.set_description(problem.announcement());
        }
    });
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    fn entry(
        ctx: &Context,
        anchor: egui::Pos2,
        name: &mut String,
        status: Result<Option<&str>, &str>,
        commit_enabled: bool,
        events: Vec<egui::Event>,
    ) -> (CanvasEntryChoice, egui::Rect) {
        let mut rect = egui::Rect::NOTHING;
        let mut choice = CanvasEntryChoice::None;
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1024.0, 640.0),
                )),
                events,
                ..Default::default()
            },
            |ctx| {
                choice = CanvasEntry::new("test.net-label", "Net label", anchor)
                    .status(status)
                    .commit_enabled(commit_enabled)
                    .show(ctx, |rows| {
                        let id = Id::new("test.net-label.name");
                        let response = rows.text("Name", id, name, "vout");
                        rect = response.rect;
                        Some(id)
                    });
            },
        );
        (choice, rect)
    }

    fn key(key: Key) -> egui::Event {
        egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }
    }

    /// Enter places, Escape leaves, and a refused draft stays open under both
    /// — the caller decides what Enter is allowed to do with it.
    #[test]
    fn enter_commits_an_accepted_draft_and_escape_always_leaves() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let anchor = egui::pos2(300.0, 200.0);
        let mut name = String::from("vout");
        for _ in 0..2 {
            entry(&ctx, anchor, &mut name, Ok(None), true, Vec::new());
        }
        let (choice, _) = entry(
            &ctx,
            anchor,
            &mut name,
            Ok(None),
            true,
            vec![key(Key::Enter)],
        );
        assert_eq!(choice, CanvasEntryChoice::Commit);

        let (choice, _) = entry(
            &ctx,
            anchor,
            &mut name,
            Err("two nodes: a net name holds no space"),
            false,
            vec![key(Key::Enter)],
        );
        assert_eq!(
            choice,
            CanvasEntryChoice::None,
            "a refused draft is not placed by Enter"
        );

        let (choice, _) = entry(
            &ctx,
            anchor,
            &mut name,
            Err("two nodes: a net name holds no space"),
            false,
            vec![key(Key::Escape)],
        );
        assert_eq!(choice, CanvasEntryChoice::Cancelled);
    }

    /// The card sits beside the click, and is pulled back onto the screen
    /// when the click was in the bottom-right corner.
    #[test]
    fn the_card_follows_the_click_and_stays_on_screen() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut name = String::new();
        let (_, inside) = entry(
            &ctx,
            egui::pos2(300.0, 200.0),
            &mut name,
            Ok(None),
            true,
            Vec::new(),
        );
        assert!(
            inside.left() > 300.0 && inside.top() > 200.0,
            "the card is placed past the click: {inside:?}"
        );

        let (_, corner) = entry(
            &ctx,
            egui::pos2(1_020.0, 636.0),
            &mut name,
            Ok(None),
            true,
            Vec::new(),
        );
        let screen = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1024.0, 640.0));
        assert!(
            screen.contains_rect(corner),
            "the card left the screen: {corner:?}"
        );
    }

    /// A status says its piece below the fields, so the card grows downwards
    /// and the field the reader is typing in does not move. With nothing to
    /// say there is no band at all: this card has no footer to hold still,
    /// and a reserved slot here would be a permanent empty strip.
    #[test]
    fn a_status_grows_the_card_downwards_and_never_moves_the_field() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let anchor = egui::pos2(100.0, 100.0);
        let measure = |status: Result<Option<&str>, &str>| {
            let mut name = String::from("vout");
            let (mut card, mut field) = (egui::Rect::NOTHING, egui::Rect::NOTHING);
            for _ in 0..3 {
                let _ = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1024.0, 640.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        let _ = CanvasEntry::new("test.height", "Net label", anchor)
                            .status(status)
                            .show(ctx, |rows| {
                                let id = Id::new("test.height.name");
                                field = rows.text("Name", id, &mut name, "vout").rect;
                                Some(id)
                            });
                        card = ctx
                            .memory(|memory| {
                                memory.area_rect(Id::new(("rspice.canvas-entry", "test.height")))
                            })
                            .unwrap_or(egui::Rect::NOTHING);
                    },
                );
            }
            (card.height(), field)
        };
        let (blank, blank_field) = measure(Ok(None));
        let (fact, fact_field) = measure(Ok(Some("8 conductors")));
        let (refused, refused_field) = measure(Err("a net name holds no space"));
        assert!(blank > 0.0);
        assert_eq!(fact_field, blank_field, "a fact moves no field");
        assert_eq!(refused_field, blank_field, "a refusal moves no field");
        assert!(fact > blank, "a fact is said under the fields");
        assert_eq!(refused, fact, "both say their piece in the same band");
    }
}
