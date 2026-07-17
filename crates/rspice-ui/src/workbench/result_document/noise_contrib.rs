//! NOISE — the ranked noise-contributor table.
//!
//! Band-integrated output-noise contributions per device and mechanism
//! (results-v2 design, section 05): rank order, mechanism chips in trace
//! hues, share-of-total drawn as an in-row bar so the dominant contributor
//! reads in one fixation. Data comes from the engine's
//! `IntegratedNoise::contribution_summary`, attached to noise analyses.

use egui::Ui;

use crate::common::AppState;
use crate::state::NoiseSummary;
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};

use super::well_hint;

const RANK_W: f32 = 44.0;
const DEVICE_W: f32 = 170.0;
const MECH_W: f32 = 110.0;
const POWER_W: f32 = 130.0;
const SHARE_W: f32 = 170.0;
const ROW_H: f32 = 25.0;
const CELL_INSET: f32 = 10.0;

fn noise_table_min_width() -> f32 {
    RANK_W + DEVICE_W + MECH_W + POWER_W + SHARE_W
}

fn column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width, row.height()),
    )
}

fn paint_clipped_cell(
    ui: &Ui,
    cell: egui::Rect,
    text: impl ToString,
    align: egui::Align2,
    font: egui::FontId,
    color: egui::Color32,
) {
    let x = if align == egui::Align2::RIGHT_CENTER {
        cell.right() - CELL_INSET
    } else {
        cell.left() + CELL_INSET
    };
    ui.painter()
        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
        .text(egui::pos2(x, cell.center().y), align, text, font, color);
}

/// Mechanism chip color: thermal/flicker/shot/burst map onto stable trace
/// hues so spectra and this table agree.
fn mechanism_color(mechanism: &str, t: &Tokens) -> egui::Color32 {
    let traces = &t.color.traces;
    match mechanism {
        "thermal" => traces[1],
        "flicker" => traces[2],
        "shot" => traces[3],
        _ => traces[5],
    }
}

/// The most recent noise summary in the active run, with its analysis label.
fn active_summary(state: &AppState) -> Option<(&NoiseSummary, &str)> {
    let run = state.simulation.active_run()?;
    run.analyses.iter().rev().find_map(|analysis| {
        analysis
            .noise_summary
            .as_ref()
            .map(|summary| (summary, analysis.label.as_str()))
    })
}

/// Render the noise-contributor center view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some((summary, _)) = active_summary(state) else {
        well_hint(ui, "No noise summary — run a noise analysis");
        return;
    };
    let summary = summary.clone();
    let viewport_width = ui.available_width().max(1.0);

    egui::ScrollArea::both()
        .id_salt("rspice.results.noise-contrib")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let width = viewport_width.max(noise_table_min_width());
            ui.set_min_width(width);
            ui.add_space(4.0);

            // Header row.
            let (header, _) = ui.allocate_exact_size(egui::vec2(width, 22.0), egui::Sense::hover());
            ui.painter().hline(
                header.x_range(),
                header.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            let header_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            paint_clipped_cell(
                ui,
                column_rect(header, 0.0, RANK_W),
                "#",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_clipped_cell(
                ui,
                column_rect(header, RANK_W, DEVICE_W),
                "DEVICE",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_clipped_cell(
                ui,
                column_rect(header, RANK_W + DEVICE_W, MECH_W),
                "MECHANISM",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_clipped_cell(
                ui,
                column_rect(header, RANK_W + DEVICE_W + MECH_W, POWER_W),
                "POWER (V²)",
                egui::Align2::RIGHT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_clipped_cell(
                ui,
                column_rect(
                    header,
                    RANK_W + DEVICE_W + MECH_W + POWER_W,
                    width - RANK_W - DEVICE_W - MECH_W - POWER_W,
                ),
                "SHARE",
                egui::Align2::RIGHT_CENTER,
                header_font,
                c.text_faint,
            );

            // Contributor rows, ranked (rows arrive sorted from the engine).
            for (rank, row) in summary.rows.iter().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_H), egui::Sense::hover());
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        format!(
                            "Rank {}, device {}, mechanism {}, noise power {:.3e} volts squared, share {:.1} percent",
                            rank + 1,
                            row.device,
                            row.mechanism,
                            row.power,
                            row.share_pct,
                        ),
                    )
                });
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_role(egui::accesskit::Role::Row);
                });
                if !ui.is_rect_visible(rect) {
                    continue;
                }
                if response.hovered() {
                    ui.painter().rect_filled(rect, 0.0, c.bg_hover);
                }
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                );

                paint_clipped_cell(
                    ui,
                    column_rect(rect, 0.0, RANK_W),
                    (rank + 1).to_string(),
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );
                paint_clipped_cell(
                    ui,
                    column_rect(rect, RANK_W, DEVICE_W),
                    row.device.as_str(),
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );

                // Mechanism chip in the matching trace hue.
                let fg = mechanism_color(row.mechanism, &t);
                let galley = ui.painter().layout_no_wrap(
                    row.mechanism.to_owned(),
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    fg,
                );
                let mechanism_cell = column_rect(rect, RANK_W + DEVICE_W, MECH_W);
                let chip = egui::Rect::from_min_size(
                    egui::pos2(
                        mechanism_cell.left() + CELL_INSET,
                        rect.center().y - galley.size().y / 2.0 - 2.0,
                    ),
                    galley.size() + egui::vec2(12.0, 4.0),
                );
                let mechanism_painter = ui
                    .painter()
                    .with_clip_rect(mechanism_cell.shrink2(egui::vec2(2.0, 0.0)));
                mechanism_painter
                    .rect_filled(chip, chip.height() / 2.0, fg.gamma_multiply(0.16));
                mechanism_painter.galley(chip.min + egui::vec2(6.0, 2.0), galley, fg);

                paint_clipped_cell(
                    ui,
                    column_rect(rect, RANK_W + DEVICE_W + MECH_W, POWER_W),
                    format!("{:.3e}", row.power),
                    egui::Align2::RIGHT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text_dim,
                );

                // Share: tinted bar under the percentage, width ∝ share.
                let share_cell = column_rect(
                    rect,
                    RANK_W + DEVICE_W + MECH_W + POWER_W,
                    width - RANK_W - DEVICE_W - MECH_W - POWER_W,
                );
                let share_rect = share_cell.shrink2(egui::vec2(CELL_INSET, 4.0));
                let fill = share_rect.width() * (row.share_pct as f32 / 100.0).clamp(0.0, 1.0);
                let share_painter = ui.painter().with_clip_rect(share_cell);
                share_painter.rect_filled(
                    egui::Rect::from_min_size(
                        share_rect.min,
                        egui::vec2(fill, share_rect.height()),
                    ),
                    2.0,
                    c.accent_dim,
                );
                share_painter.text(
                    egui::pos2(share_rect.right() - 4.0, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    format!("{:.1} %", row.share_pct),
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_table_preserves_every_desktop_column_at_phone_width() {
        assert_eq!(noise_table_min_width(), 624.0);
        assert!(noise_table_min_width() > 390.0);
        assert!(SHARE_W >= 150.0);
    }
}

/// Right panel: band, total, dominant contributor.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some((summary, label)) = active_summary(state) else {
        return;
    };

    section_header(ui, "Noise summary", None);
    let band = format!(
        "{} – {}",
        fmt_si(summary.band.0, "Hz", 3),
        fmt_si(summary.band.1, "Hz", 3)
    );
    let total = fmt_si(summary.total_rms, "V rms", 3);
    let contributors = summary.rows.len().to_string();
    measurement_table(
        ui,
        &[
            ("Analysis", label),
            ("Band", band.as_str()),
            ("Total output", total.as_str()),
            ("Contributors", contributors.as_str()),
        ],
    );

    if let Some(top) = summary.rows.first() {
        ui.add_space(8.0);
        section_header(ui, "Dominant", None);
        let share = format!("{:.1} % ({})", top.share_pct, top.mechanism);
        measurement_table(ui, &[(top.device.as_str(), share.as_str())]);
    }
}
