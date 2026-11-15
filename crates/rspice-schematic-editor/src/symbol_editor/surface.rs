//! Symbol editor status and overlays over explicit display inputs.

use super::{interaction::SymbolRequestSource, session::SymbolEditorSession};
use egui::{Align2, Color32, Rect, Stroke, Ui, vec2};
use rspice_design::symbol::{PinSummary, SymbolDocument};
use rspice_design_model::port::PortSpec;
use rspice_ui_kit::{
    accessibility::counted,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{Button, PROBLEM_MARK_SIDE, Problem, paint_problem_mark},
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

/// What the symbol canvas announces: what it holds and has selected, the
/// armed tool, whether it takes edits, and its shortcuts. `in_place`, while
/// the symbol is edited in place, says where: "in place on X1 in top".
pub fn symbol_canvas_accessibility_label(
    document: &SymbolDocument,
    session: &SymbolEditorSession,
    read_only: bool,
    in_place: Option<&str>,
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
    let place = in_place
        .map(|place| format!(", {place}"))
        .unwrap_or_default();
    format!(
        "Symbol editor canvas{place}. {}; {}, {}; {}. Active tool: {}. {}{shortcuts}",
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
                                "Restore an earlier revision or repair the library.",
                            )
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_dim),
                        );
                    });
                });
        });
    });
}

/// What the canvas says about the pins against their interface, one line
/// each, in the words of `source`, the drawing the interface is read from.
///
/// A symbol whose pins answer its interface, or that defines the interface
/// itself, says nothing: a reader who is not told otherwise is being told
/// that.
pub fn canvas_check_lines(summary: PinSummary, source: &str) -> Vec<String> {
    if summary.interface_empty {
        return vec![format!("The {source} declares no pins")];
    }
    let mut lines = Vec::new();
    if summary.missing > 0 {
        let missing = summary.missing;
        lines.push(if missing == 1 {
            format!("1 {source} pin has no symbol pin")
        } else {
            format!("{missing} {source} pins have no symbol pin")
        });
    }
    if summary.orphaned > 0 {
        lines.push(format!(
            "{} {} not in the {source}",
            counted(summary.orphaned, "pin", "pins"),
            if summary.orphaned == 1 { "is" } else { "are" }
        ));
    }
    if summary.unplaced > 0 {
        lines.push(format!(
            "{} {} not placed",
            counted(summary.unplaced, "pin", "pins"),
            if summary.unplaced == 1 { "is" } else { "are" }
        ));
    }
    lines
}

/// The pin contract's standing, where the drawing is: the one statement of a
/// mismatch in the editor, in view while a pin is being placed. A mismatch is
/// a warning; nothing here stops the symbol being saved.
pub fn canvas_check_note(ctx: &egui::Context, summary: PinSummary, source: &str, stage: Rect) {
    if ctx.content_rect().width() <= 820.0 {
        return;
    }
    let lines = canvas_check_lines(summary, source);
    if lines.is_empty() {
        return;
    }
    let t = Tokens::get(ctx);
    egui::Area::new(egui::Id::new("symbol-editor.canvas-check-note"))
        .order(egui::Order::Middle)
        .pivot(Align2::RIGHT_TOP)
        .fixed_pos(stage.right_top() + vec2(-11.0, 10.0))
        .constrain_to(stage)
        .interactable(false)
        .show(ctx, |ui| {
            for line in lines {
                overlay_frame(ui, &t, t.color.border, |ui| {
                    let (mark, _) = ui.allocate_exact_size(
                        vec2(PROBLEM_MARK_SIDE, PROBLEM_MARK_SIDE),
                        egui::Sense::hover(),
                    );
                    paint_problem_mark(ui.painter(), mark, Problem::Warning);
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(line)
                            .font(theme::sans(tokens::FS_0, FontWeight::Medium))
                            .color(t.color.text),
                    );
                });
            }
        });
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

/// How long the way to a first drawing takes to fade once the symbol has one.
const GENERATE_OVERLAY_FADE_SECONDS: f32 = 0.25;

/// Room between the canvas's top edge and the way to a first drawing.
const GENERATE_OVERLAY_INSET: f32 = 24.0;

/// The way to a first drawing, while the symbol has none: a card lying on
/// the canvas `canvas`, near its top, that offers to generate the symbol
/// from `contract_source`.
///
/// It is an overlay rather than a band laid out above the canvas, so it
/// takes no room from it: when `shown` turns false, because a shape was
/// drawn or a pin placed, the card fades where it stands and neither the
/// canvas nor its view of the symbol moves. While it fades it takes no
/// input, so the press that drew the first shape is never caught by it.
pub fn generate_overlay(
    ctx: &egui::Context,
    canvas: Rect,
    source: &SymbolRequestSource,
    ports: &[PortSpec],
    can_generate: bool,
    contract_source: &str,
    shown: bool,
) -> Option<SymbolSurfaceRequest> {
    let id = egui::Id::new(("symbol-editor.generate-overlay", &source.document));
    let opacity = ctx.animate_bool_with_time(id, shown, GENERATE_OVERLAY_FADE_SECONDS);
    if opacity <= 0.0 {
        return None;
    }
    let t = Tokens::get(ctx);
    let c = t.color;
    let mut requested = None;
    egui::Area::new(id)
        .order(egui::Order::Middle)
        .pivot(Align2::CENTER_TOP)
        .fixed_pos(canvas.center_top() + vec2(0.0, GENERATE_OVERLAY_INSET))
        .constrain_to(canvas)
        .interactable(shown)
        .show(ctx, |ui| {
            ui.set_opacity(opacity);
            egui::Frame::new()
                .fill(c.bg_panel)
                .stroke(Stroke::new(1.0, c.border))
                .corner_radius(t.radius_lg)
                .inner_margin(egui::Margin::symmetric(18, 14))
                .shadow(t.shadow())
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("No symbol drawn yet")
                                .font(theme::sans(tokens::FS_3, FontWeight::Medium))
                                .color(c.text),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "Generate from {contract_source} draws a body with all {} ports placed.",
                                ports.len()
                            ))
                            .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                            .color(c.text_faint),
                        );
                        ui.add_space(8.0);
                        if Button::new(&format!("Generate from {contract_source}"))
                            .enabled(can_generate && shown)
                            .show(ui)
                            .clicked()
                        {
                            requested = Some(SymbolSurfaceRequest {
                                source: source.clone(),
                                action: SymbolSurfaceAction::GenerateFromSchematic,
                            });
                        }
                    });
                });
        });
    if opacity < 1.0 {
        ctx.request_repaint();
    }
    requested
}

#[cfg(test)]
mod tests;
