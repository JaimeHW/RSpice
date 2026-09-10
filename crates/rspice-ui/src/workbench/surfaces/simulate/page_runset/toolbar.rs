//! Run Set commands and responsive status layout.

use super::super::page_kit::{self, CARD_PAD_X};
use super::{RSpiceApp, RunSetAction, RunSetValidation, commit};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::Button;
use egui::Ui;
pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, validation: &RunSetValidation) {
    let t = Tokens::get(ui.ctx());
    let addable = app.state.sim_setup.run_set.addable_kinds();
    let can_undo = !app.state.sim_setup.run_set.history.is_empty();
    let can_redo = !app.state.sim_setup.run_set.future.is_empty();
    let revision = app.state.sim_setup.run_set.revision;
    let mut action: Option<RunSetAction> = None;
    let minimum_height = if app.state.workbench.coarse_pointer {
        44.0
    } else {
        0.0
    };

    let width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(CARD_PAD_X as i8, 7))
        .show(ui, |ui| {
            let inner_width = (width - CARD_PAD_X * 2.0 - 2.0).max(1.0);
            ui.set_width(inner_width);
            ui.spacing_mut().item_spacing.x = 6.0;
            let add = Button::new("Add dimension")
                .icon(Icon::Add)
                .min_height(minimum_height)
                .max_width(inner_width);
            let undo = Button::new("Undo")
                .enabled(can_undo)
                .min_height(minimum_height);
            let redo = Button::new("Redo")
                .enabled(can_redo)
                .min_height(minimum_height);
            let preview = Button::new("Validate and preview")
                .accent()
                .icon(Icon::Grid)
                .min_height(minimum_height)
                .max_width(inner_width);
            let status_text = format!(
                "working revision {revision} · {}",
                validation.status.as_str()
            );
            let status_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            let status_width = ui
                .painter()
                .layout_no_wrap(status_text.clone(), status_font.clone(), t.color.text_faint)
                .size()
                .x;
            let command_width = [&add, &undo, &redo, &preview]
                .into_iter()
                .map(|button| button.measured_width(ui))
                .sum::<f32>();
            // Account for both the leading command row and its trailing
            // status group. Long revisions and changed fonts reflow too.
            let single_row = command_width + status_width + 5.0 * ui.spacing().item_spacing.x + 4.0
                <= inner_width;
            let status = egui::RichText::new(status_text)
                .font(status_font)
                .color(t.color.text_faint);
            ui.horizontal_wrapped(|ui| {
                // The leading glyph was U+FF0B, which the bundled faces do not
                // carry: the page's first control opened with a replacement
                // box. The plus is painted as a vector by the button's own
                // icon slot, where no font has to have it.
                let choices = addable
                    .iter()
                    .map(|kind| page_kit::PopupChoice {
                        label: kind.default_name().to_owned(),
                        unavailable: kind.execution_blocker(),
                    })
                    .collect::<Vec<_>>();
                if let Some(index) = page_kit::command_popup(
                    ui,
                    "run-set.add-dimension",
                    add,
                    "Every axis the executor binds is already declared.",
                    &choices,
                ) && let Some(kind) = addable.get(index)
                {
                    action = Some(RunSetAction::AddDimension(*kind));
                }
                if undo.show(ui).clicked() {
                    action = Some(RunSetAction::Undo);
                }
                if redo.show(ui).clicked() {
                    action = Some(RunSetAction::Redo);
                }
                let preview_clicked = if single_row {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let clicked = preview.show(ui).clicked();
                        ui.add_space(4.0);
                        ui.label(status.clone());
                        clicked
                    })
                    .inner
                } else {
                    preview.show(ui).clicked()
                };
                if preview_clicked {
                    action = Some(RunSetAction::Preview);
                }
            });
            if !single_row {
                ui.add(egui::Label::new(status).wrap());
            }
        });

    if let Some(action) = action {
        commit(app, action);
    }
}
