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
//! - The status is stated in the footer, left of the actions, so nothing
//!   moves under the pointer as a draft becomes valid, and the body ends at
//!   its last row: the air under it is the air over the first. A form whose
//!   refusals are each about one field states them in that field's line
//!   instead, which is always there.
//! - Every control is named to AccessKit by the label its row states. The
//!   labels here are painted, not laid out as widgets, so nothing else would
//!   name them.

use egui::{Align, Color32, Context, Id, Key, Layout, Modifiers, Rect, Response, Ui, pos2, vec2};

use crate::panels::{WorkbenchIcon, icon_button};
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

use super::dialog::{Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize};
use super::form;
use super::segmented::{SegmentedWidth, segmented};
use super::select::{SelectOutput, select, select_with_response};

/// Air between a row's label column and its control.
const LABEL_GAP: f32 = 8.0;
/// Air between two rows.
const ROW_GAP: f32 = 8.0;
/// Air between a control and the derived line that reads off it.
const DERIVED_GAP: f32 = 4.0;
/// One derived line, at `FS_0`.
const DERIVED_LINE_H: f32 = 14.0;
/// Below this content width a label no longer fits beside its control, and the
/// rows stack instead of shrinking the control into unusability.
const STACKED_MAX_WIDTH: f32 = 300.0;
/// Air between a stacked row's label and its control.
const STACKED_LABEL_GAP: f32 = 4.0;
/// Where a value starts inside its control: the inset a select and a text
/// field give the value they show. A read-only value starts there too, so
/// every value in a form starts at one x.
const VALUE_INSET: f32 = 8.0;
/// One further line of a multi-line field, past the first.
const MULTILINE_LINE_H: f32 = 17.0;
/// A whole-number field: wide enough for four digits and its drag affordance,
/// narrow enough to leave the bounds beside it on one line.
const NUMBER_FIELD_W: f32 = 64.0;
/// One line of a set of checks or choices, and one entry of a list, under a
/// fine pointer. These are words beside a mark, not inputs, so they take less
/// than a field; under a finger every line is a full control tall.
const MARK_LINE_H: f32 = 22.0;
/// A check box's side, and a choice circle's diameter.
const MARK_SIDE: f32 = 14.0;
/// Air between a mark and its words.
const MARK_GAP: f32 = 6.0;
/// Air between two columns of a set of checks or choices.
const MARK_COLUMN_GAP: f32 = 16.0;
/// Air between the two axes of a point row.
const POINT_GAP: f32 = 8.0;
/// Entries a list shows before it scrolls inside its own row.
const LIST_VISIBLE_ENTRIES: usize = 5;
/// Air between a list's last entry and the field that adds the next.
const LIST_ADD_GAP: f32 = 4.0;
/// Air between a list's add field and its add button.
const LIST_BUTTON_GAP: f32 = 4.0;
/// Air between two columns of a table.
const TABLE_COLUMN_GAP: f32 = 16.0;

/// The words a list row shows besides its label. See [`FormRows::list`].
#[derive(Debug, Clone, Copy)]
pub struct ListWords<'a> {
    /// The add field's hint: the shape of one entry.
    pub hint: &'a str,
    /// The add button's name.
    pub add: &'a str,
    /// The remove buttons' verb; each is named this and the entry it removes.
    pub remove: &'a str,
}

/// What the reader did to a list row this frame. See [`FormRows::list`].
#[derive(Debug, Clone)]
pub struct ListEdit {
    /// The field that takes the next entry. Its line, written by
    /// [`FormRows::field_line`], names it.
    pub field: Response,
    /// The reader asked to add what the field holds: Enter in the field, or
    /// its add button. Whether it may be added is the caller's to judge.
    pub add: bool,
    /// The entry whose remove button was pressed.
    pub remove: Option<usize>,
}

/// What the one-line slot under a field says. See [`FormRows::field_line`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldLine<'a> {
    /// Nothing yet. The slot keeps its height and stays blank.
    Blank,
    /// What the field's value declares, set as a derived line.
    Derived(&'a str),
    /// Why the field's value is refused, in the error colour.
    Error(&'a str),
}

/// The rows of one command form, laid out on a fixed label column.
///
/// Row kinds are added as a form needs them. There is no generic "widget row":
/// a row is a labelled control of a known kind, so the geometry below is the
/// only geometry any of these forms can have.
pub struct FormRows<'u> {
    ui: &'u mut Ui,
    label_col: f32,
    stacked: bool,
    started: bool,
}

impl<'u> FormRows<'u> {
    fn new(ui: &'u mut Ui, label_col: f32) -> Self {
        let stacked = ui.available_width() < STACKED_MAX_WIDTH;
        ui.spacing_mut().item_spacing.y = 0.0;
        // egui sets a field's hint in `weak_text_color`, which it derives by
        // fading the body tone, and overwrites any colour stated on the hint
        // itself. Derived, a hint landed darker than the labels in light mode
        // and brighter than them in dark: it read as a value already typed.
        // Stated for the form's body, it is the palette's faint tone.
        ui.visuals_mut().weak_text_color = Some(Tokens::get(ui.ctx()).color.text_faint);
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
    pub fn text(&mut self, label: &str, id: Id, value: &mut String, hint: &str) -> Response {
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

    /// One bounded whole number, with what the engine will accept stated
    /// after it.
    ///
    /// The words are the bounds, not a restatement of the control: a port
    /// that may be left out says so there, where the reader is choosing.
    pub fn number(
        &mut self,
        label: &str,
        value: &mut usize,
        range: std::ops::RangeInclusive<usize>,
        allowed: &str,
    ) -> Response {
        self.row(label, |ui, _| {
            let t = Tokens::get(ui.ctx());
            let response = ui.add_sized(
                vec2(NUMBER_FIELD_W, t.metrics.ctl_h),
                egui::DragValue::new(value).range(range).speed(1.0),
            );
            form::name_control(ui, &response, label);
            ui.label(
                egui::RichText::new(allowed)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            response
        })
    }

    /// One multi-line mono text field, `lines` controls tall.
    ///
    /// For the one value in this family that is prose rather than a name: a
    /// design note's text, where a line break is content the model keeps. A
    /// form holding one of these hands Enter to the body with
    /// [`CommandForm::primary_on_enter`].
    pub fn text_multiline(
        &mut self,
        label: &str,
        id: Id,
        value: &mut String,
        hint: &str,
        lines: usize,
    ) -> Response {
        let t = Tokens::get(self.ui.ctx());
        let first_line_h = t.metrics.ctl_h;
        let height = first_line_h + MULTILINE_LINE_H * lines.saturating_sub(1) as f32;
        self.block(label, first_line_h, |ui, width| {
            let response = ui.add_sized(
                vec2(width, height),
                egui::TextEdit::multiline(value)
                    .id(id)
                    .font(egui::TextStyle::Monospace)
                    .hint_text(hint)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            form::name_control(ui, &response, label);
            response
        })
    }

    /// One point: its X and Y side by side in the control column, for a form
    /// that enters exact geometry. Returns the two fields in that order.
    pub fn point(
        &mut self,
        label: &str,
        id: Id,
        x: &mut String,
        y: &mut String,
    ) -> (Response, Response) {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let field = ((width - POINT_GAP) * 0.5).max(1.0);
            ui.spacing_mut().item_spacing.x = POINT_GAP;
            let axis = |ui: &mut Ui, axis: &str, value: &mut String| {
                let response = ui.add_sized(
                    vec2(field, t.metrics.ctl_h),
                    egui::TextEdit::singleline(value)
                        .id(id.with(axis))
                        .font(egui::TextStyle::Monospace)
                        .hint_text(axis)
                        .margin(egui::Margin::symmetric(8, 4)),
                );
                form::name_control(ui, &response, &format!("{label} {axis}"));
                response
            };
            let x = axis(ui, "X", x);
            let y = axis(ui, "Y", y);
            (x, y)
        })
    }

    /// One contiguous group of exclusive options. Returns `true` when the
    /// selection changed this frame.
    pub fn segmented(
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

    /// [`Self::select`] with the control itself, for a form whose only row is
    /// the dropdown and which therefore has to be able to give it the caret.
    pub fn select_with_response(
        &mut self,
        label: &str,
        id_salt: &str,
        selected: &str,
        options: &[String],
    ) -> SelectOutput {
        self.row(label, |ui, width| {
            select_with_response(ui, id_salt, label, selected, options, width)
        })
    }

    /// One read-only fact, on the same grid as the controls: its label in the
    /// label column, its value where a control's value would start. What a
    /// form acts on is stated in a row, never in an unlabelled line above it.
    pub fn value(&mut self, label: &str, value: &str) {
        self.value_row(label, value, theme::sans(tokens::FS_0, FontWeight::Regular));
    }

    /// [`Self::value`] for a path or an identifier, which is set in mono. A
    /// value too long for the row is shortened from the middle, so both ends
    /// of a path survive.
    pub fn value_mono(&mut self, label: &str, value: &str) {
        self.value_row(label, value, theme::mono(tokens::FS_0, FontWeight::Regular));
    }

    fn value_row(&mut self, label: &str, value: &str, font: egui::FontId) {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let (rect, response) =
                ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), egui::Sense::hover());
            let shown = middle_elided(ui, value, &font, t.color.text, width - VALUE_INSET);
            ui.painter().text(
                egui::pos2(rect.left() + VALUE_INSET, rect.center().y),
                egui::Align2::LEFT_CENTER,
                shown,
                font,
                t.color.text,
            );
            // `widget_info` gives the node its bounds; a node built without
            // them is dropped by every consumer of the tree, so the fact was
            // painted and never read.
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
            });
            let (label, value) = (label.to_owned(), value.to_owned());
            ui.ctx().accesskit_node_builder(response.id, |node| {
                node.set_role(egui::accesskit::Role::Label);
                node.set_label(label.clone());
                node.set_value(value.clone());
            });
        });
    }

    /// A computed line under the control it reads off, indented to the control
    /// column. It states what the draft above it declares, and nothing that is
    /// true of every draft.
    ///
    /// It wraps rather than eliding: what it says is the *content* of the
    /// draft, and half of it is not worth having.
    pub fn derived(&mut self, text: &str) {
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

    /// The one-line slot under the field `field` names: blank, what the
    /// field's value declares, or why it is refused, in the error colour.
    ///
    /// The slot is present whatever it says, so nothing below it moves while
    /// the field is typed in, and a refusal stands under the field it is about
    /// instead of in the form's status line, which is left for a failure that
    /// belongs to no field. A line too long for the control column is
    /// shortened from the middle, with the whole of it on hover. A refusal
    /// also marks the field invalid and becomes its description, so a screen
    /// reader that never leaves the field still hears it.
    pub fn field_line(&mut self, field: Option<Id>, line: FieldLine<'_>) {
        let t = Tokens::get(self.ui.ctx());
        let indent = self.control_indent();
        self.ui.add_space(DERIVED_GAP);
        let full_width = self.ui.available_width();
        let width = (full_width - indent).max(1.0);
        let (rect, response) = self
            .ui
            .allocate_exact_size(vec2(full_width, DERIVED_LINE_H), egui::Sense::hover());
        let (text, font, color) = match line {
            FieldLine::Blank => return,
            FieldLine::Derived(text) => (
                text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            ),
            FieldLine::Error(text) => {
                mark_invalid(self.ui.ctx(), field, text);
                (
                    text,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.err,
                )
            }
        };
        let shown = middle_elided(self.ui, text, &font, color, width);
        let elided = shown != text;
        self.ui.painter().text(
            egui::pos2(rect.left() + indent, rect.center().y),
            egui::Align2::LEFT_CENTER,
            shown,
            font,
            color,
        );
        let value = text.to_owned();
        // A hover-only allocation gets no bounds of its own, and a line a
        // reader cannot locate is a line they cannot tell belongs to its field.
        self.ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Label);
            node.set_value(value.clone());
            node.set_bounds(egui::accesskit::Rect {
                x0: rect.min.x.into(),
                y0: rect.min.y.into(),
                x1: rect.max.x.into(),
                y1: rect.max.y.into(),
            });
        });
        if elided {
            response.on_hover_text(text);
        }
    }

    /// Open a row whose control is more than one line tall. The label sits
    /// beside the control's first line, `first_line_h` tall, so it reads
    /// level with the first thing it names; the control takes the column
    /// below it, top down.
    fn block<R>(
        &mut self,
        label: &str,
        first_line_h: f32,
        control: impl FnOnce(&mut Ui, f32) -> R,
    ) -> R {
        if self.started {
            self.ui.add_space(ROW_GAP);
        }
        self.started = true;
        let full_width = self.ui.available_width();
        let column = |ui: &mut Ui, width: f32| {
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(width);
                ui.spacing_mut().item_spacing.y = 0.0;
                control(ui, width)
            })
            .inner
        };
        if self.stacked {
            self.paint_label(label);
            self.ui.add_space(STACKED_LABEL_GAP);
            return column(self.ui, full_width);
        }
        let label_col = self.label_col;
        self.ui
            .allocate_ui_with_layout(
                vec2(full_width, first_line_h),
                Layout::left_to_right(Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let (label_rect, _) =
                        ui.allocate_exact_size(vec2(label_col, first_line_h), egui::Sense::hover());
                    paint_row_label(ui, label, label_rect);
                    let width = ui.available_width();
                    column(ui, width)
                },
            )
            .inner
    }

    /// A set of independent on/off choices, each its own check box. Items
    /// flow into as many columns as the control column holds at the width of
    /// the widest, in reading order, so a short set is one line and a long one
    /// is a grid rather than a ragged run. Returns `true` when any changed.
    pub fn checks(&mut self, label: &str, items: &mut [(&str, &mut bool)]) -> bool {
        let line_h = mark_line_height(self.ui);
        let trim = mark_block_trim(self.ui, line_h);
        let id = self.ui.id().with(("command-form.checks", label));
        self.block(label, line_h - 2.0 * trim, |ui, width| {
            let words = items.iter().map(|(words, _)| *words).collect::<Vec<_>>();
            let (columns, column_w) = mark_columns(ui, &words, width);
            let lines = items.len().div_ceil(columns);
            let mut changed = false;
            for (line_index, line) in items.chunks_mut(columns).enumerate() {
                let line_rect = allocate_mark_line(ui, width, line_h, trim, line_index, lines);
                for (column, (words, value)) in line.iter_mut().enumerate() {
                    let left = line_rect.left() + column as f32 * (column_w + MARK_COLUMN_GAP);
                    let item = id.with(line_index * columns + column);
                    if mark_control(ui, item, left, line_rect, words, **value, MarkKind::Check)
                        .clicked()
                    {
                        **value = !**value;
                        changed = true;
                    }
                }
            }
            changed
        })
    }

    /// One exclusive choice among a few, each its own radio button, laid out
    /// as [`Self::checks`] lays out its set. Returns `true` when the selection
    /// changed this frame.
    pub fn choice(&mut self, label: &str, options: &[&str], selected: &mut usize) -> bool {
        let line_h = mark_line_height(self.ui);
        let trim = mark_block_trim(self.ui, line_h);
        let id = self.ui.id().with(("command-form.choice", label));
        self.block(label, line_h - 2.0 * trim, |ui, width| {
            let (columns, column_w) = mark_columns(ui, options, width);
            let lines = options.len().div_ceil(columns);
            let mut changed = false;
            for (line_index, line) in options.chunks(columns).enumerate() {
                let line_rect = allocate_mark_line(ui, width, line_h, trim, line_index, lines);
                for (column, words) in line.iter().enumerate() {
                    let index = line_index * columns + column;
                    let left = line_rect.left() + column as f32 * (column_w + MARK_COLUMN_GAP);
                    let on = *selected == index;
                    if mark_control(
                        ui,
                        id.with(index),
                        left,
                        line_rect,
                        words,
                        on,
                        MarkKind::Radio,
                    )
                    .clicked()
                        && !on
                    {
                        *selected = index;
                        changed = true;
                    }
                }
            }
            changed
        })
    }

    /// A list of short values the reader adds to and removes from: each entry
    /// on its own line with a remove button at the line's end, then the field
    /// that takes the next entry and its add button. Past
    /// [`LIST_VISIBLE_ENTRIES`] the entries scroll inside the row, so a long
    /// list never pushes the footer away.
    ///
    /// The row reports what was asked; judging and applying it is the
    /// caller's, which states any refusal in the line under the field with
    /// [`Self::field_line`]. Enter in the field asks to add and is spent
    /// there, so it never also commits the form; Enter in an empty field is
    /// left to the form.
    pub fn list(
        &mut self,
        label: &str,
        id: Id,
        entries: &[String],
        draft: &mut String,
        words: ListWords<'_>,
    ) -> ListEdit {
        let ListWords {
            hint,
            add: add_label,
            remove: remove_label,
        } = words;
        let t = Tokens::get(self.ui.ctx());
        let line_h = mark_line_height(self.ui);
        let ctl_h = t.metrics.ctl_h;
        let first_line_h = if entries.is_empty() { ctl_h } else { line_h };
        self.block(label, first_line_h, |ui, width| {
            let mut remove = None;
            let mut entry_lines = |ui: &mut Ui| {
                for (index, entry) in entries.iter().enumerate() {
                    if list_entry(ui, id.with(("entry", index)), entry, remove_label, width) {
                        remove = Some(index);
                    }
                }
            };
            if entries.len() > LIST_VISIBLE_ENTRIES {
                egui::ScrollArea::vertical()
                    .id_salt(id.with("entries"))
                    .max_height(LIST_VISIBLE_ENTRIES as f32 * line_h)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 0.0;
                        entry_lines(ui);
                    });
            } else {
                entry_lines(ui);
            }
            if !entries.is_empty() {
                ui.add_space(LIST_ADD_GAP);
            }
            let (field, add) = ui
                .allocate_ui_with_layout(
                    vec2(width, ctl_h),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = LIST_BUTTON_GAP;
                        let field = ui.add_sized(
                            vec2((width - ctl_h - LIST_BUTTON_GAP).max(1.0), ctl_h),
                            egui::TextEdit::singleline(draft)
                                .id(id)
                                .font(egui::TextStyle::Monospace)
                                .hint_text(hint)
                                .margin(egui::Margin::symmetric(8, 4)),
                        );
                        form::name_control(ui, &field, label);
                        let typed = !draft.trim().is_empty();
                        let entered = typed
                            && field.lost_focus()
                            && ui.input(|input| input.key_pressed(Key::Enter));
                        if entered {
                            // Spent here, so the form's own Enter does not
                            // also commit a draft the reader is still filling.
                            ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Enter));
                            field.request_focus();
                        }
                        let button = ui
                            .add_enabled_ui(typed, |ui| {
                                icon_button(
                                    ui,
                                    WorkbenchIcon::Add,
                                    add_label,
                                    false,
                                    vec2(ctl_h, ctl_h),
                                )
                            })
                            .inner;
                        (field, entered || (typed && button.clicked()))
                    },
                )
                .inner;
            ListEdit { field, add, remove }
        })
    }

    /// A read-only fact with the one action it offers at the row's end: a
    /// count of findings beside the place they can be visited, a defect
    /// beside its repair. Returns whether the action was pressed.
    pub fn value_action(&mut self, label: &str, value: &str, action: &str) -> bool {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            ui.allocate_ui_with_layout(
                vec2(width, t.metrics.ctl_h),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let clicked = super::Button::new(action).show(ui).clicked();
                    let (rect, response) = ui.allocate_exact_size(
                        vec2(ui.available_width().max(1.0), t.metrics.ctl_h),
                        egui::Sense::hover(),
                    );
                    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
                    let shown =
                        middle_elided(ui, value, &font, t.color.text, rect.width() - VALUE_INSET);
                    ui.painter().text(
                        pos2(rect.left() + VALUE_INSET, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        shown,
                        font,
                        t.color.text,
                    );
                    let spoken = format!("{label}: {value}");
                    response.widget_info(|| {
                        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &spoken)
                    });
                    clicked
                },
            )
            .inner
        })
    }

    /// A read-only table across the whole form, its first column on the
    /// labels' edge: what a command will make or keep, one line per item, read
    /// before the primary is pressed. Columns are as wide as their widest
    /// entry and start at one x each, left-aligned; a table wider than the
    /// form shortens its longest entries from the middle. `rows` holds one
    /// entry per column.
    pub fn table(&mut self, columns: &[TableColumn<'_>], rows: &[Vec<String>]) {
        if self.started {
            self.ui.add_space(ROW_GAP);
        }
        self.started = true;
        let t = Tokens::get(self.ui.ctx());
        let width = self.ui.available_width();
        let line_h = mark_line_height(self.ui);
        let header_font = theme::sans(tokens::FS_0, FontWeight::SemiBold);
        let cell_font = |column: &TableColumn<'_>| {
            if column.mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            }
        };
        let measure = |ui: &Ui, text: &str, font: &egui::FontId| {
            ui.fonts_mut(|fonts| {
                fonts
                    .layout_no_wrap(text.to_owned(), font.clone(), t.color.text)
                    .size()
                    .x
            })
        };
        let mut widths = columns
            .iter()
            .enumerate()
            .map(|(index, column)| {
                rows.iter()
                    .filter_map(|row| row.get(index))
                    .map(|cell| measure(self.ui, cell, &cell_font(column)))
                    .fold(measure(self.ui, column.header, &header_font), f32::max)
            })
            .collect::<Vec<_>>();
        let gaps = TABLE_COLUMN_GAP * columns.len().saturating_sub(1) as f32;
        let natural = widths.iter().sum::<f32>() + gaps;
        if natural > width {
            let scale = ((width - gaps).max(1.0)) / (natural - gaps).max(1.0);
            for column in &mut widths {
                *column *= scale;
            }
        }
        let paint_line = |ui: &mut Ui, cells: &[&str], header: bool| {
            let (rect, response) =
                ui.allocate_exact_size(vec2(width, line_h), egui::Sense::hover());
            let mut x = rect.left();
            for (index, (column, text)) in columns.iter().zip(cells).enumerate() {
                let font = if header {
                    header_font.clone()
                } else {
                    cell_font(column)
                };
                let color = if header {
                    t.color.text_dim
                } else {
                    t.color.text
                };
                let shown = middle_elided(ui, text, &font, color, widths[index]);
                ui.painter().text(
                    pos2(x, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    shown,
                    font,
                    color,
                );
                x += widths[index] + TABLE_COLUMN_GAP;
            }
            if header {
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom() - 0.5,
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
            let spoken = if header {
                cells.join(", ")
            } else {
                columns
                    .iter()
                    .zip(cells)
                    .map(|(column, cell)| format!("{}: {cell}", column.header))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            response.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &spoken)
            });
        };
        let headers = columns
            .iter()
            .map(|column| column.header)
            .collect::<Vec<_>>();
        paint_line(self.ui, &headers, true);
        for row in rows {
            let cells = row.iter().map(String::as_str).collect::<Vec<_>>();
            paint_line(self.ui, &cells, false);
        }
    }
}

/// One column of a [`FormRows::table`]: its heading, and whether its entries
/// are identifiers, set in mono, or words.
#[derive(Debug, Clone, Copy)]
pub struct TableColumn<'a> {
    pub header: &'a str,
    pub mono: bool,
}

/// Which mark a [`mark_control`] paints.
#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkKind {
    Check,
    Radio,
}

/// How tall one line of marks or list entries is: a fine pointer's short line,
/// or a full control under a finger.
fn mark_line_height(ui: &Ui) -> f32 {
    let t = Tokens::get(ui.ctx());
    if t.metrics.is_touch() {
        t.metrics.ctl_h
    } else {
        MARK_LINE_H.min(t.metrics.ctl_h)
    }
}

/// The air a set of marks gives up at its outer edges. A mark line is taller
/// than what it holds, so its words have room above and below; between two
/// lines that room is the line spacing, but over the first line and under
/// the last it would add to the gap every row keeps, and a set of checks
/// that ends a form would leave more air under it than the form has over its
/// first row. Under a finger the whole line is the target, and keeps it.
fn mark_block_trim(ui: &Ui, line_h: f32) -> f32 {
    if Tokens::get(ui.ctx()).metrics.is_touch() {
        return 0.0;
    }
    let ink = mark_galley(ui, "Ag").size().y.max(MARK_SIDE);
    ((line_h - ink) * 0.5).floor().max(0.0)
}

/// Lay out line `index` of a set of `lines` marks: `line_h` tall where it
/// meets another line, less `trim` at the set's outer edges. Returns the
/// whole line, centred where it would be untrimmed, for its mark and words
/// to be placed in.
fn allocate_mark_line(
    ui: &mut Ui,
    width: f32,
    line_h: f32,
    trim: f32,
    index: usize,
    lines: usize,
) -> Rect {
    let top = if index == 0 { trim } else { 0.0 };
    let bottom = if index + 1 == lines { trim } else { 0.0 };
    let (rect, _) =
        ui.allocate_exact_size(vec2(width, line_h - top - bottom), egui::Sense::hover());
    Rect::from_min_size(pos2(rect.left(), rect.top() - top), vec2(width, line_h))
}

/// The words of one mark, as they are painted.
fn mark_galley(ui: &Ui, words: &str) -> std::sync::Arc<egui::Galley> {
    let t = Tokens::get(ui.ctx());
    ui.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            words.to_owned(),
            theme::sans(tokens::FS_1, FontWeight::Regular),
            t.color.text,
        )
    })
}

/// How many columns a set of marks takes in `width`, and how wide each is:
/// as many as fit at the widest item's width, never more than there are items.
fn mark_columns(ui: &Ui, words: &[&str], width: f32) -> (usize, f32) {
    let column_w = words
        .iter()
        .map(|words| MARK_SIDE + MARK_GAP + mark_galley(ui, words).size().x)
        .fold(0.0_f32, f32::max);
    let fit = ((width + MARK_COLUMN_GAP) / (column_w + MARK_COLUMN_GAP)).floor() as usize;
    (fit.clamp(1, words.len().max(1)), column_w)
}

/// One check box or radio button with its words, from `left` on `line`.
///
/// Painted rather than borrowed from egui, whose marks take the full control
/// height and fill with the selection colour: a mark here is the text's own
/// colour on the field well, so a form's only colour stays its primary.
fn mark_control(
    ui: &mut Ui,
    id: Id,
    left: f32,
    line: Rect,
    words: &str,
    on: bool,
    kind: MarkKind,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let galley = mark_galley(ui, words);
    let hit = Rect::from_min_max(
        pos2(left, line.top()),
        pos2(
            (left + MARK_SIDE + MARK_GAP + galley.size().x).min(line.right()),
            line.bottom(),
        ),
    );
    let response = ui.interact(hit, id, egui::Sense::click());
    let enabled = ui.is_enabled();
    let kind_info = match kind {
        MarkKind::Check => egui::WidgetType::Checkbox,
        MarkKind::Radio => egui::WidgetType::RadioButton,
    };
    response.widget_info(|| egui::WidgetInfo::selected(kind_info, enabled, on, words));
    if !ui.is_rect_visible(hit) {
        return response;
    }
    let mark = Rect::from_center_size(
        pos2(left + MARK_SIDE * 0.5, line.center().y),
        vec2(MARK_SIDE, MARK_SIDE),
    );
    let border = if enabled && response.hovered() {
        t.color.border_strong
    } else {
        t.color.border
    };
    let painter = ui.painter();
    match kind {
        MarkKind::Check => {
            painter.rect(
                mark,
                3.0,
                t.color.bg_inset,
                egui::Stroke::new(1.0, border),
                egui::StrokeKind::Inside,
            );
            if on {
                // A tick on the 14-unit box: down to the base, then up and out.
                let at = |x: f32, y: f32| pos2(mark.left() + x, mark.top() + y);
                painter.add(egui::Shape::line(
                    vec![at(3.5, 7.2), at(6.0, 9.8), at(10.6, 4.4)],
                    egui::Stroke::new(1.6, t.color.text),
                ));
            }
        }
        MarkKind::Radio => {
            painter.circle(
                mark.center(),
                MARK_SIDE * 0.5 - 0.5,
                t.color.bg_inset,
                egui::Stroke::new(1.0, border),
            );
            if on {
                painter.circle_filled(mark.center(), 3.5, t.color.text);
            }
        }
    }
    painter.galley(
        pos2(
            mark.right() + MARK_GAP,
            line.center().y - galley.size().y * 0.5,
        ),
        galley,
        t.color.text,
    );
    theme::paint_focus_ring_outset(ui, &response, mark);
    response
}

/// One list entry: its value, shortened from the middle when it is longer
/// than the line, and its remove button at the line's end. Returns whether
/// the remove button was pressed.
fn list_entry(ui: &mut Ui, id: Id, entry: &str, remove_label: &str, width: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let line_h = mark_line_height(ui);
    ui.push_id(id, |ui| {
        ui.allocate_ui_with_layout(
            vec2(width, line_h),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let (rect, response) = ui.allocate_exact_size(
                    vec2((width - line_h).max(1.0), line_h),
                    egui::Sense::hover(),
                );
                let font = theme::mono(tokens::FS_1, FontWeight::Regular);
                let shown =
                    middle_elided(ui, entry, &font, t.color.text, rect.width() - VALUE_INSET);
                let elided = shown != entry;
                ui.painter().text(
                    pos2(rect.left() + VALUE_INSET, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    shown,
                    font,
                    t.color.text,
                );
                let value = entry.to_owned();
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_role(egui::accesskit::Role::Label);
                    node.set_value(value.clone());
                });
                if elided {
                    response.on_hover_text(entry);
                }
                icon_button(
                    ui,
                    WorkbenchIcon::Close,
                    &format!("{remove_label} {entry}"),
                    false,
                    vec2(line_h, line_h),
                )
                .clicked()
            },
        )
        .inner
    })
    .inner
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
pub(crate) fn middle_elided(
    ui: &Ui,
    text: &str,
    font: &egui::FontId,
    color: Color32,
    width: f32,
) -> String {
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
    // Keep a complete final path component where it fits: the cell-view kind
    // is more useful than an arbitrary half of the trailing text.
    let final_component = text
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(text)
        .chars()
        .count();
    let elide = |keep: usize| -> String {
        let balanced_tail = keep / 2;
        let tail = if final_component < chars.len() && final_component < keep {
            balanced_tail.max(final_component)
        } else {
            balanced_tail
        };
        let head = keep - tail;
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
pub struct CommandForm<'a> {
    title: &'a str,
    primary: &'a str,
    secondary: Option<&'a str>,
    secondary_enabled: bool,
    describe: Option<&'a str>,
    primary_enabled: bool,
    fields_enabled: bool,
    label_col: f32,
    status: Result<Option<&'a str>, &'a str>,
    consequence: Option<&'a str>,
    primary_on_enter: bool,
    cancel_enabled: bool,
    size: DialogSize,
}

impl<'a> CommandForm<'a> {
    pub fn new(title: &'a str, primary: &'a str) -> Self {
        Self {
            title,
            primary,
            secondary: None,
            secondary_enabled: true,
            describe: None,
            primary_enabled: true,
            fields_enabled: true,
            label_col: form::LABEL_COL,
            status: Ok(None),
            consequence: None,
            primary_on_enter: true,
            cancel_enabled: true,
            size: DialogSize::CommandForm,
        }
    }

    /// The 480-point card, for a form whose last row is a table of what the
    /// command will make or keep, which the 420-point card would elide.
    pub fn wide(mut self) -> Self {
        self.size = DialogSize::Confirmation;
        self
    }

    /// Hold Cancel off while the command is finishing what it started — a
    /// save in flight — so leaving cannot abandon it half-done. Esc still
    /// reports [`DialogChoice::Cancelled`]; the caller ignores it meanwhile.
    pub fn cancel_enabled(mut self, enabled: bool) -> Self {
        self.cancel_enabled = enabled;
        self
    }

    /// What the primary will cost that a reader would want to know first —
    /// values it discards. Stated in the footer behind the warning mark,
    /// unless the status refuses, which says more.
    pub fn consequence(mut self, consequence: Option<&'a str>) -> Self {
        self.consequence = consequence;
        self
    }

    /// Hand Enter to the body, for a form holding a
    /// [`FormRows::text_multiline`] row where Enter is a line break.
    pub fn primary_on_enter(mut self, on: bool) -> Self {
        self.primary_on_enter = on;
        self
    }

    /// A plain action beside the primary, for a form whose primary does one
    /// more thing than committing the draft — Save beside Save & compile.
    pub fn secondary(mut self, label: &'a str) -> Self {
        self.secondary = Some(label);
        self
    }

    pub fn secondary_enabled(mut self, enabled: bool) -> Self {
        self.secondary_enabled = enabled;
        self
    }

    /// Widen the label column for a form whose longest label does not fit
    /// the shared one. Every row of the form still starts its control at one
    /// x; only that x moves.
    pub fn label_column(mut self, width: f32) -> Self {
        self.label_col = width.max(form::LABEL_COL);
        self
    }

    /// The sentence assistive technology reads for the whole surface.
    pub fn describe(mut self, description: &'a str) -> Self {
        self.describe = Some(description);
        self
    }

    pub fn primary_enabled(mut self, enabled: bool) -> Self {
        self.primary_enabled = enabled;
        self
    }

    /// Disable every row, for a form that can still be read but not filled —
    /// a read-only document, or one that changed under an open form.
    pub fn fields_enabled(mut self, enabled: bool) -> Self {
        self.fields_enabled = enabled;
        self
    }

    /// The form's status, stated in the footer left of the actions:
    /// `Ok(None)` says nothing, `Ok(Some(_))` states a neutral fact, `Err(_)`
    /// states why the primary is refused, behind a failure mark. The footer is
    /// there whether or not it says anything, so nothing moves as a draft
    /// becomes valid, and the body ends at its last row.
    pub fn status(mut self, status: Result<Option<&'a str>, &'a str>) -> Self {
        self.status = status;
        self
    }

    /// Show the form. `body` returns the control that takes initial focus.
    pub fn show(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut FormRows<'_>) -> Option<Id>,
    ) -> DialogChoice {
        let status = self.status;
        let fields_enabled = self.fields_enabled;
        let label_col = self.label_col;
        // Cancel is the only word any form in this family has for leaving.
        // Enter commits, except in a form that said its body owns the key.
        let mut dialog = Dialog::new(self.title, self.primary)
            .size(self.size)
            .overlay_scrollbar()
            .ghost("Cancel")
            .ghost_enabled(self.cancel_enabled)
            .primary_enabled(self.primary_enabled)
            .primary_on_enter(self.primary_on_enter)
            .initial_focus(DialogInitialFocus::BodyControl);
        if let Some(secondary) = self.secondary {
            dialog = dialog
                .secondary(secondary)
                .secondary_enabled(self.secondary_enabled);
        }
        if let Some(describe) = self.describe {
            dialog = dialog.description(describe);
        }
        match (status, self.consequence) {
            (Err(refusal), _) => {
                dialog = dialog.hint(refusal).hint_tone(DialogHintTone::Error);
            }
            (Ok(_), Some(consequence)) => {
                dialog = dialog.hint(consequence).hint_tone(DialogHintTone::Warn);
            }
            (Ok(Some(fact)), None) => dialog = dialog.hint(fact),
            (Ok(None), None) => {}
        }
        let mut focused_field = None;
        let choice = dialog.show_with_initial_body_focus(ctx, |ui| {
            // Every gap in this body is stated, so nothing is the sum of an
            // explicit space and egui's own inter-widget one.
            ui.spacing_mut().item_spacing.y = 0.0;
            let requested = {
                if !fields_enabled {
                    ui.disable();
                }
                let mut rows = FormRows::new(&mut *ui, label_col);
                body(&mut rows)
            };
            focused_field = requested;
            requested
        });
        if let Err(message) = status {
            mark_invalid(ctx, focused_field, message);
        }
        choice
    }
}

/// Mark the form's principal field invalid, and give it the refusal as its
/// description, so the sentence in the footer reaches a screen reader that
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
    use crate::tokens::Mode;
    use crate::widgets::painted_runs::painted_runs;

    /// A fixture body of the shape every command form in this family has: one
    /// text row with a derived line, two segmented rows, one select.
    fn fixture(ctx: &Context, status: Result<Option<&str>, &str>) -> DialogChoice {
        let mut name = String::from("VERY_LONG_INTERFACE_NAME_1 VERY_LONG_INTERFACE_NAME_2");
        let mut direction = 0usize;
        let mut signal = 0usize;
        CommandForm::new("Command form", "Place")
            .describe("A fixture of the command form's row vocabulary.")
            .status(status)
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
    /// at, in either theme, with the status at its longest.
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
                painted.assert_inside_clip_and_surface_with_horizontal_scroll(
                    &format!("{screen:?} {mode:?}"),
                    &["VERY_LONG_INTERFACE_NAME_1 VERY_LONG_INTERFACE_NAME_2"],
                );
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

    /// The status costs the card nothing whether or not it says anything, so
    /// a form that becomes invalid does not move its own footer out from
    /// under the pointer.
    #[test]
    fn the_footer_holds_still_between_valid_and_invalid() {
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

    /// A refusal that belongs to no field is a failure mark in front of dim
    /// text in the footer, never red words; a neutral fact is faint and
    /// unmarked.
    #[test]
    fn a_form_level_refusal_is_a_failure_mark_before_neutral_text() {
        fn status_paint(
            status: Result<Option<&str>, &str>,
            message: &str,
        ) -> (egui::epaint::TextShape, Vec<egui::Rect>) {
            fn walk(
                shape: &egui::Shape,
                message: &str,
                text: &mut Option<egui::epaint::TextShape>,
                segments: &mut Vec<egui::Rect>,
            ) {
                match shape {
                    egui::Shape::Vec(shapes) => {
                        for shape in shapes {
                            walk(shape, message, text, segments);
                        }
                    }
                    egui::Shape::Text(painted) if painted.galley.text() == message => {
                        *text = Some(painted.clone());
                    }
                    egui::Shape::LineSegment { .. } => segments.push(shape.visual_bounding_rect()),
                    _ => {}
                }
            }
            let ctx = Context::default();
            crate::Theme::default().apply(&ctx);
            // The card fades in, and a faded colour is not the authored one.
            ctx.all_styles_mut(|style| style.animation_time = 0.0);
            let input = || egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1024.0, 640.0),
                )),
                ..Default::default()
            };
            let mut shapes = Vec::new();
            for _ in 0..3 {
                shapes = ctx
                    .run_ui(input(), |ctx| {
                        let _ =
                            CommandForm::new("Import", "Import")
                                .status(status)
                                .show(ctx, |rows| {
                                    rows.value_mono("File", "pair.va");
                                    None
                                });
                    })
                    .shapes;
            }
            let mut text = None;
            let mut segments = Vec::new();
            for clipped in &shapes {
                walk(&clipped.shape, message, &mut text, &mut segments);
            }
            (text.expect("the status is painted"), segments)
        }

        let palette = Tokens::default().color;
        let refusal = "The module declares no ports";
        let (text, segments) = status_paint(Err(refusal), refusal);
        assert_eq!(text.fallback_color, palette.text_dim);
        let first_line = text.galley.rows[0].rect().translate(text.pos.to_vec2());
        let mark = segments
            .iter()
            .filter(|segment| {
                segment.right() <= text.pos.x
                    && (segment.center().y - first_line.center().y).abs() < 4.0
            })
            .count();
        assert_eq!(mark, 2, "a refusal carries the failure cross: {segments:?}");

        let fact = "One module, two ports";
        let (text, segments) = status_paint(Ok(Some(fact)), fact);
        assert_eq!(text.fallback_color, palette.text_faint);
        assert!(
            !segments.iter().any(|segment| segment.right() <= text.pos.x
                && (segment.center().y - text.pos.y).abs() < DERIVED_LINE_H),
            "a neutral fact is unmarked: {segments:?}"
        );
    }

    /// Where a form's body ink starts below the header's rule, and ends above
    /// the footer's: the air a reader sees over the first row and under the
    /// last.
    fn body_air(pass: impl FnMut(&Context)) -> (f32, f32) {
        fn walk(shape: &egui::Shape, into: &mut Vec<(Rect, bool)>) {
            match shape {
                egui::Shape::Vec(shapes) => {
                    for shape in shapes {
                        walk(shape, into);
                    }
                }
                egui::Shape::LineSegment { points, .. } => into.push((
                    Rect::from_two_pos(points[0], points[1]),
                    (points[0].y - points[1].y).abs() < 0.5,
                )),
                egui::Shape::Rect(rect) if rect.blur_width == 0.0 => {
                    let inked =
                        rect.fill.a() > 0 || (rect.stroke.width > 0.0 && rect.stroke.color.a() > 0);
                    if inked {
                        into.push((rect.rect, false));
                    }
                }
                egui::Shape::Text(text) => {
                    into.push((Rect::from_min_size(text.pos, text.galley.size()), false));
                }
                egui::Shape::Path(_) | egui::Shape::Circle(_) => {
                    into.push((shape.visual_bounding_rect(), false));
                }
                _ => {}
            }
        }
        let screen = egui::vec2(1024.0, 640.0);
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let input = || egui::RawInput {
            screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, screen)),
            ..Default::default()
        };
        let mut pass = pass;
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx.run_ui(input(), |ctx| pass(ctx)).shapes;
        }
        let mut ink = Vec::new();
        for clipped in &shapes {
            walk(&clipped.shape, &mut ink);
        }
        // The card is the widest opaque rect short of the scrim; its rules
        // are the horizontal lines that cross it.
        let card = ink
            .iter()
            .filter(|(rect, rule)| !rule && rect.width() < screen.x - 1.0)
            .map(|(rect, _)| *rect)
            .fold(Rect::NOTHING, |widest, rect| {
                if rect.width() > widest.width() {
                    rect
                } else {
                    widest
                }
            });
        let rules: Vec<f32> = ink
            .iter()
            .filter(|(rect, rule)| *rule && rect.width() >= card.width() - 2.0)
            .map(|(rect, _)| rect.center().y)
            .collect();
        let header = rules.iter().copied().fold(f32::INFINITY, f32::min);
        let footer = rules.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let (mut top, mut bottom) = (f32::INFINITY, f32::NEG_INFINITY);
        for (rect, _) in &ink {
            if rect.width() >= card.width() - 2.0
                || rect.top() <= header + 0.5
                || rect.bottom() >= footer - 0.5
            {
                continue;
            }
            top = top.min(rect.top());
            bottom = bottom.max(rect.bottom());
        }
        (top - header, footer - bottom)
    }

    /// The body ends at its last row: the air under it is the air over the
    /// first, whether the status says nothing or refuses, and whether the
    /// last row is a control or a set of checks.
    #[test]
    fn the_air_under_the_last_row_matches_the_air_over_the_first() {
        for status in [
            Ok(None),
            Err("EN: an interface port named 'EN' already exists"),
        ] {
            let (over, under) = body_air(|ctx| {
                let _ = fixture(ctx, status);
            });
            assert!(
                (over - under).abs() <= 1.0,
                "{status:?}: {over} over the first row, {under} under the last"
            );
        }
        let (over, under) = body_air(|ctx| {
            let (mut grid, mut border) = (true, false);
            let mut name = String::from("ota");
            let _ = CommandForm::new("Display", "Apply").show(ctx, |rows| {
                let field = rows.text("Name", Id::new("command-form.air.name"), &mut name, "");
                rows.checks("Show", &mut [("Grid", &mut grid), ("Border", &mut border)]);
                Some(field.id)
            });
        });
        assert!(
            (over - under).abs() <= 1.0,
            "checks last: {over} over the first row, {under} under the last"
        );
    }

    /// A field's line is one line whatever it says, so a field that becomes
    /// invalid, or valid again, moves nothing below it.
    #[test]
    fn a_field_line_holds_its_height_whether_blank_derived_or_refused() {
        let screen = egui::vec2(1024.0, 640.0);
        let render = |line: FieldLine<'_>| {
            painted_runs(screen, Mode::Dark, 3, |ctx| {
                let mut name = String::from("ota");
                let _ = CommandForm::new("Field lines", "Create").show(ctx, |rows| {
                    let id = Id::new("command-form.field-line.name");
                    rows.text("Name", id, &mut name, "");
                    rows.field_line(Some(id), line);
                    Some(id)
                });
            })
        };
        let blank = render(FieldLine::Blank);
        let derived = render(FieldLine::Derived("module ota"));
        let refused = render(FieldLine::Error(
            "a refusal far longer than the control column can hold on one line of it, which is shortened from the middle",
        ));
        for painted in [&derived, &refused] {
            assert_eq!(painted.surface.height(), blank.surface.height());
            assert_eq!(
                painted.rect_of("Create").map(|rect| rect.top()),
                blank.rect_of("Create").map(|rect| rect.top()),
            );
        }
        assert!(
            refused
                .rect_of("a refusal far longer than the control column can hold on one line of it, which is shortened from the middle")
                .is_none(),
            "a refusal too long for the column is elided, not wrapped"
        );
        let unreserved = painted_runs(screen, Mode::Dark, 3, |ctx| {
            let mut name = String::from("ota");
            let _ = CommandForm::new("Field lines", "Create").show(ctx, |rows| {
                let id = Id::new("command-form.field-line.name");
                rows.text("Name", id, &mut name, "");
                Some(id)
            });
        });
        assert!(
            (blank.surface.height() - unreserved.surface.height() - DERIVED_GAP - DERIVED_LINE_H)
                .abs()
                < 0.5,
            "the line costs one line and its gap"
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

    /// A form of every multi-line row: a check set, a choice, a list with a
    /// path too long for a phone's column, and a five-item check set.
    fn mark_and_list_fixture(ctx: &Context) -> DialogChoice {
        let (mut generated, mut jit, mut fallback) = (false, true, 0usize);
        let [
            mut units,
            mut convergence,
            mut discontinuities,
            mut hidden,
            mut portable,
        ] = [true, true, false, true, true];
        let mut draft = String::new();
        let entries = vec![
            "include".to_owned(),
            "vendor/foundry/process/models/verilog_a/include/common".to_owned(),
        ];
        CommandForm::new("Build settings", "Save & compile")
            .secondary("Save")
            .label_column(112.0)
            .show(ctx, |rows| {
                rows.checks(
                    "Targets",
                    &mut [
                        ("Generated Rust", &mut generated),
                        ("Native x86-64 JIT", &mut jit),
                    ],
                );
                rows.choice(
                    "If a target fails",
                    &["Use the interpreter", "Fail the compile"],
                    &mut fallback,
                );
                let id = Id::new("command-form.fixture.paths");
                rows.list(
                    "Include paths",
                    id,
                    &entries,
                    &mut draft,
                    ListWords {
                        hint: "include/models",
                        add: "Add include path",
                        remove: "Remove",
                    },
                );
                rows.field_line(Some(id), FieldLine::Blank);
                rows.checks(
                    "Checks",
                    &mut [
                        ("Units and ranges", &mut units),
                        ("Convergence", &mut convergence),
                        ("Discontinuities", &mut discontinuities),
                        ("Hidden state", &mut hidden),
                        ("Portability", &mut portable),
                    ],
                );
                None
            })
    }

    /// Check sets, choices and lists stay on the card at every viewport, and a
    /// set flows into columns where they fit and stacks where they do not. A
    /// phone's touch-height lines make this form scroll there, which is
    /// allowed; nothing may leave the card or its pane sideways.
    #[test]
    fn mark_sets_and_lists_fit_and_flow_into_columns() {
        for (screen, side_by_side) in [
            (egui::vec2(1024.0, 640.0), true),
            (egui::vec2(390.0, 844.0), false),
        ] {
            for mode in [Mode::Dark, Mode::Light] {
                let painted = painted_runs(screen, mode, 3, |ctx| {
                    let _ = mark_and_list_fixture(ctx);
                });
                painted.assert_inside_clip_and_surface_with_horizontal_scroll(
                    &format!("{screen:?} {mode:?}"),
                    &[],
                );
                let top = |text: &str| {
                    painted
                        .rect_of(text)
                        .unwrap_or_else(|| panic!("{screen:?}: {text} is painted"))
                        .top()
                };
                assert_eq!(
                    (top("Generated Rust") - top("Native x86-64 JIT")).abs() < 0.5,
                    side_by_side,
                    "{screen:?} {mode:?}: the target set's columns"
                );
                assert_eq!(
                    (top("Units and ranges") - top("Convergence")).abs() < 0.5,
                    side_by_side,
                    "{screen:?} {mode:?}: the check set's columns"
                );
                if side_by_side {
                    assert!(
                        top("Discontinuities") > top("Units and ranges"),
                        "{screen:?}: a set wider than its columns wraps to a new line"
                    );
                }
            }
        }
    }

    /// One frame of a form holding one list, with `events` delivered to it:
    /// what the form chose and whether the list asked to add.
    fn list_frame(
        ctx: &Context,
        id: Id,
        draft: &mut String,
        events: Vec<egui::Event>,
    ) -> (DialogChoice, bool) {
        let entries = vec!["include".to_owned()];
        let mut outcome = (DialogChoice::None, false);
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
                let mut add = false;
                let choice = CommandForm::new("Lists", "Commit").show(ctx, |rows| {
                    add = rows
                        .list(
                            "Paths",
                            id,
                            &entries,
                            draft,
                            ListWords {
                                hint: "",
                                add: "Add path",
                                remove: "Remove",
                            },
                        )
                        .add;
                    None
                });
                outcome = (choice, add);
            },
        );
        outcome
    }

    /// Enter in a list's field asks to add what it holds and is spent there,
    /// so a reader filling a list never commits the form by accident; Enter
    /// in an empty field is the form's again.
    #[test]
    fn enter_in_a_list_field_adds_and_never_commits_the_form() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let id = Id::new("command-form.list-enter.field");
        let enter = || egui::Event::Key {
            key: Key::Enter,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        let mut draft = String::from("models");
        list_frame(&ctx, id, &mut draft, Vec::new());
        ctx.memory_mut(|memory| memory.request_focus(id));
        list_frame(&ctx, id, &mut draft, Vec::new());
        let (choice, add) = list_frame(&ctx, id, &mut draft, vec![enter()]);
        assert!(add, "Enter in a filled field asks to add");
        assert_eq!(choice, DialogChoice::None, "and does not commit the form");

        draft.clear();
        ctx.memory_mut(|memory| memory.request_focus(id));
        list_frame(&ctx, id, &mut draft, Vec::new());
        let (choice, add) = list_frame(&ctx, id, &mut draft, vec![enter()]);
        assert!(!add, "an empty field has nothing to add");
        assert_eq!(choice, DialogChoice::Primary, "so Enter is the form's");
    }

    /// A long line is shortened from the middle, so both ends survive.
    #[test]
    fn a_long_value_is_elided_in_the_middle() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
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
                    let width = ui.fonts_mut(|fonts| {
                        fonts.layout_no_wrap(elided, font, Color32::WHITE).size().x
                    });
                    assert!(width <= 120.0, "elided path is {width} points wide");
                });
            },
        );
    }

    /// Every text painted by one settled pass of `pass`, with its ink.
    fn painted_texts(pass: impl FnMut(&Context)) -> Vec<egui::epaint::TextShape> {
        fn walk(shape: &egui::Shape, into: &mut Vec<egui::epaint::TextShape>) {
            match shape {
                egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
                egui::Shape::Text(text) => into.push(text.clone()),
                _ => {}
            }
        }
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.all_styles_mut(|style| style.animation_time = 0.0);
        let input = || egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1024.0, 640.0),
            )),
            ..Default::default()
        };
        let mut pass = pass;
        let mut shapes = Vec::new();
        for _ in 0..3 {
            shapes = ctx.run_ui(input(), |ctx| pass(ctx)).shapes;
        }
        let mut texts = Vec::new();
        for clipped in &shapes {
            walk(&clipped.shape, &mut texts);
        }
        texts
    }

    /// A table starts on the labels' edge, and each column's entries start
    /// where its heading does.
    #[test]
    fn a_table_starts_on_the_labels_edge_and_aligns_its_columns() {
        let painted = painted_runs(egui::vec2(1024.0, 640.0), Mode::Light, 3, |ctx| {
            let mut name = String::from("child");
            let _ = CommandForm::new("Create hierarchy", "Create")
                .wide()
                .show(ctx, |rows| {
                    let id = Id::new("command-form.table.name");
                    rows.text("Cell name", id, &mut name, "");
                    rows.table(
                        &[
                            TableColumn {
                                header: "Port",
                                mono: true,
                            },
                            TableColumn {
                                header: "Direction",
                                mono: false,
                            },
                            TableColumn {
                                header: "Net",
                                mono: true,
                            },
                        ],
                        &[
                            vec!["in".to_owned(), "Input".to_owned(), "net1".to_owned()],
                            vec!["out".to_owned(), "Inout".to_owned(), "out".to_owned()],
                        ],
                    );
                    Some(id)
                });
        });
        painted.assert_inside_clip_and_surface_with_horizontal_scroll("table", &["child"]);
        assert_eq!(painted.surface.width(), 480.0, "a wide form is 480 wide");
        let left = |text: &str| painted.rect_of(text).expect(text).left();
        assert!((left("Port") - left("Cell name")).abs() < 0.5);
        for (header, cells) in [
            ("Port", ["in", "out"]),
            ("Direction", ["Input", "Inout"]),
            ("Net", ["net1", "out"]),
        ] {
            for cell in cells {
                let rect = painted
                    .runs
                    .iter()
                    .filter(|(text, _, _)| text == cell)
                    .map(|(_, rect, _)| *rect)
                    .find(|rect| (rect.left() - left(header)).abs() < 0.5);
                assert!(rect.is_some(), "{cell} does not start under {header}");
            }
        }
    }

    /// A read-only fact reaches assistive technology as the pair it paints,
    /// with the bounds it is painted in: a node without bounds is dropped by
    /// every consumer of the tree.
    #[test]
    fn a_value_row_is_published_with_its_bounds() {
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let input = || egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1024.0, 640.0),
            )),
            ..Default::default()
        };
        let mut nodes = Vec::new();
        for _ in 0..2 {
            let output = ctx.run_ui(input(), |ui| {
                let _ = CommandForm::new("Rename", "Rename").show(ui, |rows| {
                    rows.value("Object", "Resistor R1");
                    None
                });
            });
            nodes = output
                .platform_output
                .accesskit_update
                .expect("AccessKit tree update")
                .nodes;
        }
        let node = nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| node.label() == Some("Object") && node.value() == Some("Resistor R1"))
            .expect("the fact is published as its label and value");
        let bounds = node.bounds().expect("the fact is published with bounds");
        assert!(bounds.x1 > bounds.x0 && bounds.y1 > bounds.y0);
    }

    /// An empty field's hint is set in the faint tone, below the labels, so it
    /// cannot be read as a value already typed. egui's own derived tone put
    /// it darker than the labels in light mode and brighter in dark.
    #[test]
    fn a_hint_is_set_in_the_faint_tone() {
        let hint = "name[msb:lsb]";
        let texts = painted_texts(|ctx| {
            let mut bus = String::new();
            let _ = CommandForm::new("Place bus tap", "Place").show(ctx, |rows| {
                let field = rows.text("Bus", Id::new("command-form.hint.bus"), &mut bus, hint);
                Some(field.id)
            });
        });
        let painted = texts
            .iter()
            .find(|text| text.galley.job.text == hint)
            .expect("the hint is painted");
        let faint = Tokens::default().color.text_faint;
        assert!(
            painted
                .galley
                .job
                .sections
                .iter()
                .all(|section| section.format.color == faint),
            "the hint is not in the faint tone: {:?}",
            painted
                .galley
                .job
                .sections
                .iter()
                .map(|section| section.format.color)
                .collect::<Vec<_>>()
        );
    }

    /// A consequence is stated behind the warning mark in dim text, and a
    /// refusal, which says more, takes its place.
    #[test]
    fn a_consequence_is_marked_and_gives_way_to_a_refusal() {
        let consequence = "2 authored parameter values are discarded.";
        let refusal = "The schematic changed. Close and reopen Replace instance.";
        let form = |status: Result<Option<&'static str>, &'static str>| {
            painted_texts(move |ctx| {
                let _ = CommandForm::new("Replace instance", "Replace")
                    .status(status)
                    .consequence(Some(consequence))
                    .show(ctx, |rows| {
                        rows.value("Current", "Resistor R1");
                        None
                    });
            })
        };
        let warned = form(Ok(None));
        let text = warned
            .iter()
            .find(|text| text.galley.job.text == consequence)
            .expect("the consequence is stated");
        assert_eq!(text.fallback_color, Tokens::default().color.text_dim);
        let refused = form(Err(refusal));
        assert!(refused.iter().any(|text| text.galley.job.text == refusal));
        assert!(
            !refused
                .iter()
                .any(|text| text.galley.job.text == consequence)
        );
    }
}
