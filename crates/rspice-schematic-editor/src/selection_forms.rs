//! Move and Stretch form presentation over local options and resolved context.

use egui::Ui;
use rspice_design::schematic::{movement::MoveSelectionMode, stretch::StretchOrthogonalPolicy};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{
    SchematicCommandPreview, field_label, read_only_value, schematic_command_workflow,
    select_with_response,
};

const FOOTER_NOTE: &str = "Pointer, touch, stylus, and keyboard entry resolve to the same exact coordinates. Escape cancels without modifying the document.";

pub fn show_move(
    ui: &mut Ui,
    summary: &str,
    snap: &str,
    validation_message: Option<&str>,
    mode: &mut MoveSelectionMode,
) -> (Option<egui::Id>, bool) {
    let preview = SchematicCommandPreview {
        subject: summary,
        location: "anchor and destination pending",
        electrical_outcome: "connectivity-preserving transform",
        grid: snap,
    };
    let focus = schematic_command_workflow(
        ui,
        "MOVE",
        preview,
        if validation_message.is_some() {
            "blocked"
        } else {
            "legal preview"
        },
        validation_message.is_none(),
        |ui| {
            let labels = MoveSelectionMode::ALL.map(|mode| mode.label().to_owned());
            let output = field_label(ui, "Mode", |ui| {
                select_with_response(
                    ui,
                    "move-selection-mode",
                    "Move mode",
                    mode.label(),
                    &labels,
                    ui.available_width(),
                )
            });
            ui.add_space(9.0);
            read_only_value(ui, "Snap", snap);
            ui.add_space(9.0);
            read_only_value(ui, "Selection", summary);
            ui.add_space(12.0);
            footer(ui, validation_message);
            output
        },
    );
    let mut edited = false;
    if let Some(index) = focus.picked {
        let next = MoveSelectionMode::ALL[index];
        if next != *mode {
            *mode = next;
            edited = true;
        }
    }
    (Some(focus.response.id), edited)
}

pub fn show_stretch(
    ui: &mut Ui,
    selection: &str,
    snap: &str,
    validation_message: Option<&str>,
    policy: &mut StretchOrthogonalPolicy,
) -> (Option<egui::Id>, bool) {
    let preview = SchematicCommandPreview {
        subject: selection,
        location: "anchor and destination pending",
        electrical_outcome: match *policy {
            StretchOrthogonalPolicy::PreserveOrthogonal => "orthogonal segment update",
            StretchOrthogonalPolicy::AllowDiagonal => "diagonal segment update",
        },
        grid: snap,
    };
    let focus = schematic_command_workflow(
        ui,
        "STRETCH",
        preview,
        if validation_message.is_some() {
            "blocked"
        } else {
            "legal preview"
        },
        validation_message.is_none(),
        |ui| {
            read_only_value(ui, "Selection", selection);
            ui.add_space(9.0);
            let labels = StretchOrthogonalPolicy::ALL.map(|policy| policy.label().to_owned());
            let output = field_label(ui, "Orthogonal policy", |ui| {
                select_with_response(
                    ui,
                    "stretch-selection-orthogonal-policy",
                    "Orthogonal policy",
                    policy.label(),
                    &labels,
                    ui.available_width(),
                )
            });
            ui.add_space(9.0);
            read_only_value(ui, "Snap", snap);
            ui.add_space(12.0);
            footer(ui, validation_message);
            output
        },
    );
    let mut edited = false;
    if let Some(index) = focus.picked {
        let next = StretchOrthogonalPolicy::ALL[index];
        if next != *policy {
            *policy = next;
            edited = true;
        }
    }
    (Some(focus.response.id), edited)
}

fn footer(ui: &mut Ui, validation_message: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(validation_message.unwrap_or(FOOTER_NOTE))
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(if validation_message.is_some() {
                    t.color.err
                } else {
                    t.color.text_faint
                }),
        )
        .wrap(),
    );
}
