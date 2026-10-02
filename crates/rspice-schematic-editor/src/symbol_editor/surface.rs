//! Symbol editor status, overlays and publication form over explicit display inputs.

use super::{interaction::SymbolRequestSource, session::SymbolEditorSession};
use egui::{Align2, Color32, Rect, Stroke, Ui, vec2};
use rspice_design::symbol::{PinSummary, SymbolDocument, publication::SymbolSaveCheck};
use rspice_design_model::{cell_view::CellViewRef, port::PortSpec};
use rspice_ui_kit::{
    accessibility::counted,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{
        Button, Dialog, DialogChoice, DialogInitialFocus, DialogSize, DialogTransactionTone,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolSurfaceAction {
    CopyToEditableLibrary,
    GenerateFromSchematic,
}

#[derive(Debug, Clone)]
pub struct SymbolSurfaceRequest {
    pub source: SymbolRequestSource,
    pub action: SymbolSurfaceAction,
}

pub fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    session: &SymbolEditorSession,
    read_only: bool,
    shortcuts: &str,
) -> String {
    let placed_pins = document
        .pins
        .iter()
        .filter(|pin| pin.position.is_some())
        .count();
    let selection = session.effective_selection();
    let selected = selection.pins.len() + selection.shapes.len() + selection.attributes.len();
    let edit_state = if read_only { "Read only." } else { "Editable." };
    format!(
        "Symbol editor canvas. {}; {}, {}; {}. Active tool: {}. {}{shortcuts}",
        counted(document.body.len(), "shape", "shapes"),
        counted(document.pins.len(), "pin", "pins"),
        counted(placed_pins, "pin placed", "pins placed"),
        counted(selected, "item selected", "items selected"),
        session.tool.label(),
        edit_state,
    )
}

pub fn invalid_symbol_document_state(ui: &mut Ui, error: &str) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let stage = ui.available_rect_before_wrap();
    ui.scope_builder(egui::UiBuilder::new().max_rect(stage), |ui| {
        ui.painter().rect_filled(stage, 0.0, c.canvas_bg);
        ui.centered_and_justified(|ui| {
            egui::Frame::new()
                .fill(c.bg_panel)
                .stroke(Stroke::new(1.0, c.err))
                .corner_radius(t.radius)
                .inner_margin(egui::Margin::symmetric(18, 14))
                .show(ui, |ui| {
                    ui.set_max_width(620.0);
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new("Symbol document could not be opened")
                                .font(theme::sans(tokens::FS_3, FontWeight::Medium))
                                .color(c.err),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new(error)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .color(c.text),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(
                                "The stored symbol metadata was left unchanged. Restore a valid project revision or repair the imported library before editing this view.",
                            )
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_dim),
                        );
                    });
                });
        });
    });
}

pub fn canvas_breadcrumb(ctx: &egui::Context, reference: &CellViewRef, stage: Rect) {
    let t = Tokens::get(ctx);
    let text = format!(
        "{} / {} / {}",
        reference.library, reference.cell, reference.view
    );
    egui::Area::new(egui::Id::new("symbol-editor.canvas-breadcrumb"))
        .order(egui::Order::Middle)
        .fixed_pos(stage.min + vec2(10.0, 9.0))
        .constrain_to(stage)
        .interactable(false)
        .show(ctx, |ui| {
            overlay_frame(ui, &t, t.color.border, |ui| {
                ui.label(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text),
                );
            });
        });
}

pub fn canvas_check_note(
    ctx: &egui::Context,
    document: &SymbolDocument,
    ports: &[PortSpec],
    stage: Rect,
) {
    if ctx.content_rect().width() <= 820.0 {
        return;
    }
    let t = Tokens::get(ctx);
    let (message, color) = match document.pin_summary(ports) {
        PinSummary::Match => ("Pin contract matches the interface".to_owned(), t.color.ok),
        PinSummary::Unplaced(count) => (
            format!(
                "{count} declared pin{} unplaced",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
        ),
        PinSummary::Orphaned(count) => (
            format!(
                "{count} pin{} not declared by the interface",
                if count == 1 { "" } else { "s" }
            ),
            t.color.err,
        ),
        PinSummary::NoSchematic => (
            "No schematic interface declares this cell".to_owned(),
            t.color.warn,
        ),
    };
    egui::Area::new(egui::Id::new("symbol-editor.canvas-check-note"))
        .order(egui::Order::Middle)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(stage.right_top() + vec2(-11.0, 10.0))
        .constrain_to(stage)
        .interactable(false)
        .show(ctx, |ui| {
            overlay_frame(ui, &t, color.gamma_multiply(0.55), |ui| {
                ui.label(
                    egui::RichText::new(message)
                        .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                        .color(color),
                );
            });
        });
}

fn symbol_save_dialog_body(
    ui: &mut Ui,
    checks: &[SymbolSaveCheck],
    current_revision: u64,
    revision_note: &mut String,
) -> Option<egui::Id> {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.accent.gamma_multiply(0.08))
        .stroke(Stroke::new(1.0, t.color.accent.gamma_multiply(0.42)))
        .inner_margin(egui::Margin::symmetric(12, 9))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(
                    "Pin count, netlist order, electrical type and model binding are checked atomically.",
                )
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(t.color.text),
            );
        });
    ui.add_space(8.0);
    egui::Grid::new("symbol-editor.save-checks")
        .num_columns(4)
        .striped(true)
        .min_col_width(112.0)
        .show(ui, |ui| {
            for heading in ["CHECK", "EXPECTED", "OBSERVED", "STATUS"] {
                ui.label(
                    egui::RichText::new(heading)
                        .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                        .color(t.color.text_faint),
                );
            }
            ui.end_row();
            for check in checks {
                ui.label(check.label);
                ui.label(
                    egui::RichText::new(&check.expected)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
                ui.label(
                    egui::RichText::new(&check.observed)
                        .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                );
                ui.label(
                    egui::RichText::new(if check.passed { "pass" } else { "blocked" }).color(
                        if check.passed {
                            t.color.ok
                        } else {
                            t.color.err
                        },
                    ),
                );
                ui.end_row();
            }
        });
    ui.add_space(10.0);
    ui.label(
        egui::RichText::new(format!(
            "Revision note \u{00b7} next revision {}",
            current_revision.saturating_add(1)
        ))
        .font(theme::sans(tokens::FS_1, FontWeight::Medium)),
    );
    let response = ui.add(
        egui::TextEdit::singleline(revision_note)
            .hint_text("Describe the reviewed symbol change")
            .desired_width(f32::INFINITY),
    );
    if revision_note.trim().is_empty() {
        ui.label(
            egui::RichText::new("A revision note is required.")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.err),
        );
    }
    Some(response.id)
}

fn overlay_frame(ui: &mut Ui, t: &Tokens, stroke: Color32, body: impl FnOnce(&mut Ui)) {
    egui::Frame::new()
        .fill(Color32::from_rgba_unmultiplied(
            t.color.bg_panel.r(),
            t.color.bg_panel.g(),
            t.color.bg_panel.b(),
            242,
        ))
        .stroke(Stroke::new(1.0, stroke))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(9, 0))
        .shadow(t.shadow())
        .show(ui, |ui| {
            ui.set_min_height(27.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), body);
        });
}

pub fn read_only_banner(
    ui: &mut Ui,
    source: &SymbolRequestSource,
    message: &str,
    copy_available: bool,
) -> Option<SymbolSurfaceRequest> {
    let mut requested = None;
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 24.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter()
                .rect_filled(rect, 0.0, c.warn.gamma_multiply(0.13));
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, c.border),
            );
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(message)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.warn),
            );
            if !copy_available {
                return;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                if Button::new("Copy to editable library...")
                    .ghost()
                    .show(ui)
                    .clicked()
                {
                    requested = Some(SymbolSurfaceRequest {
                        source: source.clone(),
                        action: SymbolSurfaceAction::CopyToEditableLibrary,
                    });
                }
            });
        },
    );
    requested
}

pub fn revision_state_strip(ui: &mut Ui, dirty: bool, revision: u64) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (text, color) = if dirty {
        (
            format!("Unpublished edits \u{00b7} last published revision {revision}"),
            c.warn,
        )
    } else if revision == 0 {
        ("No revision published yet".to_owned(), c.text_dim)
    } else {
        (format!("Published revision {revision}"), c.ok)
    };
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 22.0),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| {
            let rect = ui.max_rect();
            ui.painter().hline(
                rect.x_range(),
                rect.bottom() - 0.5,
                Stroke::new(1.0, c.border),
            );
            ui.add_space(12.0);
            ui.label(
                egui::RichText::new(text)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(color),
            );
        },
    );
}

pub fn empty_generate_state(
    ui: &mut Ui,
    source: &SymbolRequestSource,
    ports: &[PortSpec],
    can_generate: bool,
) -> Option<SymbolSurfaceRequest> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut requested = None;
    ui.allocate_ui_with_layout(
        vec2(ui.available_width(), 96.0),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            ui.add_space(14.0);
            ui.label(
                egui::RichText::new("No symbol drawn yet")
                    .font(theme::sans(tokens::FS_3, FontWeight::Medium))
                    .color(c.text),
            );
            ui.label(
                egui::RichText::new(format!(
                    "Generate from schematic builds a box body with all {} ports placed.",
                    ports.len()
                ))
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text_faint),
            );
            ui.add_space(8.0);
            if Button::new("Generate from schematic")
                .enabled(can_generate)
                .show(ui)
                .clicked()
            {
                requested = Some(SymbolSurfaceRequest {
                    source: source.clone(),
                    action: SymbolSurfaceAction::GenerateFromSchematic,
                });
            }
        },
    );
    requested
}

pub fn show_save_symbol_dialog(
    ctx: &egui::Context,
    checks: &[SymbolSaveCheck],
    current_revision: u64,
    revision_note: &mut String,
    error: Option<&str>,
    can_publish: bool,
) -> DialogChoice {
    let blocking = checks.iter().filter(|check| !check.passed).count();
    let note_valid = !revision_note.trim().is_empty();
    let blocking_title = format!("{blocking} blocking contract issue(s)");
    let mut dialog = Dialog::new(
        "SYMBOL EDITOR \u{00b7} MODEL-BOUND CONTRACT",
        "Validate and save symbol",
        "Save symbol revision",
    )
    .description("Pin order, electrical types and model binding are validated before this symbol revision replaces the current project view.")
    .size(DialogSize::SimulationWorkflow)
    .initial_height(520.0)
    .initial_focus(DialogInitialFocus::BodyControl)
    .ghost("Cancel")
    .primary_enabled(blocking == 0 && note_valid && can_publish);
    if let Some(error) = error {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            "Symbol revision was not saved",
            error,
        );
    } else if blocking > 0 {
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            &blocking_title,
            "Resolve every failed contract row before publishing a symbol revision.",
        );
    }
    dialog.show_with_initial_body_focus(ctx, |ui| {
        symbol_save_dialog_body(ui, checks, current_revision, revision_note)
    })
}

#[cfg(test)]
mod tests;
