//! Device-local visibility for optional drawing-sheet construction layers.
//!
//! The authored paper edge is deliberately not configurable here. Layer
//! choices affect only the editing canvas and never mutate Page Setup,
//! printing, export, netlisting, or simulation.

use egui::{Context, Frame, Margin, RichText, Stroke, Ui};

use crate::state::DrawingSheetLayerVisibility;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const EYEBROW: &str = "VIEW \u{00b7} PERMANENT PAPER \u{00b7} OPTIONAL CONSTRUCTION DETAIL";
const TITLE: &str = "Drawing-sheet layers";
const DESCRIPTION: &str = "Choose which supporting drawing-sheet details appear while editing. The physical paper boundary is permanent and cannot be hidden.";

pub(crate) fn open_drawing_sheet_layers_dialog(state: &mut AppState) -> bool {
    if state.dialogs.drawing_sheet_layers.open {
        return false;
    }
    state
        .dialogs
        .drawing_sheet_layers
        .open(state.ui.drawing_sheet_layers);
    true
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_drawing_sheet_layers_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.drawing_sheet_layers.open {
            return;
        }

        let choice = Dialog::new(EYEBROW, TITLE, "Close")
            .description(DESCRIPTION)
            .size(DialogSize::Transaction)
            .initial_focus(DialogInitialFocus::Primary)
            .show(ctx, |ui| {
                drawing_sheet_layers_body(ui, &mut self.state.dialogs.drawing_sheet_layers.draft);
            });
        // The approved surface is a live device-local view controller. It
        // never creates project history and never changes printed output.
        self.state.ui.drawing_sheet_layers = self.state.dialogs.drawing_sheet_layers.draft;

        match choice {
            DialogChoice::Primary | DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.drawing_sheet_layers.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

fn drawing_sheet_layers_body(ui: &mut Ui, layers: &mut DrawingSheetLayerVisibility) {
    let t = Tokens::get(ui.ctx());
    ui.group(|ui| {
        ui.label(RichText::new("The paper boundary always remains visible.").strong());
        ui.weak("These are workspace-view layers only. They do not change the authored sheet and do not control what Print or Export includes.");
    });
    ui.add_space(10.0);
    Frame::NONE
        .fill(t.color.border)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(1))
        .corner_radius(3.0)
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            layer_row(
                ui,
                "Printable margin boundary",
                "Dashed inset showing the authored printable area.",
                &mut layers.margins,
            );
            layer_row(
                ui,
                "Zone references",
                "Zone ticks and edge labels. The authored drawing border remains visible independently.",
                &mut layers.zones,
            );
            layer_row(
                ui,
                "Title block",
                "Authored identity block and its visible fields.",
                &mut layers.title_block,
            );
            layer_row(
                ui,
                "Registration and folding marks",
                "Corner registration targets and format-defined fold marks.",
                &mut layers.registration_marks,
            );
            layer_row(
                ui,
                "Physical dimension callouts at detail zoom",
                "Width and height readouts shown only when they remain legible. The page origin remains visible independently.",
                &mut layers.dimensions_at_detail_zoom,
            );
        });

    ui.add_space(12.0);
    ui.columns(3, |columns| {
        note(
            &mut columns[0],
            "Canvas",
            "The unbounded workspace remains available outside the paper for parking and reorganizing content.",
        );
        note(
            &mut columns[1],
            "Output",
            "Print and Export have independent inclusion controls for border, title block, zones, grid, crop marks, and outside-sheet content.",
        );
        columns[2].group(|ui| {
            ui.label(
                RichText::new("Reset")
                    .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(6.0);
            if Button::new("Restore recommended editing layers")
                .show(ui)
                .clicked()
            {
                *layers = DrawingSheetLayerVisibility::default();
            }
        });
    });
}

fn layer_row(ui: &mut Ui, label: &str, detail: &str, checked: &mut bool) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(11, 10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.checkbox(checked, "");
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(label)
                            .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.weak(detail);
                });
            });
        });
}

fn note(ui: &mut Ui, heading: &str, body: &str) {
    ui.group(|ui| {
        ui.label(RichText::new(heading).strong());
        ui.weak(body);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::AppState;

    #[test]
    fn opening_captures_the_current_device_local_view() {
        let mut state = AppState::default();
        state.ui.drawing_sheet_layers.zones = false;

        assert!(open_drawing_sheet_layers_dialog(&mut state));
        assert!(!state.dialogs.drawing_sheet_layers.draft.zones);
    }
}
