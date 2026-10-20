//! The label/value grid the Help dialogs state their facts on.
//!
//! One label column, every value from one x, every row a control tall with
//! one gap between rows, so a dialog's facts read down two edges whatever a
//! row holds: words, an identifier, a control, or a multi-line field. A note
//! that reads off a row sits under it, from the value edge.

use egui::{Align, Color32, Layout, RichText, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// The label column.
const LABEL_COLUMN: f32 = 96.0;
/// Air between the label column and the values.
const LABEL_GAP: f32 = 12.0;
/// Where every value starts, from the grid's left edge.
const VALUE_X: f32 = LABEL_COLUMN + LABEL_GAP;
/// Air between two rows.
const ROW_GAP: f32 = 4.0;
/// Air between a row and the note under it.
const NOTE_GAP: f32 = 2.0;
/// Air between two things on one row.
const ITEM_GAP: f32 = 8.0;

/// The rows of one grid. See [`show`].
pub(in crate::workbench::app::dialogs) struct Rows<'u> {
    ui: &'u mut Ui,
    started: bool,
    /// The first row is words centred in a row, not a control drawn from
    /// the row's top.
    starts_with_air: bool,
    ends_in_note: bool,
}

/// Lay out `rows` on the grid, from the left edge of `ui`.
///
/// A grid that opens with a row of words and ends in a note closes with the
/// air that row keeps above its words, so the words end as far above the
/// body's edge as they begin below it.
pub(in crate::workbench::app::dialogs) fn show<R>(
    ui: &mut Ui,
    rows: impl FnOnce(&mut Rows<'_>) -> R,
) -> R {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let mut grid = Rows {
            ui,
            started: false,
            starts_with_air: false,
            ends_in_note: false,
        };
        let inner = rows(&mut grid);
        if grid.ends_in_note && grid.starts_with_air {
            let air = row_air(grid.ui);
            grid.ui.add_space(air);
        }
        inner
    })
    .inner
}

/// The air above and below the words of a row a control tall: what a
/// block of the dialog that is not a row keeps above or below it to line up
/// with the rows' words rather than their edges.
pub(in crate::workbench::app::dialogs) fn row_air(ui: &Ui) -> f32 {
    let line =
        ui.fonts_mut(|fonts| fonts.row_height(&theme::sans(tokens::FS_1, FontWeight::Regular)));
    ((Tokens::get(ui.ctx()).metrics.ctl_h - line) / 2.0).max(0.0)
}

impl Rows<'_> {
    /// Open a row: the gap above it, unless it is the first, which says
    /// whether the grid opens on words (`air`) or on a control's edge.
    fn gap(&mut self, air: bool) {
        if self.started {
            self.ui.add_space(ROW_GAP);
        } else {
            self.starts_with_air = air;
        }
        self.started = true;
        self.ends_in_note = false;
    }

    /// One row: `label` in the label column, then what `value` adds, left to
    /// right and centred on a control's height.
    pub(in crate::workbench::app::dialogs) fn row<R>(
        &mut self,
        label: &str,
        value: impl FnOnce(&mut Ui) -> R,
    ) -> R {
        self.gap(true);
        let height = Tokens::get(self.ui.ctx()).metrics.ctl_h;
        self.ui
            .horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                label_cell(ui, label, height);
                ui.add_space(LABEL_GAP);
                ui.allocate_ui_with_layout(
                    vec2(ui.available_width(), height),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = ITEM_GAP;
                        ui.set_min_height(height);
                        value(ui)
                    },
                )
                .inner
            })
            .inner
    }

    /// A row whose value is taller than one line, such as a multi-line
    /// field: its label stands beside the value's first line.
    pub(in crate::workbench::app::dialogs) fn block<R>(
        &mut self,
        label: &str,
        value: impl FnOnce(&mut Ui, f32) -> R,
    ) -> R {
        self.gap(false);
        let height = Tokens::get(self.ui.ctx()).metrics.ctl_h;
        self.ui
            .horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                label_cell(ui, label, height);
                ui.add_space(LABEL_GAP);
                let width = ui.available_width();
                ui.vertical(|ui| value(ui, width)).inner
            })
            .inner
    }

    /// A row stating `value` in words.
    pub(in crate::workbench::app::dialogs) fn text(&mut self, label: &str, value: &str) {
        self.fact(label, value, theme::sans(tokens::FS_1, FontWeight::Regular));
    }

    /// A row stating an identifier, a path or a version, set in mono.
    pub(in crate::workbench::app::dialogs) fn mono(&mut self, label: &str, value: &str) {
        self.fact(label, value, theme::mono(tokens::FS_1, FontWeight::Regular));
    }

    fn fact(&mut self, label: &str, value: &str, font: egui::FontId) {
        self.row(label, |ui| {
            let color = Tokens::get(ui.ctx()).color.text;
            ui.add(egui::Label::new(RichText::new(value).font(font).color(color)).selectable(true));
        });
    }

    /// A line under the row above, from the value edge, in the faint tone:
    /// what that row's value means or holds. It wraps rather than elides.
    pub(in crate::workbench::app::dialogs) fn note(&mut self, text: &str) {
        let color = Tokens::get(self.ui.ctx()).color.text_faint;
        self.note_toned(text, color);
    }

    /// [`Self::note`] in `color`: a refusal in the error tone.
    pub(in crate::workbench::app::dialogs) fn note_toned(&mut self, text: &str, color: Color32) {
        self.ui.add_space(NOTE_GAP);
        let width = (self.ui.available_width() - VALUE_X).max(1.0);
        self.ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.add_space(VALUE_X);
            ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                ui.set_width(width);
                ui.add(
                    egui::Label::new(
                        RichText::new(text)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(color),
                    )
                    .wrap(),
                );
            });
        });
        self.ends_in_note = true;
    }
}

/// A row's label, centred on a control's height in the label column.
fn label_cell(ui: &mut Ui, label: &str, height: f32) {
    let color = Tokens::get(ui.ctx()).color.text_dim;
    ui.allocate_ui_with_layout(
        vec2(LABEL_COLUMN, height),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_min_size(vec2(LABEL_COLUMN, height));
            if !label.is_empty() {
                ui.label(
                    RichText::new(label)
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(color),
                );
            }
        },
    );
}
