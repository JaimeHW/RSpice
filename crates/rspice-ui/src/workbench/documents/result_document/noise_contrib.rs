//! NOISE — amplitude-density instrument derived from retained power spectral
//! density and its contributor
//! evidence.
//!
//! The center surface owns the frequency spectrum. The right inspector keeps
//! the complete band-integrated contributor table associated with the exact
//! same noise analysis, so changing the active dataset or analysis cannot
//! leave either surface reading stale evidence.

use egui::{Align2, Sense, Ui};

use crate::state::{AnalysisResult, NoiseSummary};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

const RANK_W: f32 = 30.0;
const SOURCE_W: f32 = 132.0;
const POWER_W: f32 = 94.0;
const SHARE_W: f32 = 70.0;
const ROW_H: f32 = 31.0;
const CELL_INSET: f32 = 6.0;

fn contributor_table_width() -> f32 {
    RANK_W + SOURCE_W + POWER_W + SHARE_W
}

fn selected_noise_analysis(state: &AppState) -> Option<&AnalysisResult> {
    let run = state.simulation.active_run()?;
    let index = super::bode::selected_noise_analysis_index(state)?;
    run.analyses.get(index)
}

fn selected_summary(state: &AppState) -> Option<(&NoiseSummary, &str)> {
    let analysis = selected_noise_analysis(state)?;
    Some((analysis.noise_summary.as_ref()?, analysis.label.as_str()))
}

/// Render the independent Noise spectrum instrument.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    super::bode::show_noise_spectrum(ui, state);
}

/// Render spectrum provenance and the full contributor table for the exact
/// analysis shown in the center instrument.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    super::bode::noise_spectrum_right_panel(ui, state);

    let Some((summary, label)) = selected_summary(state) else {
        ui.add_space(8.0);
        section_header(ui, "Contributors", Some("not retained"));
        super::panel_note(
            ui,
            "This noise result contains a spectrum, but no band-integrated contributor evidence was retained.",
        );
        return;
    };
    let summary = summary.clone();

    ui.add_space(8.0);
    section_header(ui, "Contributor evidence", None);
    let band = format!(
        "{} – {}",
        fmt_si(summary.band.0, "Hz", 3),
        fmt_si(summary.band.1, "Hz", 3)
    );
    let total = summary
        .total_rms
        .map(|value| fmt_si(value, "V rms", 3))
        .unwrap_or_else(|| "Not retained".to_owned());
    let input = summary
        .input_rms
        .map(|value| fmt_si(value, "V rms", 3))
        .unwrap_or_else(|| "Not retained".to_owned());
    let count = summary.rows.len().to_string();
    measurement_table(
        ui,
        &[
            ("Analysis", label),
            ("Band", band.as_str()),
            ("Output integrated", total.as_str()),
            ("Input integrated", input.as_str()),
            ("Contributors", count.as_str()),
        ],
    );

    ui.add_space(8.0);
    section_header(ui, "Ranked contributors", Some("integrated V²"));
    if summary.rows.is_empty() {
        super::panel_note(ui, "No per-device contributor rows were retained.");
        return;
    }
    contributor_table(ui, &summary);
}

fn contributor_table(ui: &mut Ui, summary: &NoiseSummary) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let viewport_width = ui.available_width().max(1.0);

    let table = ui
        .scope(|ui| {
            egui::ScrollArea::horizontal()
                .id_salt("rspice.results.noise.contributors")
                .auto_shrink([false, true])
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    let width = viewport_width.max(contributor_table_width());
                    ui.set_min_width(width);
                    contributor_header(ui, width);
                    for (rank, row) in summary.rows.iter().enumerate() {
                        let (rect, response) =
                            ui.allocate_exact_size(egui::vec2(width, ROW_H), Sense::hover());
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Label,
                                ui.is_enabled(),
                                format!(
                                    "Contributor rank {}, {} {}, integrated noise power {:.6e} volts squared, share {:.3} percent",
                                    rank + 1,
                                    row.device,
                                    row.mechanism,
                                    row.power,
                                    row.share_pct
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
                        paint_cell(
                            ui,
                            rect,
                            0.0,
                            RANK_W,
                            rank + 1,
                            Align2::LEFT_CENTER,
                            c.text_faint,
                        );
                        let source = format!("{}\n{}", row.device, row.mechanism);
                        paint_cell(ui, rect, RANK_W, SOURCE_W, source, Align2::LEFT_CENTER, c.text);
                        paint_cell(
                            ui,
                            rect,
                            RANK_W + SOURCE_W,
                            POWER_W,
                            format!("{:.3e}", row.power),
                            Align2::RIGHT_CENTER,
                            c.text_dim,
                        );
                        let share_offset = RANK_W + SOURCE_W + POWER_W;
                        let share_cell = column_rect(rect, share_offset, width - share_offset);
                        let fill_rect = share_cell.shrink2(egui::vec2(CELL_INSET, 8.0));
                        let fill = fill_rect.width()
                            * (row.share_pct as f32 / 100.0).clamp(0.0, 1.0);
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                fill_rect.min,
                                egui::vec2(fill, fill_rect.height()),
                            ),
                            2.0,
                            c.accent_dim,
                        );
                        paint_cell(
                            ui,
                            rect,
                            share_offset,
                            width - share_offset,
                            format!("{:.1}%", row.share_pct),
                            Align2::RIGHT_CENTER,
                            c.text,
                        );
                    }
                });
        })
        .response;
    table.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("Ranked noise contributors, {} rows", summary.rows.len()),
        )
    });
    ui.ctx().accesskit_node_builder(table.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_row_count(summary.rows.len().saturating_add(1));
        node.set_column_count(4);
    });
}

fn contributor_header(ui: &mut Ui, width: f32) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, row) = ui.allocate_exact_size(egui::vec2(width, 23.0), Sense::hover());
    ui.ctx().accesskit_node_builder(row.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    for (index, (label, offset, column_width, align)) in [
        ("#", 0.0, RANK_W, Align2::LEFT_CENTER),
        ("SOURCE", RANK_W, SOURCE_W, Align2::LEFT_CENTER),
        ("POWER", RANK_W + SOURCE_W, POWER_W, Align2::RIGHT_CENTER),
        (
            "SHARE",
            RANK_W + SOURCE_W + POWER_W,
            width - RANK_W - SOURCE_W - POWER_W,
            Align2::RIGHT_CENTER,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let cell = column_rect(rect, offset, column_width);
        let response = ui.interact(cell, row.id.with(index), Sense::hover());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
            node.set_label(label);
        });
        paint_cell(ui, rect, offset, column_width, label, align, c.text_faint);
    }
}

fn column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width.max(0.0), row.height()),
    )
}

fn paint_cell(
    ui: &Ui,
    row: egui::Rect,
    offset: f32,
    width: f32,
    text: impl ToString,
    align: Align2,
    color: egui::Color32,
) {
    let cell = column_rect(row, offset, width);
    let x = if align == Align2::RIGHT_CENTER {
        cell.right() - CELL_INSET
    } else {
        cell.left() + CELL_INSET
    };
    ui.painter()
        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
        .text(
            egui::pos2(x, cell.center().y),
            align,
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    #[test]
    fn contributor_table_preserves_all_columns_at_narrow_inspector_width() {
        assert_eq!(contributor_table_width(), 326.0);
        assert!(SOURCE_W >= 120.0);
        assert!(POWER_W >= 90.0);
    }

    #[test]
    fn selected_summary_is_bound_to_the_same_renderable_noise_analysis() {
        let mut first = AnalysisResult::new(1, AnalysisType::Noise, "first").with_waveforms(vec![
            WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
        ]);
        first.noise_summary = Some(NoiseSummary {
            band: (1.0, 10.0),
            ..NoiseSummary::default()
        });
        let mut second =
            AnalysisResult::new(2, AnalysisType::Noise, "second").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![3.0e-9, 4.0e-9], "#fff"),
            ]);
        second.noise_summary = Some(NoiseSummary {
            band: (2.0, 20.0),
            ..NoiseSummary::default()
        });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(first);
        run.add_analysis(second);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(1));

        let (summary, label) = selected_summary(&state).expect("selected noise summary");
        assert_eq!(label, "second");
        assert_eq!(summary.band, (2.0, 20.0));
    }
}
