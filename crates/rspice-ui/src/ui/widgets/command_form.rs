//! The compact command form: the surface a schematic command wears when it
//! has to ask something before it arms a canvas tool.
//!
//! A placement command asks three or four questions, and the honest shape for
//! three or four questions is a 420 pt card with three or four rows in it.
//! What this module removes, relative to the 760 pt transaction surface these
//! forms used to borrow, is everything that was not one of those questions: a
//! constant kicker, a hand-drawn preview of an object the canvas already draws
//! for real under the pointer, cards restating the same three sentences on
//! every form, and a two-step discard for a draft that owns nothing.
//!
//! Two invariants are worth stating, because both are things a reader only
//! notices when they are missing:
//!
//! - The status slot is allocated whether or not it says anything, so the
//!   footer does not move under the pointer as a draft becomes valid.
//! - Every control is named to AccessKit by the label its row states. The
//!   labels here are painted, not laid out as widgets, so nothing else would
//!   name them.

use egui::{Color32, Context, Id, Response, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::dialog::{Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use super::form;
use super::segmented::{SegmentedWidth, segmented};
use super::select::select;

/// Air between a row's label column and its control.
const LABEL_GAP: f32 = 8.0;
/// Air between two rows.
const ROW_GAP: f32 = 8.0;
/// Air between a control and the derived line that reads off it.
const DERIVED_GAP: f32 = 4.0;
/// One derived line, at `FS_0`.
const DERIVED_LINE_H: f32 = 14.0;
/// One status line, at `FS_0`.
const STATUS_LINE_H: f32 = 15.0;
/// Air between the context line and the first row.
const CONTEXT_GAP: f32 = 10.0;
/// Below this content width a label no longer fits beside its control, and the
/// rows stack instead of shrinking the control into unusability.
const STACKED_MAX_WIDTH: f32 = 300.0;
/// Air between a stacked row's label and its control.
const STACKED_LABEL_GAP: f32 = 4.0;

/// The rows of one command form, laid out on a fixed label column.
///
/// Row kinds are added as a form needs them. There is no generic "widget row":
/// a row is a labelled control of a known kind, so the geometry below is the
/// only geometry any of these forms can have.
pub(crate) struct FormRows<'u> {
    ui: &'u mut Ui,
    label_col: f32,
    stacked: bool,
    started: bool,
}

impl<'u> FormRows<'u> {
    fn new(ui: &'u mut Ui, label_col: f32) -> Self {
        let stacked = ui.available_width() < STACKED_MAX_WIDTH;
        ui.spacing_mut().item_spacing.y = 0.0;
        Self {
            ui,
            label_col,
            stacked,
            started: false,
        }
    }

    /// The x offset a derived line and a stacked control start at.
    fn control_indent(&self) -> f32 {
        if self.stacked {
            0.0
        } else {
            self.label_col + LABEL_GAP
        }
    }

    /// Open one row: the gap above it, then its label, then its control.
    fn row<R>(&mut self, label: &str, control: impl FnOnce(&mut Ui, f32) -> R) -> R {
        if self.started {
            self.ui.add_space(ROW_GAP);
        }
        self.started = true;
        let t = Tokens::get(self.ui.ctx());
        let row_h = t.metrics.ctl_h;
        let full_width = self.ui.available_width();
        if self.stacked {
            self.paint_label(label);
            self.ui.add_space(STACKED_LABEL_GAP);
            return self
                .ui
                .allocate_ui_with_layout(
                    vec2(full_width, row_h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| control(ui, full_width),
                )
                .inner;
        }
        let label_col = self.label_col;
        self.ui
            .allocate_ui_with_layout(
                vec2(full_width, row_h),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let (label_rect, _) =
                        ui.allocate_exact_size(vec2(label_col, row_h), egui::Sense::hover());
                    paint_row_label(ui, label, label_rect);
                    let width = ui.available_width();
                    control(ui, width)
                },
            )
            .inner
    }

    /// A stacked row's label, above its control.
    fn paint_label(&mut self, label: &str) {
        let width = self.ui.available_width();
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(width, DERIVED_LINE_H), egui::Sense::hover());
        paint_row_label(self.ui, label, rect);
    }

    /// One single-line mono text field.
    ///
    /// A field that takes focus from the keyboard rather than a click shows its
    /// content selected, so a prefilled name is replaced by typing over it and
    /// a click still lands the caret where it was aimed.
    pub(crate) fn text(&mut self, label: &str, id: Id, value: &mut String, hint: &str) -> Response {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let response = ui.add_sized(
                vec2(width, t.metrics.ctl_h),
                egui::TextEdit::singleline(value)
                    .id(id)
                    .font(egui::TextStyle::Monospace)
                    .hint_text(hint)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            form::name_control(ui, &response, label);
            select_all_on_keyboard_focus(ui, &response, value);
            response
        })
    }

    /// One contiguous group of exclusive options. Returns `true` when the
    /// selection changed this frame.
    pub(crate) fn segmented(
        &mut self,
        label: &str,
        id_salt: &'static str,
        options: &[&str],
        selected: &mut usize,
    ) -> bool {
        self.row(label, |ui, _| {
            segmented(ui, id_salt, options, selected, SegmentedWidth::Natural)
        })
    }

    /// One dropdown. Returns the picked index when the reader picked one.
    pub(crate) fn select(
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
    /// column. It states what the draft above it declares, and nothing that is
    /// true of every draft.
    ///
    /// It wraps rather than eliding: what it says is the *content* of the
    /// draft, and half of it is not worth having.
    pub(crate) fn derived(&mut self, text: &str) {
        let t = Tokens::get(self.ui.ctx());
        let indent = self.control_indent();
        self.ui.add_space(DERIVED_GAP);
        let full_width = self.ui.available_width();
        let width = (full_width - indent).max(1.0);
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat {
                font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
                color: t.color.text_faint,
                ..Default::default()
            },
        );
        job.wrap = egui::text::TextWrapping {
            max_width: width,
            ..Default::default()
        };
        let galley = self.ui.fonts_mut(|fonts| fonts.layout_job(job));
        let height = galley.size().y.max(DERIVED_LINE_H);
        let (rect, _) = self
            .ui
            .allocate_exact_size(vec2(full_width, height), egui::Sense::hover());
        self.ui.painter().galley(
            egui::pos2(rect.left() + indent, rect.top()),
            galley,
            t.color.text_faint,
        );
    }
}

/// One row label: painted, dimmed, vertically centred on its control.
fn paint_row_label(ui: &Ui, label: &str, rect: egui::Rect) {
    let t = Tokens::get(ui.ctx());
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
}

/// Select the whole value when a field takes focus from anything but a click.
///
/// egui's own default is a caret at position zero, which for a prefilled name
/// means the reader types in front of a suggestion instead of over it.
fn select_all_on_keyboard_focus(ui: &Ui, response: &Response, value: &str) {
    if !response.gained_focus() || value.is_empty() {
        return;
    }
    if ui.input(|input| input.pointer.any_down() || input.pointer.any_pressed()) {
        return;
    }
    let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), response.id) else {
        return;
    };
    state
        .cursor
        .set_char_range(Some(egui::text::CCursorRange::two(
            egui::text::CCursor::new(0),
            egui::text::CCursor::new(value.chars().count()),
        )));
    egui::TextEdit::store_state(ui.ctx(), response.id, state);
}

/// `text` shortened from the middle until it fits `width` on one line.
///
/// The ends of a path or a name are what identify it; the middle is what a
/// reader skips. Truncating the tail, as a plain `truncate` does, leaves every
/// view of the same cell reading identically.
fn middle_elided(ui: &Ui, text: &str, font: &egui::FontId, color: Color32, width: f32) -> String {
    let measure = |candidate: &str| {
        ui.fonts_mut(|fonts| {
            fonts
                .layout_no_wrap(candidate.to_owned(), font.clone(), color)
                .size()
                .x
        })
    };
    if measure(text) <= width {
        return text.to_owned();
    }
    let chars: Vec<char> = text.chars().collect();
    let elide = |keep: usize| -> String {
        let head = keep.div_ceil(2);
        let tail = keep - head;
        let mut candidate: String = chars[..head].iter().collect();
        candidate.push('\u{2026}');
        candidate.extend(chars[chars.len() - tail..].iter());
        candidate
    };
    // Binary search the longest prefix+suffix that still fits, so a long view
    // path costs a handful of measurements rather than one per character.
    let (mut low, mut high) = (0usize, chars.len().saturating_sub(1));
    while low < high {
        let middle = (low + high).div_ceil(2);
        if measure(&elide(middle)) <= width {
            low = middle;
        } else {
            high = middle - 1;
        }
    }
    elide(low)
}

/// One compact command form.
///
/// Built, shown and discarded in a frame: the draft it edits lives in the
/// caller's dialog state, not here.
pub(crate) struct CommandForm<'a> {
    title: &'a str,
    primary: &'a str,
    context: Option<&'a str>,
    describe: Option<&'a str>,
    primary_enabled: bool,
    fields_enabled: bool,
    status: Result<Option<&'a str>, &'a str>,
    status_lines: usize,
}

impl<'a> CommandForm<'a> {
    pub(crate) fn new(title: &'a str, primary: &'a str) -> Self {
        Self {
            title,
            primary,
            context: None,
            describe: None,
            primary_enabled: true,
            fields_enabled: true,
            status: Ok(None),
            status_lines: 1,
        }
    }

    /// One mono line under the header naming what the command will act on.
    ///
    /// It carries only what depends on the draft or the selection — a cell
    /// path, a clicked object. A constant here would be a kicker with extra
    /// steps, so `None` prints nothing at all rather than a placeholder.
    pub(crate) fn context(mut self, context: Option<&'a str>) -> Self {
        self.context = context;
        self
    }

    /// The sentence assistive technology reads for the whole surface.
    pub(crate) fn describe(mut self, description: &'a str) -> Self {
        self.describe = Some(description);
        self
    }

    pub(crate) fn primary_enabled(mut self, enabled: bool) -> Self {
        self.primary_enabled = enabled;
        self
    }

    /// Disable every row, for a form that can still be read but not filled —
    /// a read-only document, or one that changed under an open form.
    pub(crate) fn fields_enabled(mut self, enabled: bool) -> Self {
        self.fields_enabled = enabled;
        self
    }

    /// The validation slot: `Ok(None)` says nothing, `Ok(Some(_))` states a
    /// neutral fact, `Err(_)` states why the primary is refused.
    pub(crate) fn status(mut self, status: Result<Option<&'a str>, &'a str>) -> Self {
        self.status = status;
        self
    }

    /// How many lines the status slot reserves. The slot is that tall whether
    /// or not it says anything, so the footer never moves as a draft changes.
    pub(crate) fn status_lines(mut self, lines: usize) -> Self {
        self.status_lines = lines.max(1);
        self
    }

    /// Show the form. `body` returns the control that takes initial focus.
    pub(crate) fn show(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut FormRows<'_>) -> Option<Id>,
    ) -> DialogChoice {
        let status = self.status;
        let status_lines = self.status_lines;
        let context = self.context;
        let fields_enabled = self.fields_enabled;
        // Cancel is the only word any form in this family has for leaving, and
        // Enter is the only key any of them commits on: a form with a
        // multiline body will add the opt-out when it arrives.
        let mut dialog = Dialog::prompt(self.title, self.primary)
            .size(DialogSize::CommandForm)
            .overlay_scrollbar()
            .ghost("Cancel")
            .primary_enabled(self.primary_enabled)
            .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(describe) = self.describe {
            dialog = dialog.description(describe);
        }
        let mut focused_field = None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            // Every gap in this body is stated, so nothing is the sum of an
            // explicit space and egui's own inter-widget one.
            ui.spacing_mut().item_spacing.y = 0.0;
            if let Some(context) = context {
                paint_context_line(ui, context);
                ui.add_space(CONTEXT_GAP);
            }
            let requested = {
                if !fields_enabled {
                    ui.disable();
                }
                let mut rows = FormRows::new(&mut *ui, form::LABEL_COL);
                body(&mut rows)
            };
            paint_status(ui, status, status_lines);
            focused_field = requested;
            requested
        });
        if let Err(message) = status {
            mark_invalid(ctx, focused_field, message);
        }
        choice
    }
}

/// The context line: one mono `FS_0` row that never wraps.
fn paint_context_line(ui: &mut Ui, context: &str) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (rect, response) =
        ui.allocate_exact_size(vec2(width, DERIVED_LINE_H), egui::Sense::hover());
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let shown = middle_elided(ui, context, &font, t.color.text_dim, width);
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        shown,
        font,
        t.color.text_dim,
    );
    let context = context.to_owned();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_value(context.clone());
    });
}

/// The status slot, always `lines` lines tall.
fn paint_status(ui: &mut Ui, status: Result<Option<&str>, &str>, lines: usize) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(ROW_GAP);
    let width = ui.available_width();
    let height = STATUS_LINE_H * lines as f32;
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), egui::Sense::hover());
    let (message, color) = match status {
        Ok(None) => return,
        Ok(Some(message)) => (message, t.color.text_dim),
        Err(message) => (message, t.color.err),
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
        max_width: width,
        max_rows: lines,
        overflow_character: Some('\u{2026}'),
        ..Default::default()
    };
    let galley = ui.fonts_mut(|fonts| fonts.layout_job(job));
    ui.painter()
        .galley(egui::pos2(rect.left(), rect.top()), galley, color);
    let message = message.to_owned();
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_value(message.clone());
    });
}

/// Mark the form's principal field invalid, and give it the refusal as its
/// description, so the sentence in the status slot reaches a screen reader that
/// never leaves the field.
///
/// The principal field is the one the body nominates for initial focus, which
/// in a form of this size is the field every refusal is about.
fn mark_invalid(ctx: &Context, field: Option<Id>, message: &str) {
    let Some(field) = field else {
        return;
    };
    let message = message.to_owned();
    ctx.accesskit_node_builder(field, |node| {
        node.set_description(message.clone());
        node.set_invalid(egui::accesskit::Invalid::True);
    });
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::ui::tokens::Mode;
    use crate::ui::widgets::painted_runs::painted_runs;

    /// A fixture body of the shape every command form in this family has: one
    /// text row with a derived line, two segmented rows, one select.
    fn fixture(ctx: &Context, status: Result<Option<&str>, &str>) -> DialogChoice {
        let mut name = String::from("VERY_LONG_INTERFACE_NAME_1 VERY_LONG_INTERFACE_NAME_2");
        let mut direction = 0usize;
        let mut signal = 0usize;
        CommandForm::new("Command form", "Place")
            .context(Some("work/ota_5t/schematic"))
            .describe("A fixture of the command form's row vocabulary.")
            .status(status)
            .status_lines(2)
            .show(ctx, |rows| {
                let field = rows.text(
                    "Names",
                    Id::new("command-form.fixture.names"),
                    &mut name,
                    "IN OUT VDD",
                );
                rows.derived("2 pins \u{00b7} 2 conductors \u{00b7} port-list positions 1 to 2");
                rows.segmented(
                    "Direction",
                    "command-form.fixture.direction",
                    &["Input", "Output", "Inout", "Supply"],
                    &mut direction,
                );
                rows.segmented(
                    "Signal",
                    "command-form.fixture.signal",
                    &["Analog", "Logic", "Power"],
                    &mut signal,
                );
                let _ = rows.select(
                    "Discipline",
                    "command-form.fixture.discipline",
                    "electrical",
                    &["electrical".to_owned(), "logic".to_owned()],
                );
                Some(field.id)
            })
    }

    /// Nothing the shell paints leaves the card, at any viewport it is offered
    /// at, in either theme, with the status slot at its tallest.
    #[test]
    fn a_command_form_fits_every_viewport() {
        for screen in [
            egui::vec2(1024.0, 640.0),
            egui::vec2(760.0, 900.0),
            egui::vec2(390.0, 844.0),
        ] {
            for mode in [Mode::Dark, Mode::Light] {
                let painted = painted_runs(screen, mode, 3, |ctx| {
                    let _ = fixture(
                        ctx,
                        Err(
                            "VERY_LONG_INTERFACE_NAME_1: an interface port named 'VERY_LONG_INTERFACE_NAME_1' already exists",
                        ),
                    );
                });
                painted.assert_inside_clip_and_surface(&format!("{screen:?} {mode:?}"));
                assert!(
                    painted.surface.height() < screen.y,
                    "{screen:?} {mode:?}: the form is taller than the viewport ({:?})",
                    painted.surface
                );
                assert!(
                    painted
                        .rect_of("Place")
                        .is_some_and(|rect| painted.surface.expand(1.0).contains_rect(rect)),
                    "{screen:?} {mode:?}: the primary is not inside the surface"
                );
            }
        }
    }

    /// The slot is the same height whether or not it says anything, so a form
    /// that becomes invalid does not move its own footer out from under the
    /// pointer.
    #[test]
    fn the_status_slot_holds_its_height_between_valid_and_invalid() {
        let screen = egui::vec2(1024.0, 640.0);
        let valid = painted_runs(screen, Mode::Dark, 3, |ctx| {
            let _ = fixture(ctx, Ok(None));
        });
        let invalid = painted_runs(screen, Mode::Dark, 3, |ctx| {
            let _ = fixture(ctx, Err("EN: an interface port named 'EN' already exists"));
        });
        assert_eq!(valid.surface.height(), invalid.surface.height());
        assert_eq!(
            valid.rect_of("Place").map(|rect| rect.top()),
            invalid.rect_of("Place").map(|rect| rect.top()),
        );
    }

    /// The card is 420 pt wide wherever it fits, and is inset rather than
    /// becoming a full-screen sheet where it does not.
    #[test]
    fn the_card_is_420_wide_and_never_a_sheet() {
        for (screen, expected) in [
            (egui::vec2(1024.0, 640.0), 420.0),
            (egui::vec2(760.0, 900.0), 420.0),
            (egui::vec2(390.0, 844.0), 366.0),
        ] {
            let painted = painted_runs(screen, Mode::Dark, 3, |ctx| {
                let _ = fixture(ctx, Ok(None));
            });
            assert_eq!(
                painted.surface.width(),
                expected,
                "{screen:?}: surface {:?}",
                painted.surface
            );
        }
    }

    /// A long line is shortened from the middle, so both ends survive.
    #[test]
    fn a_context_line_is_elided_in_the_middle() {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(800.0, 600.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
                    let long = "analog_library/differential_amplifier_stage_two/schematic";
                    let short = middle_elided(ui, long, &font, Color32::WHITE, 4_000.0);
                    assert_eq!(short, long, "a line that fits is left alone");

                    let elided = middle_elided(ui, long, &font, Color32::WHITE, 120.0);
                    assert!(elided.contains('\u{2026}'), "{elided}");
                    assert!(elided.starts_with("analog"), "{elided}");
                    assert!(elided.ends_with("schematic"), "{elided}");
                    assert!(elided.chars().count() < long.chars().count());
                });
            },
        );
    }
}
