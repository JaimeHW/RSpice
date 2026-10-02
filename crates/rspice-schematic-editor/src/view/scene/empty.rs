//! Empty-sheet guidance at desktop and narrow canvas widths.

use egui::{Painter, Rect};

const EMPTY_HINT_MOBILE_BREAKPOINT: f32 = 460.0;
const EMPTY_HINT_DESKTOP_LINES: [&str; 3] = [
    "Empty schematic",
    "Use Place instance to choose devices and sources",
    "File > Open project loads an existing design",
];
const EMPTY_HINT_MOBILE_LINES: [&str; 4] = [
    "Empty schematic",
    "Use Place instance to choose a component",
    "The toolbar provides wiring, labels, and probes",
    "File > Open project loads an existing design",
];

pub fn draw_empty_hint(painter: &Painter, available: Rect) {
    use rspice_ui_kit::theme::{self, FontWeight};

    let palette = rspice_ui_kit::tokens::active_palette();
    let center = available.center();
    if available.width() < EMPTY_HINT_MOBILE_BREAKPOINT {
        draw_empty_hint_mobile(painter, center);
        return;
    }
    painter.text(
        center - egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[0],
        theme::sans(15.0, FontWeight::Medium),
        palette.text_dim,
    );
    painter.text(
        center + egui::vec2(0.0, 2.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[1],
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
    painter.text(
        center + egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        EMPTY_HINT_DESKTOP_LINES[2],
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
}

fn draw_empty_hint_mobile(painter: &Painter, center: egui::Pos2) {
    use rspice_ui_kit::theme::{self, FontWeight};

    let palette = rspice_ui_kit::tokens::active_palette();
    let lines = &EMPTY_HINT_MOBILE_LINES;
    let line_height = 20.0;
    let first_y = center.y - (lines.len().saturating_sub(1) as f32 * line_height) * 0.5;
    for (index, line) in lines.iter().enumerate() {
        let title = index == 0;
        painter.text(
            egui::pos2(center.x, first_y + index as f32 * line_height),
            egui::Align2::CENTER_CENTER,
            *line,
            theme::sans(
                if title { 15.0 } else { 12.0 },
                if title {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            ),
            if title {
                palette.text_dim
            } else {
                palette.text_faint
            },
        );
    }
}
