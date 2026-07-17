//! SENS - retained, ranked parameter-sensitivity contributions.
//!
//! This viewer is intentionally dataset-bound. It reads the exact
//! [`AnalysisResultPayload::Sensitivity`] attached to the selected analysis
//! and never falls back to console text or the legacy mutable analysis cache.

use std::cmp::Ordering;

use egui::{Sense, Ui};

use crate::common::AppState;
use crate::state::{AnalysisResultPayload, SensitivityResultMode, SensitivityResultRow};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};

use super::strip::StripHeader;
use super::{panel_note, well_hint};

const RANK_WIDTH: f32 = 44.0;
const PARAMETER_WIDTH: f32 = 230.0;
const CONTRIBUTION_WIDTH: f32 = 360.0;
const VALUE_WIDTH: f32 = 118.0;
const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + CONTRIBUTION_WIDTH + VALUE_WIDTH;
const ROW_HEIGHT: f32 = 30.0;
const HEADER_HEIGHT: f32 = 34.0;
const CELL_INSET: f32 = 10.0;

#[derive(Debug, Clone, Copy)]
struct SensitivityView<'a> {
    analysis_label: &'a str,
    output: &'a str,
    result_mode: SensitivityResultMode,
    rows: &'a [SensitivityResultRow],
}

#[derive(Debug, Clone, Copy)]
enum ActiveSensitivity<'a> {
    Ready(SensitivityView<'a>),
    Missing,
    Invalid,
}

/// Select sensitivity evidence from exactly the active analysis.
///
/// A mismatched, failed, or invalid payload is rejected instead of allowing
/// a stale result from another analysis to appear under the current run.
fn active_sensitivity(state: &AppState) -> ActiveSensitivity<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
        return ActiveSensitivity::Missing;
    };
    let Some(
        payload @ AnalysisResultPayload::Sensitivity {
            output,
            result_mode,
            rows,
        },
    ) = analysis.result_payload.as_ref()
    else {
        return ActiveSensitivity::Missing;
    };

    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveSensitivity::Invalid;
    }

    ActiveSensitivity::Ready(SensitivityView {
        analysis_label: analysis.label.as_str(),
        output,
        result_mode: *result_mode,
        rows,
    })
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    matches!(active_sensitivity(state), ActiveSensitivity::Ready(_))
}

/// Rank by contribution magnitude. Equal magnitudes are ordered by the
/// retained parameter identity, giving identical results across platforms.
fn ranked_rows(rows: &[SensitivityResultRow]) -> Vec<&SensitivityResultRow> {
    let mut ranked: Vec<_> = rows.iter().collect();
    ranked.sort_by(|left, right| {
        right
            .normalized
            .abs()
            .total_cmp(&left.normalized.abs())
            .then_with(|| left.parameter.cmp(&right.parameter))
            .then_with(|| right.normalized.total_cmp(&left.normalized))
            .then_with(|| right.raw.total_cmp(&left.raw))
    });
    ranked
}

fn basis_label(result_mode: SensitivityResultMode) -> String {
    match result_mode {
        SensitivityResultMode::Dc => "DC operating point".to_owned(),
        SensitivityResultMode::Ac { frequency_hz } => {
            format!("AC at {}", fmt_si(frequency_hz, "Hz", 4))
        }
    }
}

fn exact_basis_label(result_mode: SensitivityResultMode) -> String {
    match result_mode {
        SensitivityResultMode::Dc => "DC operating point".to_owned(),
        SensitivityResultMode::Ac { frequency_hz } => {
            format!("AC at {} Hz", exact_value(frequency_hz))
        }
    }
}

/// Scientific notation with enough digits to round-trip the retained f64.
fn exact_value(value: f64) -> String {
    format!("{value:.17e}")
}

fn chart_value(value: f64) -> String {
    if value == 0.0 {
        "0".to_owned()
    } else {
        format!("{value:+.6e}")
    }
}

fn column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width, row.height()),
    )
}

fn paint_cell(
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

/// Render the retained sensitivity contribution chart.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let view = match active_sensitivity(state) {
        ActiveSensitivity::Ready(view) => view,
        ActiveSensitivity::Missing => {
            well_hint(ui, "Select an analysis with a retained sensitivity result");
            return;
        }
        ActiveSensitivity::Invalid => {
            well_hint(
                ui,
                "The retained sensitivity result is invalid and cannot be displayed",
            );
            return;
        }
    };
    if view.rows.is_empty() {
        well_hint(
            ui,
            "The retained sensitivity result has no parameter contributions",
        );
        return;
    }

    let ranked = ranked_rows(view.rows);
    let max_magnitude = ranked
        .iter()
        .map(|row| row.normalized.abs())
        .fold(0.0_f64, f64::max);
    if !max_magnitude.is_finite() {
        well_hint(
            ui,
            "The retained sensitivity result is invalid and cannot be displayed",
        );
        return;
    }

    let subtitle = format!(
        "{} - {} - {} parameters",
        view.output,
        basis_label(view.result_mode),
        ranked.len()
    );
    StripHeader::new("SENS", &subtitle, &[]).show(ui);

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let viewport_width = ui.available_width().max(1.0);
    egui::ScrollArea::both()
        .id_salt("rspice.results.sensitivity-contribution")
        .auto_shrink([false, false])
        .show(ui, |ui| {
            let width = viewport_width.max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);

            let contribution_width = width - RANK_WIDTH - PARAMETER_WIDTH - VALUE_WIDTH;
            let contribution_offset = RANK_WIDTH + PARAMETER_WIDTH;
            let value_offset = contribution_offset + contribution_width;
            let (header, _) =
                ui.allocate_exact_size(egui::vec2(width, HEADER_HEIGHT), Sense::hover());
            ui.painter().hline(
                header.x_range(),
                header.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            let header_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            paint_cell(
                ui,
                column_rect(header, 0.0, RANK_WIDTH),
                "#",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_cell(
                ui,
                column_rect(header, RANK_WIDTH, PARAMETER_WIDTH),
                "PARAMETER",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_cell(
                ui,
                column_rect(header, contribution_offset, contribution_width),
                "NORMALIZED CONTRIBUTION",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_cell(
                ui,
                column_rect(header, value_offset, VALUE_WIDTH),
                "VALUE",
                egui::Align2::RIGHT_CENTER,
                header_font,
                c.text_faint,
            );

            let chart_cell = column_rect(header, contribution_offset, contribution_width)
                .shrink2(egui::vec2(CELL_INSET, 0.0));
            let zero_x = chart_cell.center().x;
            let scale_text = format!("{max_magnitude:.6e}");
            ui.painter().text(
                egui::pos2(chart_cell.left(), header.bottom() - 4.0),
                egui::Align2::LEFT_BOTTOM,
                format!("-{scale_text}"),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );
            ui.painter().text(
                egui::pos2(zero_x, header.bottom() - 4.0),
                egui::Align2::CENTER_BOTTOM,
                "0",
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );
            ui.painter().text(
                egui::pos2(chart_cell.right(), header.bottom() - 4.0),
                egui::Align2::RIGHT_BOTTOM,
                scale_text,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );

            for (rank, row) in ranked.iter().enumerate() {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), Sense::hover());
                let accessible_label = format!(
                    "Rank {}, parameter {}, normalized contribution {}, raw sensitivity {}",
                    rank + 1,
                    row.parameter,
                    exact_value(row.normalized),
                    exact_value(row.raw),
                );
                response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Label,
                        ui.is_enabled(),
                        accessible_label.clone(),
                    )
                });
                ui.ctx().accesskit_node_builder(response.id, |node| {
                    node.set_role(egui::accesskit::Role::Row);
                });
                let hovered = response.hovered();
                let response = response.on_hover_ui(|ui| {
                    ui.label(
                        egui::RichText::new(row.parameter.as_str())
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
                    );
                    ui.label(format!("Normalized: {}", exact_value(row.normalized)));
                    ui.label(format!("Raw: {}", exact_value(row.raw)));
                    ui.label(format!("Output: {}", view.output));
                    ui.label(format!("Basis: {}", exact_basis_label(view.result_mode)));
                });

                if !ui.is_rect_visible(rect) {
                    continue;
                }
                if hovered {
                    ui.painter().rect_filled(rect, 0.0, c.bg_hover);
                }
                ui.painter().hline(
                    rect.x_range(),
                    rect.bottom() - 0.5,
                    egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                );

                paint_cell(
                    ui,
                    column_rect(rect, 0.0, RANK_WIDTH),
                    rank + 1,
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );
                paint_cell(
                    ui,
                    column_rect(rect, RANK_WIDTH, PARAMETER_WIDTH),
                    row.parameter.as_str(),
                    egui::Align2::LEFT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );

                let contribution_cell = column_rect(rect, contribution_offset, contribution_width);
                let bar_area = contribution_cell.shrink2(egui::vec2(CELL_INSET, 7.0));
                let center_x = bar_area.center().x;
                let half_width = bar_area.width() * 0.5;
                let ratio = if max_magnitude > 0.0 {
                    (row.normalized.abs() / max_magnitude).clamp(0.0, 1.0) as f32
                } else {
                    0.0
                };
                let signed_width = half_width * ratio;
                let (left, right) = match row.normalized.total_cmp(&0.0) {
                    Ordering::Less => (center_x - signed_width, center_x),
                    Ordering::Equal => (center_x - 0.5, center_x + 0.5),
                    Ordering::Greater => (center_x, center_x + signed_width),
                };
                let bar_color = if row.normalized < 0.0 {
                    c.traces[2]
                } else {
                    c.accent
                };
                let contribution_painter = ui.painter().with_clip_rect(contribution_cell);
                contribution_painter.vline(
                    center_x,
                    rect.y_range(),
                    egui::Stroke::new(1.0, c.border_strong),
                );
                contribution_painter.rect_filled(
                    egui::Rect::from_min_max(
                        egui::pos2(left, bar_area.top()),
                        egui::pos2(right, bar_area.bottom()),
                    ),
                    2.0,
                    bar_color.gamma_multiply(if hovered { 0.92 } else { 0.72 }),
                );

                paint_cell(
                    ui,
                    column_rect(rect, value_offset, VALUE_WIDTH),
                    chart_value(row.normalized),
                    egui::Align2::RIGHT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );
                theme::paint_focus_ring(ui, &response, rect);
            }
        });
}

/// Render output/basis context and the exact ranked contribution table.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let view = match active_sensitivity(state) {
        ActiveSensitivity::Ready(view) => view,
        ActiveSensitivity::Missing => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "Select an analysis with retained sensitivity data.");
            return;
        }
        ActiveSensitivity::Invalid => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "The retained sensitivity payload is invalid.");
            return;
        }
    };

    section_header(ui, "Sensitivity", None);
    let basis = exact_basis_label(view.result_mode);
    let count = view.rows.len().to_string();
    measurement_table(
        ui,
        &[
            ("Analysis", view.analysis_label),
            ("Output", view.output),
            ("Basis", basis.as_str()),
            ("Parameters", count.as_str()),
        ],
    );

    section_header(ui, "Ranked contribution", None);
    if view.rows.is_empty() {
        panel_note(ui, "No parameter contributions were retained.");
        return;
    }

    let ranked = ranked_rows(view.rows);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.sensitivity-ranked-table")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(470.0);
            egui::Grid::new("rspice.results.sensitivity-ranked-grid")
                .num_columns(4)
                .striped(true)
                .min_col_width(48.0)
                .spacing(egui::vec2(12.0, 6.0))
                .show(ui, |ui| {
                    let header_color = Tokens::get(ui.ctx()).color.text_faint;
                    let heading = |text: &str| {
                        egui::RichText::new(text)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(header_color)
                    };
                    ui.label(heading("RANK"));
                    ui.label(heading("PARAMETER"));
                    ui.label(heading("NORMALIZED"));
                    ui.label(heading("RAW"));
                    ui.end_row();

                    for (rank, row) in ranked.iter().enumerate() {
                        ui.label(
                            egui::RichText::new((rank + 1).to_string())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(row.parameter.as_str())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(exact_value(row.normalized))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(exact_value(row.raw))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.end_row();
                    }
                });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

    fn sensitivity_result(id: u64, label: &str, rows: Vec<SensitivityResultRow>) -> AnalysisResult {
        AnalysisResult::new(id, AnalysisType::Sensitivity, label).with_result_payload(
            AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows,
            },
        )
    }

    fn state_with_analyses(analyses: Vec<AnalysisResult>) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        for analysis in analyses {
            run.add_analysis(analysis);
        }
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
    }

    #[test]
    fn ranking_is_deterministic_by_magnitude_then_parameter() {
        let rows = vec![
            SensitivityResultRow {
                parameter: "zeta".to_owned(),
                raw: 3.0,
                normalized: -2.0,
            },
            SensitivityResultRow {
                parameter: "alpha".to_owned(),
                raw: 2.0,
                normalized: 2.0,
            },
            SensitivityResultRow {
                parameter: "middle".to_owned(),
                raw: 1.0,
                normalized: 0.5,
            },
        ];

        let names: Vec<_> = ranked_rows(&rows)
            .into_iter()
            .map(|row| row.parameter.as_str())
            .collect();

        assert_eq!(names, ["alpha", "zeta", "middle"]);
    }

    #[test]
    fn selection_reads_only_the_active_analysis() {
        let rows = vec![SensitivityResultRow {
            parameter: "r1".to_owned(),
            raw: 1.0,
            normalized: 0.25,
        }];
        let mut state = state_with_analyses(vec![
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN"),
            sensitivity_result(2, "SENS", rows),
        ]);

        assert!(matches!(
            active_sensitivity(&state),
            ActiveSensitivity::Missing
        ));
        assert!(state.simulation.select_analysis(1));
        let ActiveSensitivity::Ready(view) = active_sensitivity(&state) else {
            panic!("the selected sensitivity payload should be available");
        };
        assert_eq!(view.analysis_label, "SENS");
        assert_eq!(view.output, "V(out)");
        assert_eq!(view.rows.len(), 1);
    }

    #[test]
    fn invalid_payload_is_rejected_fail_closed() {
        let mut invalid = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS");
        invalid.result_payload = Some(AnalysisResultPayload::Sensitivity {
            output: "V(out)".to_owned(),
            result_mode: SensitivityResultMode::Dc,
            rows: vec![SensitivityResultRow {
                parameter: "r1".to_owned(),
                raw: f64::NAN,
                normalized: 1.0,
            }],
        });
        let state = state_with_analyses(vec![invalid]);

        assert!(matches!(
            active_sensitivity(&state),
            ActiveSensitivity::Invalid
        ));
    }

    #[test]
    fn exact_values_round_trip_and_ac_basis_keeps_frequency() {
        let value = 1.234_567_890_123_456_7e-9;
        assert_eq!(exact_value(value).parse::<f64>().unwrap(), value);
        let basis = exact_basis_label(SensitivityResultMode::Ac {
            frequency_hz: 2.5e6,
        });
        assert!(basis.contains(&exact_value(2.5e6)));
    }
}
