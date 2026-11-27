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

use super::dialog::{
    DIALOG_BODY_INSET, Dialog, DialogChoice, DialogHintTone, DialogInitialFocus, DialogSize,
};
use super::form;
use super::searchable_select::{SelectGroup, searchable_select};
use super::segmented::{SegmentedWidth, segmented};
use super::select::{SelectOutput, select, select_with_disabled_response, select_with_response};

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
/// A typed number with its unit: a dozen characters of mantissa, exponent and
/// sign, and room for a short unit inside the field's end.
const QUANTITY_FIELD_W: f32 = 120.0;
/// Air between a quantity's digits and the unit set inside its field.
const UNIT_GAP: f32 = 4.0;
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
/// Air between a list's add field and its add button: the kit's control
/// gap, as between any two controls on a row.
const LIST_BUTTON_GAP: f32 = tokens::SP_3;
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
    /// What the field's value declares, in words: a derived line in the body
    /// face.
    Derived(&'a str),
    /// What the field's value works out to, when the line is itself a value:
    /// a derived line in the mono face, as values are set.
    Value(&'a str),
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
    /// Air the row above held back under its ink, which the gap to the next
    /// row gives back: see [`Self::check_columns`].
    owed: f32,
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
            owed: 0.0,
        }
    }

    /// Rows laid straight into `ui` on the form's own label column, for a
    /// dialog that sets a form's rows among parts of its own, such as one
    /// step of a review: they read as the rows of every [`CommandForm`].
    pub fn in_ui(ui: &'u mut Ui) -> Self {
        Self::new(ui, form::LABEL_COL)
    }

    /// The air over what comes next: `gap`, and whatever the row above held
    /// back.
    fn gap(&mut self, gap: f32) {
        let owed = std::mem::take(&mut self.owed);
        self.ui.add_space(gap + owed);
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
        self.row_trimmed(label, 0.0, control)
    }

    /// [`Self::row`], laid out `trim` shorter at its top. The label is still
    /// centred where the whole row would centre it, so a control that does
    /// the same keeps its words level with the label.
    fn row_trimmed<R>(
        &mut self,
        label: &str,
        trim: f32,
        control: impl FnOnce(&mut Ui, f32) -> R,
    ) -> R {
        self.row_inset(label, trim, 0.0, control)
    }

    /// [`Self::row_trimmed`], laid out `bottom` shorter at its foot too.
    fn row_inset<R>(
        &mut self,
        label: &str,
        trim: f32,
        bottom: f32,
        control: impl FnOnce(&mut Ui, f32) -> R,
    ) -> R {
        if self.started {
            self.gap(ROW_GAP);
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
        let laid_h = row_h - trim - bottom;
        self.ui
            .allocate_ui_with_layout(
                vec2(full_width, laid_h),
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let (label_rect, _) =
                        ui.allocate_exact_size(vec2(label_col, laid_h), egui::Sense::hover());
                    let whole = Rect::from_min_size(
                        pos2(label_rect.left(), label_rect.top() - trim),
                        vec2(label_col, row_h),
                    );
                    paint_row_label(ui, label, whole);
                    let width = ui.available_width();
                    control(ui, width)
                },
            )
            .inner
    }

    /// A caption over a hairline, opening a named group of the rows after it.
    pub fn section(&mut self, title: &str) {
        if self.started {
            self.gap(ROW_GAP * 2.0);
        }
        self.started = true;
        let t = Tokens::get(self.ui.ctx());
        let width = self.ui.available_width();
        let (rect, response) = self
            .ui
            .allocate_exact_size(vec2(width, DERIVED_LINE_H + 2.0), egui::Sense::hover());
        self.ui.painter().text(
            pos2(rect.left(), rect.top()),
            egui::Align2::LEFT_TOP,
            title,
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text_dim,
        );
        self.ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, self.ui.is_enabled(), title)
        });
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
                    .font(theme::field_mono())
                    .text_color(t.color.text)
                    .hint_text(hint)
                    .vertical_align(Align::Center)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            form::name_control(ui, &response, label);
            select_all_until_focused(ui, &response, value);
            response
        })
    }

    /// [`Self::text`] in the body face, for a name or a description a reader
    /// writes in words beside a select, which shows its value in that face:
    /// one face down the form, the field as tall as the select and ending
    /// where it ends.
    pub fn text_sans(&mut self, label: &str, id: Id, value: &mut String, hint: &str) -> Response {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let response = form::text_field(
                ui,
                egui::TextEdit::singleline(value)
                    .id(id)
                    .font(theme::field_sans())
                    .text_color(t.color.text)
                    .hint_text(hint)
                    .margin(egui::Margin::symmetric(8, 4)),
                width,
            );
            form::name_control(ui, &response, label);
            select_all_until_focused(ui, &response, value);
            response
        })
    }

    /// One typed number in a field sized for a number rather than the whole
    /// control column, its `unit` set inside the field's end, and `note`
    /// after the field: what an empty field stands for, where the hint inside
    /// a field that short cannot hold it. An empty `unit` or `note` sets none.
    pub fn quantity(
        &mut self,
        label: &str,
        id: Id,
        value: &mut String,
        hint: &str,
        unit: &str,
        note: &str,
    ) -> Response {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let font = theme::field_mono();
            let unit_w = if unit.is_empty() {
                0.0
            } else {
                ui.fonts_mut(|fonts| {
                    fonts
                        .layout_no_wrap(unit.to_owned(), font.clone(), t.color.text_dim)
                        .size()
                        .x
                }) + UNIT_GAP
            };
            let response = ui.add_sized(
                vec2(QUANTITY_FIELD_W.min(width), t.metrics.ctl_h),
                egui::TextEdit::singleline(value)
                    .id(id)
                    .font(font.clone())
                    .text_color(t.color.text)
                    .hint_text(hint)
                    .vertical_align(Align::Center)
                    .margin(egui::Margin {
                        left: 8,
                        right: (8.0 + unit_w).ceil().min(f32::from(i8::MAX)) as i8,
                        top: 4,
                        bottom: 4,
                    }),
            );
            if !unit.is_empty() {
                ui.painter().text(
                    pos2(response.rect.right() - 8.0, response.rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    unit,
                    font,
                    t.color.text_dim,
                );
            }
            form::name_control(ui, &response, label);
            select_all_until_focused(ui, &response, value);
            if !note.is_empty() {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(note)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    )
                    .truncate(),
                );
            }
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
            // A drag value has no colour of its own to state; its value is
            // set in the text tone as every typed value is.
            let response = ui
                .scope(|ui| {
                    ui.visuals_mut().override_text_color = Some(t.color.text);
                    ui.add_sized(
                        vec2(NUMBER_FIELD_W, t.metrics.ctl_h),
                        egui::DragValue::new(value).range(range).speed(1.0),
                    )
                })
                .inner;
            form::name_control(ui, &response, label);
            ui.label(
                egui::RichText::new(allowed)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
            response
        })
    }

    /// One multi-line prose field, `lines` controls tall, in the body face.
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
                    .font(theme::field_sans())
                    .text_color(t.color.text)
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
                        .font(theme::field_mono())
                        .text_color(t.color.text)
                        .hint_text(axis)
                        .vertical_align(Align::Center)
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

    /// A name and one choice about it side by side in the control column,
    /// each half of it: a port's name and its direction, say. Returns the
    /// field, and the option picked this frame.
    pub fn text_with_select(
        &mut self,
        label: &str,
        id: Id,
        value: &mut String,
        choice_label: &str,
        selected: &str,
        options: &[String],
    ) -> (Response, Option<usize>) {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let half = ((width - POINT_GAP) * 0.5).max(1.0);
            ui.spacing_mut().item_spacing.x = POINT_GAP;
            let field = ui.add_sized(
                vec2(half, t.metrics.ctl_h),
                egui::TextEdit::singleline(value)
                    .id(id)
                    .font(theme::field_mono())
                    .text_color(t.color.text)
                    .vertical_align(Align::Center)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            form::name_control(ui, &field, label);
            select_all_until_focused(ui, &field, value);
            let salt = format!("{:?}", id.with("choice"));
            let choice = select_with_response(
                ui,
                &salt,
                &format!("{label} {choice_label}"),
                selected,
                options,
                half,
            );
            (field, choice.picked)
        })
    }

    /// One contiguous group of exclusive options across the whole control
    /// column in equal segments: as wide as the fields above it, with its
    /// options standing in the columns a [`Self::check_columns`] row of as
    /// many checks uses. Returns `true` when the selection changed this frame.
    pub fn segmented_columns(
        &mut self,
        label: &str,
        id_salt: &'static str,
        options: &[&str],
        selected: &mut usize,
    ) -> bool {
        self.row(label, |ui, _| {
            segmented(ui, id_salt, options, selected, SegmentedWidth::Equal)
        })
    }

    /// [`Self::segmented_columns`] cut into `columns` equal segments however
    /// few options it holds, the options taking the first of them. Related
    /// rows whose sets differ by an option or two keep each shared option in
    /// one column, so the eye reads straight down them: an edge type and a
    /// second edge type that may also be "same".
    pub fn segmented_in_columns(
        &mut self,
        label: &str,
        id_salt: impl egui::AsIdSalt,
        options: &[&str],
        selected: &mut usize,
        columns: usize,
    ) -> bool {
        self.row(label, |ui, width| {
            let columns = columns.max(options.len()).max(1);
            let span = width * options.len() as f32 / columns as f32;
            let height = Tokens::get(ui.ctx()).metrics.ctl_h;
            ui.allocate_ui_with_layout(
                vec2(span, height),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.push_id(id_salt, |ui| {
                        segmented(ui, "segments", options, selected, SegmentedWidth::Equal)
                    })
                    .inner
                },
            )
            .inner
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

    /// [`Self::select`] with the one action it offers at the row's end, such
    /// as taking its value from somewhere else on screen. Returns the picked
    /// index, and whether the action was pressed.
    pub fn select_with_action(
        &mut self,
        label: &str,
        id_salt: &str,
        selected: &str,
        options: &[String],
        action: &str,
    ) -> (Option<usize>, bool) {
        self.row(label, |ui, width| {
            let t = Tokens::get(ui.ctx());
            ui.allocate_ui_with_layout(
                vec2(width, t.metrics.ctl_h),
                Layout::right_to_left(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let pressed = super::Button::new(action).show(ui).clicked();
                    let field = ui.available_width().max(1.0);
                    let picked = select(ui, id_salt, label, selected, options, field);
                    (picked, pressed)
                },
            )
            .inner
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

    /// [`Self::select_with_response`] with choices the reader can see but not
    /// pick, each listed with the reason it is unavailable.
    pub fn select_with_disabled(
        &mut self,
        label: &str,
        id_salt: &str,
        selected: &str,
        options: &[String],
        disabled: &[(usize, &'static str)],
    ) -> SelectOutput {
        self.row(label, |ui, width| {
            select_with_disabled_response(ui, id_salt, label, selected, options, disabled, width)
        })
    }

    /// One dropdown over a long, grouped list, with a search field at the top
    /// of the list. Returns `(group, option)` when the reader picked one.
    pub fn searchable_select(
        &mut self,
        label: &str,
        id_salt: &str,
        selected: &str,
        groups: &[SelectGroup<'_>],
        query: &mut String,
    ) -> Option<(usize, usize)> {
        self.row(label, |ui, width| {
            searchable_select(ui, id_salt, label, selected, groups, query, width)
        })
    }

    /// One read-only fact, on the same grid as the controls: its label in the
    /// label column, its value where a control's value would start, at the
    /// size a field holds its value. What a form acts on is stated in a row,
    /// never in an unlabelled line above it.
    pub fn value(&mut self, label: &str, value: &str) {
        self.value_row(label, value, theme::field_sans());
    }

    /// [`Self::value`] for a path or an identifier, which is set in mono. A
    /// value too long for the row is shortened from the middle, so both ends
    /// of a path survive.
    pub fn value_mono(&mut self, label: &str, value: &str) {
        self.value_row(label, value, theme::field_mono());
    }

    fn value_row(&mut self, label: &str, value: &str, font: egui::FontId) {
        let trim = if self.started || self.stacked {
            0.0
        } else {
            fact_trim(self.ui, &font)
        };
        self.row_trimmed(label, trim, |ui, width| {
            let t = Tokens::get(ui.ctx());
            let (rect, response) =
                ui.allocate_exact_size(vec2(width, t.metrics.ctl_h - trim), egui::Sense::hover());
            let shown = middle_elided(ui, value, &font, t.color.text, width - VALUE_INSET);
            ui.painter().text(
                egui::pos2(rect.left() + VALUE_INSET, rect.center().y - trim * 0.5),
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
    /// draft, and half of it is not worth having. It is words, so it is set
    /// in the body face; [`Self::derived_mono`] sets names as a deck writes
    /// them.
    pub fn derived(&mut self, text: &str) {
        self.derived_in(text, theme::sans(tokens::FS_0, FontWeight::Regular));
    }

    /// [`Self::derived`] for a line that is names or code as a deck writes
    /// them, such as the conductors a slice resolves to: set in mono.
    pub fn derived_mono(&mut self, text: &str) {
        self.derived_in(text, theme::mono(tokens::FS_0, FontWeight::Regular));
    }

    /// [`Self::derived`] held to one line: a text longer than the control
    /// column is cut at its end, with the whole of it on hover. For prose the
    /// reader reads once, such as what a picked function does, which would
    /// otherwise spend several lines of every form on it.
    pub fn derived_line(&mut self, text: &str) {
        let t = Tokens::get(self.ui.ctx());
        let indent = self.control_indent();
        self.gap(DERIVED_GAP);
        let full_width = self.ui.available_width();
        let width = (full_width - indent).max(1.0);
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat {
                font_id: theme::sans(tokens::FS_0, FontWeight::Regular),
                color: t.color.text_faint,
                ..Default::default()
            },
        );
        job.wrap = egui::text::TextWrapping {
            max_width: width,
            max_rows: 1,
            break_anywhere: false,
            overflow_character: Some('…'),
        };
        let galley = self.ui.fonts_mut(|fonts| fonts.layout_job(job));
        let elided = galley.elided;
        let (rect, response) = self
            .ui
            .allocate_exact_size(vec2(full_width, DERIVED_LINE_H), egui::Sense::hover());
        self.ui.painter().galley(
            pos2(
                rect.left() + indent,
                rect.center().y - galley.size().y * 0.5,
            ),
            galley,
            t.color.text_faint,
        );
        let value = text.to_owned();
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

    fn derived_in(&mut self, text: &str, font_id: egui::FontId) {
        let t = Tokens::get(self.ui.ctx());
        let indent = self.control_indent();
        self.gap(DERIVED_GAP);
        let full_width = self.ui.available_width();
        let width = (full_width - indent).max(1.0);
        let mut job = egui::text::LayoutJob::single_section(
            text.to_owned(),
            egui::TextFormat {
                font_id,
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
        self.gap(DERIVED_GAP);
        let full_width = self.ui.available_width();
        let width = (full_width - indent).max(1.0);
        let (rect, response) = self
            .ui
            .allocate_exact_size(vec2(full_width, DERIVED_LINE_H), egui::Sense::hover());
        let (text, font, color) = match line {
            FieldLine::Blank => return,
            FieldLine::Derived(text) => (
                text,
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_faint,
            ),
            FieldLine::Value(text) => (
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
            self.gap(ROW_GAP);
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
    /// the widest, each an equal share of it, in reading order, so a short set
    /// is one line and a long one is a grid rather than a ragged run, and the
    /// sets of one form share their columns. Returns `true` when any changed.
    pub fn checks(&mut self, label: &str, items: &mut [(&str, &mut bool)]) -> bool {
        let line_h = mark_line_height(self.ui);
        let trim = mark_block_trim(self.ui, line_h);
        let id = self.ui.id().with(("command-form.checks", label));
        self.block(label, line_h - 2.0 * trim, |ui, width| {
            let words = items.iter().map(|(words, _)| *words).collect::<Vec<_>>();
            let (columns, pitch) = mark_columns(ui, &words, width);
            let lines = items.len().div_ceil(columns);
            let mut changed = false;
            for (line_index, line) in items.chunks_mut(columns).enumerate() {
                let mark_line = allocate_mark_line(ui, width, line_h, trim, line_index, lines);
                for (column, (words, value)) in line.iter_mut().enumerate() {
                    let left = mark_line.line.left() + column as f32 * pitch;
                    let item = id.with(line_index * columns + column);
                    if mark_control(ui, item, left, mark_line, words, **value, MarkKind::Check)
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

    /// A set of on/off choices on one line of the form's grid, each check in
    /// an equal share of the control column. Rows of as many checks line up
    /// column by column, and with a [`Self::segmented_columns`] row of as
    /// many options. Where the shares cannot hold the set's widest words, the
    /// set flows as [`Self::checks`] lays it out. Returns `true` when any
    /// changed.
    ///
    /// The row is as tall as a field's, so every row of the form keeps one
    /// pitch. Its words, not its line, meet the body's edges: as the first
    /// row it gives up the air over its ink, as a field's frame starts at the
    /// row's top, and it holds back the air under its ink, which the gap to
    /// the next row gives back. The whole line stays each check's target.
    pub fn check_columns(&mut self, label: &str, items: &mut [(&str, &mut bool)]) -> bool {
        let control_w = self.ui.available_width() - self.label_col - LABEL_GAP;
        let share = control_w / items.len().max(1) as f32;
        let last = items.len().saturating_sub(1);
        let each_fits = items.iter().enumerate().all(|(index, (words, _))| {
            let air = if index == last { 0.0 } else { MARK_COLUMN_GAP };
            MARK_SIDE + MARK_GAP + mark_galley(self.ui, words).size().x + air <= share
        });
        // A set the flow of [`Self::checks`] would lay on one line, in the
        // same equal shares, is laid on one line here too.
        let words = items.iter().map(|(words, _)| *words).collect::<Vec<_>>();
        let flows_on_one_line = mark_columns(self.ui, &words, control_w).0 == items.len();
        let fits = !self.stacked && (each_fits || flows_on_one_line);
        if !fits {
            return self.checks(label, items);
        }
        let row_h = Tokens::get(self.ui.ctx()).metrics.ctl_h;
        // What the row inks: its marks, and its words' glyphs centred with
        // them, not the words' whole line.
        let ink = mark_galley(self.ui, "Ag")
            .mesh_bounds
            .height()
            .max(MARK_SIDE);
        let edge = ((row_h - ink) * 0.5).floor().max(0.0);
        let top = if self.started { 0.0 } else { edge };
        let id = self.ui.id().with(("command-form.check-columns", label));
        let changed = self.row_inset(label, top, edge, |ui, width| {
            let (span, _) =
                ui.allocate_exact_size(vec2(width, row_h - top - edge), egui::Sense::hover());
            // The held-back air is never another row's, so the line is the
            // span each mark answers to.
            let line = Rect::from_min_size(pos2(span.left(), span.top() - top), vec2(width, row_h));
            let mark_line = MarkLine { line, span: line };
            let share = width / items.len() as f32;
            let mut changed = false;
            for (column, (words, value)) in items.iter_mut().enumerate() {
                let left = line.left() + column as f32 * share;
                let item = id.with(column);
                if mark_control(ui, item, left, mark_line, words, **value, MarkKind::Check)
                    .clicked()
                {
                    **value = !**value;
                    changed = true;
                }
            }
            changed
        });
        self.owed = edge;
        changed
    }

    /// One exclusive choice among a few, each its own radio button, laid out
    /// as [`Self::checks`] lays out its set. Returns `true` when the selection
    /// changed this frame.
    pub fn choice(&mut self, label: &str, options: &[&str], selected: &mut usize) -> bool {
        let line_h = mark_line_height(self.ui);
        let trim = mark_block_trim(self.ui, line_h);
        let id = self.ui.id().with(("command-form.choice", label));
        self.block(label, line_h - 2.0 * trim, |ui, width| {
            let (columns, pitch) = mark_columns(ui, options, width);
            let lines = options.len().div_ceil(columns);
            let mut changed = false;
            for (line_index, line) in options.chunks(columns).enumerate() {
                let mark_line = allocate_mark_line(ui, width, line_h, trim, line_index, lines);
                for (column, words) in line.iter().enumerate() {
                    let index = line_index * columns + column;
                    let left = mark_line.line.left() + column as f32 * pitch;
                    let on = *selected == index;
                    if mark_control(
                        ui,
                        id.with(index),
                        left,
                        mark_line,
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
                                .font(theme::field_mono())
                                .text_color(t.color.text)
                                .hint_text(hint)
                                .vertical_align(Align::Center)
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
                    let font = theme::field_sans();
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

    /// [`Self::value_action`] for a value that is a sentence, such as a count
    /// of findings or what is wrong with an instance: it wraps in the room the
    /// action leaves, never cut, and the row grows to hold it, its first line
    /// level with the label.
    pub fn sentence_action(&mut self, label: &str, sentence: &str, action: &str) -> bool {
        let line_h = Tokens::get(self.ui.ctx()).metrics.ctl_h;
        self.block(label, line_h, |ui, width| {
            ui.allocate_ui_with_layout(
                vec2(width, line_h),
                Layout::right_to_left(Align::Min),
                |ui| {
                    ui.spacing_mut().item_spacing.x = LABEL_GAP;
                    let clicked = super::Button::new(action).show(ui).clicked();
                    let t = Tokens::get(ui.ctx());
                    let font = theme::field_sans();
                    let room = ui.available_width().max(1.0);
                    let galley = ui.painter().layout(
                        sentence.to_owned(),
                        font.clone(),
                        t.color.text,
                        (room - VALUE_INSET).max(1.0),
                    );
                    let first_line = ui.fonts_mut(|fonts| fonts.row_height(&font));
                    let air = (line_h - first_line).max(0.0);
                    let (rect, response) = ui.allocate_exact_size(
                        vec2(room, (galley.size().y + air).max(line_h)),
                        egui::Sense::hover(),
                    );
                    ui.painter().galley(
                        pos2(rect.left() + VALUE_INSET, rect.top() + air * 0.5),
                        galley,
                        t.color.text,
                    );
                    let spoken = format!("{label}: {sentence}");
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
    ///
    /// Its lines are read, not pressed, so its words meet the body's edges as
    /// a field's frame does: as the first row it gives up the air over its
    /// headings, and it holds back the air under its last line's words, which
    /// the gap to a next row gives back.
    pub fn table(&mut self, columns: &[TableColumn<'_>], rows: &[Vec<String>]) {
        let first = !self.started;
        if self.started {
            self.gap(ROW_GAP);
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
            // Only the longest columns give up width: a column narrower than
            // an even share of what is left keeps its own, and the rest share
            // the remainder evenly.
            let mut order = (0..widths.len()).collect::<Vec<_>>();
            order.sort_by(|a, b| widths[*a].total_cmp(&widths[*b]));
            let mut left = (width - gaps).max(1.0);
            for (placed, column) in order.iter().enumerate() {
                let share = left / (widths.len() - placed) as f32;
                widths[*column] = widths[*column].min(share);
                left -= widths[*column];
            }
        }
        // The air a line holds over or under its words' ink, set centred.
        let air = |ui: &Ui, cells: &[&str], fonts: &dyn Fn(usize) -> egui::FontId, under: bool| {
            cells
                .iter()
                .enumerate()
                .filter(|(_, cell)| !cell.trim().is_empty())
                .map(|(index, cell)| {
                    let galley = ui.painter().layout_no_wrap(
                        (*cell).to_owned(),
                        fonts(index),
                        Color32::PLACEHOLDER,
                    );
                    let half = (line_h - galley.size().y) * 0.5;
                    if under {
                        half + galley.size().y - galley.mesh_bounds.bottom()
                    } else {
                        half + galley.mesh_bounds.top()
                    }
                })
                .fold(f32::INFINITY, f32::min)
        };
        let edge = |air: f32| {
            if air.is_finite() {
                air.floor().max(0.0)
            } else {
                0.0
            }
        };
        let headers = columns
            .iter()
            .map(|column| column.header)
            .collect::<Vec<_>>();
        let top = if first {
            edge(air(self.ui, &headers, &|_| header_font.clone(), false))
        } else {
            0.0
        };
        // The headings' rule is ink at their line's foot, so a table of no
        // rows keeps that line whole.
        let bottom = rows.last().map_or(0.0, |row| {
            let cells = row.iter().map(String::as_str).collect::<Vec<_>>();
            edge(air(
                self.ui,
                &cells,
                &|index| cell_font(&columns[index]),
                true,
            ))
        });
        let paint_line = |ui: &mut Ui, cells: &[&str], header: bool, top: f32, bottom: f32| {
            let (span, response) =
                ui.allocate_exact_size(vec2(width, line_h - top - bottom), egui::Sense::hover());
            let rect =
                Rect::from_min_size(pos2(span.left(), span.top() - top), vec2(width, line_h));
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
        paint_line(self.ui, &headers, true, top, 0.0);
        for (index, row) in rows.iter().enumerate() {
            let cells = row.iter().map(String::as_str).collect::<Vec<_>>();
            let under = if index + 1 == rows.len() { bottom } else { 0.0 };
            paint_line(self.ui, &cells, false, 0.0, under);
        }
        self.owed = bottom;
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

/// One laid-out line of a set of marks.
#[derive(Clone, Copy)]
struct MarkLine {
    /// The whole line, centred where it would be untrimmed: what its mark and
    /// words are placed in.
    line: Rect,
    /// What the layout gave the line. The mark answers to this and no more,
    /// so a trimmed outer line never reaches into the gap to the next row.
    span: Rect,
}

/// The air a read-only fact gives up when it is a form's first row. A field's
/// frame is its ink and meets its row's top, so the body's padding is the
/// air over the form; a fact's ink is its words, centred in a row as tall as
/// a field, and over the first row the air above them added to that padding.
/// Given up, the words' ink starts at the row's top, where a field's frame
/// would. Between rows a fact keeps its whole row, so the grid's rhythm is
/// the same either way.
fn fact_trim(ui: &Ui, font: &egui::FontId) -> f32 {
    let row_h = Tokens::get(ui.ctx()).metrics.ctl_h;
    let galley =
        ui.painter()
            .layout_no_wrap("Ag".to_owned(), font.clone(), egui::Color32::PLACEHOLDER);
    let ink_top = (row_h - galley.size().y) * 0.5 + galley.mesh_bounds.top();
    ink_top.floor().max(0.0)
}

/// Lay out line `index` of a set of `lines` marks: `line_h` tall where it
/// meets another line, less `trim` at the set's outer edges.
fn allocate_mark_line(
    ui: &mut Ui,
    width: f32,
    line_h: f32,
    trim: f32,
    index: usize,
    lines: usize,
) -> MarkLine {
    let top = if index == 0 { trim } else { 0.0 };
    let bottom = if index + 1 == lines { trim } else { 0.0 };
    let (span, _) =
        ui.allocate_exact_size(vec2(width, line_h - top - bottom), egui::Sense::hover());
    MarkLine {
        line: Rect::from_min_size(pos2(span.left(), span.top() - top), vec2(width, line_h)),
        span,
    }
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

/// How many columns a set of marks takes in `width`, and the pitch between
/// them: as many as fit at the widest item's width, never more than there
/// are items, each an equal share of `width`. A set of two then starts its
/// columns where [`FormRows::check_columns`] starts a row of two, so the
/// sets of one form line up column by column.
fn mark_columns(ui: &Ui, words: &[&str], width: f32) -> (usize, f32) {
    let column_w = words
        .iter()
        .map(|words| MARK_SIDE + MARK_GAP + mark_galley(ui, words).size().x)
        .fold(0.0_f32, f32::max);
    let fit = ((width + MARK_COLUMN_GAP) / (column_w + MARK_COLUMN_GAP)).floor() as usize;
    let columns = fit.clamp(1, words.len().max(1));
    (columns, width / columns as f32)
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
    MarkLine { line, span }: MarkLine,
    words: &str,
    on: bool,
    kind: MarkKind,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let galley = mark_galley(ui, words);
    let hit = Rect::from_min_max(
        pos2(left, span.top()),
        pos2(
            (left + MARK_SIDE + MARK_GAP + galley.size().x).min(line.right()),
            span.bottom(),
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
        MarkKind::Check => paint_check_box(painter, &t, mark, on, border),
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

/// A check box filling `mark`, a 14-unit square: the text's own colour on the
/// field well, ticked while `on`, its edge in `border`. A form's checks and a
/// table's checked rows paint this one box.
pub(crate) fn paint_check_box(
    painter: &egui::Painter,
    t: &Tokens,
    mark: Rect,
    on: bool,
    border: egui::Color32,
) {
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
                let font = theme::field_mono();
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

/// Keep the whole value selected while the field is without the keyboard, so
/// it takes the keyboard with its value selected — when the form opens, or by
/// Tab — and typing replaces a prefilled name. A click still lands the caret
/// where it was aimed: the press places it over this selection.
///
/// egui's own default is a caret at position zero, which for a prefilled name
/// means the reader types in front of a suggestion instead of over it. The
/// selection waits for the keyboard rather than following it in, because the
/// dialog hands its first field the keyboard only after drawing it, so the
/// field never reports gaining focus as a form opens.
fn select_all_until_focused(ui: &Ui, response: &Response, value: &str) {
    if response.has_focus() || value.is_empty() {
        return;
    }
    let mut state = egui::TextEdit::load_state(ui.ctx(), response.id).unwrap_or_default();
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
    cancel: &'a str,
    cancel_enabled: bool,
    size: DialogSize,
    first_field: Option<Id>,
    destructive: bool,
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
            cancel: "Cancel",
            cancel_enabled: true,
            size: DialogSize::CommandForm,
            first_field: None,
            destructive: false,
        }
    }

    /// The field that has the keyboard as the form opens, focused before it
    /// reads the opening frame's input, so what the reader types straight
    /// after the press that opened the form lands in it. The body returns
    /// the same id. Without it the body's nominee takes the keyboard once the
    /// body has drawn, and that frame's typing is lost.
    pub fn first_field(mut self, id: Id) -> Self {
        self.first_field = Some(id);
        self
    }

    /// The 480-point card, for a form whose last row is a table of what the
    /// command will make or keep, which the 420-point card would elide.
    pub fn wide(mut self) -> Self {
        self.size = DialogSize::Confirmation;
        self
    }

    /// The 520-point card, for a form whose option rows set three choices
    /// side by side in shared columns ([`FormRows::check_columns`],
    /// [`FormRows::segmented_columns`]), which the 420-point card would wrap.
    pub fn option_columns(mut self) -> Self {
        self.size = DialogSize::CommandFormColumns;
        self
    }

    /// The 760-point card, for a form over a table of each change the
    /// command makes ([`Self::show_over`]), whose lines the narrower cards
    /// would cut to a few words.
    pub fn transaction(mut self) -> Self {
        self.size = DialogSize::Transaction;
        self
    }

    /// The 600-point reading column, for the properties of a whole object,
    /// whose values are paths and prose the narrower cards would elide.
    pub fn reading_column(mut self) -> Self {
        self.size = DialogSize::AccountManager;
        self
    }

    /// Cancel's word in the language the rest of the form is written in.
    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel = label;
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

    /// Widen the label column to the widest of `labels`, as the rows paint
    /// them, for a form of facts whose names run longer than the shared
    /// column.
    pub fn fit_labels(self, ctx: &Context, labels: &[&str]) -> Self {
        let font = theme::sans(tokens::FS_1, FontWeight::Regular);
        let widest = ctx.fonts_mut(|fonts| {
            labels
                .iter()
                .map(|label| {
                    fonts
                        .layout_no_wrap((*label).to_owned(), font.clone(), Color32::PLACEHOLDER)
                        .size()
                        .x
                })
                .fold(0.0, f32::max)
        });
        self.label_column(widest.ceil())
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

    /// The primary in the destructive treatment, for a form whose command
    /// replaces or removes what the reader has.
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
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
        self.show_parts(ctx, body, None::<fn(&mut Ui)>)
    }

    /// The form over a part that runs edge to edge: its rows keep the
    /// body's inset, as the title and the buttons do, and `below` draws
    /// under them across the whole body, flush to its sides, as a dialog's
    /// table under its band does.
    pub fn show_over(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut FormRows<'_>) -> Option<Id>,
        below: impl FnOnce(&mut Ui),
    ) -> DialogChoice {
        self.show_parts(ctx, body, Some(below))
    }

    fn show_parts(
        self,
        ctx: &Context,
        body: impl FnOnce(&mut FormRows<'_>) -> Option<Id>,
        below: Option<impl FnOnce(&mut Ui)>,
    ) -> DialogChoice {
        let status = self.status;
        let fields_enabled = self.fields_enabled;
        let label_col = self.label_col;
        // Cancel is the only word any form in this family has for leaving.
        // Enter commits, except in a form that said its body owns the key.
        let mut dialog = Dialog::new(self.title, self.primary)
            .size(self.size)
            .overlay_scrollbar()
            .ghost(self.cancel)
            .ghost_enabled(self.cancel_enabled)
            .primary_enabled(self.primary_enabled)
            .primary_on_enter(self.primary_on_enter)
            .initial_focus(
                self.first_field
                    .map_or(DialogInitialFocus::BodyControl, DialogInitialFocus::Control),
            );
        if let Some(secondary) = self.secondary {
            dialog = dialog
                .secondary(secondary)
                .secondary_enabled(self.secondary_enabled);
        }
        if self.destructive {
            dialog = dialog.destructive();
        }
        if let Some(describe) = self.describe {
            dialog = dialog.description(describe);
        }
        if below.is_some() {
            dialog = dialog.flush_body();
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
            let rows = |ui: &mut Ui| {
                if !fields_enabled {
                    ui.disable();
                }
                let mut rows = FormRows::new(ui, label_col);
                body(&mut rows)
            };
            let requested = match below {
                None => rows(ui),
                Some(below) => {
                    // The flush body leaves the rows' inset to the form.
                    let requested = egui::Frame::NONE
                        .inner_margin(egui::Margin::same(DIALOG_BODY_INSET as i8))
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing.y = 0.0;
                            rows(ui)
                        })
                        .inner;
                    below(ui);
                    requested
                }
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

    /// A form opens with its first field's prefilled value selected, so what
    /// the reader types replaces it, and Tab selects the next field's whole
    /// value too. The dialog hands its field the keyboard only after drawing
    /// it, which egui never reports as the field gaining focus.
    #[test]
    fn a_form_opens_with_its_prefilled_value_selected_for_typing_over() {
        fn frame(ctx: &Context, name: &mut String, width: &mut String, events: Vec<egui::Event>) {
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
                    let _ = CommandForm::new("Rename", "Rename").show(ctx, |rows| {
                        let field = Id::new("command-form.open-select.name");
                        rows.text("Name", field, name, "");
                        rows.quantity(
                            "Width",
                            Id::new("command-form.open-select.width"),
                            width,
                            "",
                            "m",
                            "",
                        );
                        Some(field)
                    });
                },
            );
        }
        let ctx = Context::default();
        crate::Theme::default().apply(&ctx);
        let mut name = String::from("old_name");
        let mut width = String::from("10u");
        for _ in 0..3 {
            frame(&ctx, &mut name, &mut width, Vec::new());
        }
        frame(
            &ctx,
            &mut name,
            &mut width,
            vec![egui::Event::Text("amp".to_owned())],
        );
        assert_eq!(name, "amp", "typing replaced the prefilled name");

        let tab = egui::Event::Key {
            key: Key::Tab,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        frame(&ctx, &mut name, &mut width, vec![tab]);
        frame(&ctx, &mut name, &mut width, Vec::new());
        frame(
            &ctx,
            &mut name,
            &mut width,
            vec![egui::Event::Text("2".to_owned())],
        );
        assert_eq!(width, "2", "Tab selected the next field's whole value");
        assert_eq!(name, "amp", "and left the first field as typed");
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
