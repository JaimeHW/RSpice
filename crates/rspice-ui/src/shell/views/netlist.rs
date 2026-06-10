//! Netlist view — the generated SPICE deck with line numbers and syntax
//! coloring, rendered virtually for large decks.

use egui::Ui;

use crate::common::AppState;
use crate::simulation::netlist_viewer::LineType;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::{Button, crumb_text, docbar};

/// Render the netlist view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    docbar(ui, |ui| {
        let reference = &state.workspace.active_view;
        crumb_text(
            ui,
            &[
                (reference.library.as_str(), false),
                (reference.cell.as_str(), true),
                ("netlist", false),
            ],
        );
        let t = Tokens::get(ui.ctx());
        ui.label(
            egui::RichText::new("· generated from schematic")
                .font(theme::sans(crate::ui::tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_faint),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            if Button::new("Regenerate").show(ui).clicked() {
                regenerate(state);
            }
            if Button::new("Copy").show(ui).clicked() {
                ui.ctx()
                    .copy_text(state.simulation.netlist_content.clone());
                state.shell.toasts.info(ui.ctx(), "Netlist copied to clipboard");
            }
        });
    });

    if state.simulation.netlist_content.is_empty() {
        regenerate(state);
    }

    let t = Tokens::get(ui.ctx());
    let c = t.color;

    // Document well.
    let painter_rect = ui.available_rect_before_wrap();
    ui.painter().rect_filled(painter_rect, 0.0, c.canvas_bg);

    let font = theme::mono(12.5, FontWeight::Regular);
    let line_height = 12.5 * 1.7;
    let lines: Vec<&str> = state.simulation.netlist_content.lines().collect();
    let total = lines.len();

    egui::ScrollArea::both()
        .id_salt("volta.netlist.scroll")
        .auto_shrink([false, false])
        .show_rows(ui, line_height, total, |ui, range| {
            for row in range {
                let line = lines[row];
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width().max(640.0), line_height),
                    egui::Sense::hover(),
                );
                if !ui.is_rect_visible(rect) {
                    continue;
                }
                let painter = ui.painter();
                // Line number gutter.
                painter.text(
                    egui::pos2(rect.left() + 18.0 + 30.0, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    (row + 1).to_string(),
                    font.clone(),
                    c.text_faint,
                );
                painter.text(
                    egui::pos2(rect.left() + 18.0 + 30.0 + 16.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    line,
                    font.clone(),
                    line_color(line, &c),
                );
            }
        });
}

/// Map a netlist line to its syntax color.
fn line_color(line: &str, c: &crate::ui::palette::Palette) -> egui::Color32 {
    match LineType::from_line(line) {
        LineType::Comment | LineType::Title | LineType::Blank => c.text_faint,
        LineType::SubcktDef | LineType::SubcktEnd | LineType::ModelDef => c.traces[3],
        LineType::Component => c.traces[1],
        LineType::ParamDef
        | LineType::Include
        | LineType::Options
        | LineType::Analysis
        | LineType::End => c.accent,
        LineType::Continuation | LineType::Unknown => c.text,
    }
}

fn regenerate(state: &mut AppState) {
    let result = crate::simulation::netlist_gen::generate_netlist(&state.schematic);
    state.simulation.netlist_content = result.netlist;
    for warning in &result.warnings {
        state.push_console_message_with_source(
            crate::panels::LogSource::Netlist,
            crate::common::app::ConsoleMessage::warning(warning.clone()),
        );
    }
}
