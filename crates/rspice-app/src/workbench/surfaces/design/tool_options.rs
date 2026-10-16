//! The armed tool's options strip.
//!
//! A canvas command that has a choice left to make shows it here, over the
//! drawing, while the command runs — the way Virtuoso puts command options
//! beside the cursor rather than in front of it. The strip appears only while
//! a tool that has an option is armed, states the command and that option,
//! and nothing else: the snap is in the status bar, the selection is
//! highlighted on the canvas, and Escape cancels every command in the editor.

use egui::Ui;

use crate::state::{MoveSelectionMode, StretchOrthogonalPolicy, Tool};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, segmented};
use crate::workbench::RSpiceApp;

/// Air between the command's name and its options.
const NAME_GAP: f32 = 12.0;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    match app.state.schematic.session.editor.tool {
        Tool::MoveSelection if app.state.dialogs.move_selection.armed => {
            strip(ui, "Move", |ui| {
                let mode = &mut app.state.dialogs.move_selection.mode;
                let mut selected = MoveSelectionMode::ALL
                    .iter()
                    .position(|candidate| candidate == mode)
                    .unwrap_or(0);
                let labels = MoveSelectionMode::ALL.map(MoveSelectionMode::label);
                if segmented(
                    ui,
                    "move-selection-mode",
                    &labels,
                    &mut selected,
                    SegmentedWidth::Natural,
                ) {
                    *mode = MoveSelectionMode::ALL[selected];
                }
            });
        }
        Tool::StretchSelection if app.state.dialogs.stretch_selection.armed => {
            strip(ui, "Stretch", |ui| {
                let policy = &mut app.state.dialogs.stretch_selection.policy;
                let mut orthogonal = *policy == StretchOrthogonalPolicy::PreserveOrthogonal;
                if ui
                    .checkbox(&mut orthogonal, "Keep wires orthogonal")
                    .changed()
                {
                    *policy = if orthogonal {
                        StretchOrthogonalPolicy::PreserveOrthogonal
                    } else {
                        StretchOrthogonalPolicy::AllowDiagonal
                    };
                }
            });
        }
        _ => {}
    }
}

fn strip(ui: &mut Ui, command: &str, options: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.bg_inset)
        .inner_margin(egui::Margin::symmetric(12, 5))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = NAME_GAP;
                ui.label(
                    egui::RichText::new(command)
                        .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text),
                );
                options(ui);
            });
        });
}
