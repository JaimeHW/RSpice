//! Retained single-point sensitivity charts and exact readouts.

use crate::{
    presentation::{panel_note, well_hint},
    strip::StripHeader,
    virtual_rows::RowOffsets,
};
use egui::{Sense, Ui};
use rspice_results::sensitivity::{
    SensitivityResultMode, SensitivityResultRow, SensitivityUnavailability, SensitivityValue,
};
use rspice_ui_kit::{
    plot::fmt_si,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{measurement_table, section_header},
};
use std::cmp::Ordering;
pub mod mismatch;
pub mod study;

const RANK_WIDTH: f32 = 44.0;
const PARAMETER_WIDTH: f32 = 230.0;
const SENSITIVITY_WIDTH: f32 = 360.0;
const VALUE_WIDTH: f32 = 118.0;
const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + SENSITIVITY_WIDTH + VALUE_WIDTH;
const ROW_HEIGHT: f32 = 30.0;
const HEADER_HEIGHT: f32 = 34.0;
const CELL_INSET: f32 = 10.0;
/// Height of one row of the panel's ranked table, spacing included.
const PANEL_ROW_HEIGHT: f32 = 20.0;
const NOT_RETAINED: &str = "Not retained by sensitivity result";

#[derive(Debug, Clone, Copy)]
pub struct SensitivityView<'a> {
    pub analysis_label: &'a str,
    pub output: &'a str,
    pub result_mode: SensitivityResultMode,
    pub rows: &'a [SensitivityResultRow],
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveSensitivity<'a> {
    Ready(SensitivityView<'a>),
    Missing,
    Invalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensitivityPlan {
    order: Vec<usize>,
    /// The chart's row offsets, built once with the order they address.
    ///
    /// The rows are uniform, but the prefix sum over them is `O(parameters)`
    /// and it was rebuilt inside the scroll area on every frame — on the same
    /// sheet whose whole point is that a real design ranks thousands of them.
    offsets: RowOffsets,
    max_magnitude: f64,
}

impl SensitivityPlan {
    pub fn max_magnitude(&self) -> f64 {
        self.max_magnitude
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }

    pub fn new(rows: &[SensitivityResultRow]) -> Self {
        let order = ranked_rows(rows);
        let max_magnitude = order
            .iter()
            .filter_map(|index| rows[*index].normalized.value().map(f64::abs))
            .fold(0.0_f64, f64::max);
        Self {
            offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, order.len())),
            order,
            max_magnitude,
        }
    }
}
fn ranked_rows(rows: &[SensitivityResultRow]) -> Vec<usize> {
    let mut ranked: Vec<usize> = (0..rows.len()).collect();
    ranked.sort_by(|left, right| {
        let (left, right) = (&rows[*left], &rows[*right]);
        match (left.normalized.value(), right.normalized.value()) {
            (Some(left), Some(right)) => right.abs().total_cmp(&left.abs()),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
        .then_with(|| left.parameter.cmp(&right.parameter))
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

fn unavailable_reason(reason: SensitivityUnavailability) -> &'static str {
    match reason {
        SensitivityUnavailability::ZeroOutput => "zero output",
        SensitivityUnavailability::NondifferentiableMagnitude => {
            "magnitude derivative is undefined"
        }
        SensitivityUnavailability::OutOfRange => "outside numeric range",
        SensitivityUnavailability::InvalidInput => "invalid input",
    }
}

fn exact_sensitivity(value: SensitivityValue<f64>) -> String {
    match value {
        SensitivityValue::Available(value) => exact_value(value),
        SensitivityValue::Unavailable { unavailable } => {
            format!("Unavailable ({})", unavailable_reason(unavailable))
        }
    }
}

fn chart_value(value: SensitivityValue<f64>) -> String {
    let Some(value) = value.value() else {
        return "Unavailable".to_owned();
    };
    if value == 0.0 {
        "0".to_owned()
    } else {
        format!("{value:+.6e}")
    }
}

fn highest_magnitude_sensitivity_label(rows: &[SensitivityResultRow], ranked: &[usize]) -> String {
    ranked
        .iter()
        .find(|index| rows[**index].normalized.value().is_some())
        .map_or_else(
            || {
                if rows.is_empty() {
                    "Not retained"
                } else {
                    "No normalized sensitivity available"
                }
                .to_owned()
            },
            |index| {
                let row = &rows[*index];
                format!(
                    "{} · {} normalized sensitivity",
                    row.parameter,
                    exact_sensitivity(row.normalized)
                )
            },
        )
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

pub fn show(ui: &mut Ui, source: ActiveSensitivity<'_>, plan: Option<&SensitivityPlan>) {
    let view = match source {
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
            "The retained sensitivity result has no parameter sensitivities",
        );
        return;
    }

    let Some(plan) = plan else {
        well_hint(
            ui,
            "The retained sensitivity result is invalid and cannot be displayed",
        );
        return;
    };
    let ranked = plan.order();
    let max_magnitude = plan.max_magnitude;
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
        .show_viewport(ui, |ui, viewport| {
            let width = viewport_width.max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);

            let sensitivity_width = width - RANK_WIDTH - PARAMETER_WIDTH - VALUE_WIDTH;
            let sensitivity_offset = RANK_WIDTH + PARAMETER_WIDTH;
            let value_offset = sensitivity_offset + sensitivity_width;
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
                column_rect(header, sensitivity_offset, sensitivity_width),
                "NORMALIZED SENSITIVITY",
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

            let chart_cell = column_rect(header, sensitivity_offset, sensitivity_width)
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

            // One row per swept parameter: a real design ranks thousands, and
            // only the ones on screen are worth laying out. The offsets come
            // from the plan, built with the ranking they address.
            let rows = plan.offsets().plan(egui::Rangef::new(
                viewport.min.y - HEADER_HEIGHT,
                viewport.max.y - HEADER_HEIGHT,
            ));
            ui.allocate_space(egui::vec2(width, rows.leading));
            for (rank, row) in ranked
                .iter()
                .map(|index| &view.rows[*index])
                .enumerate()
                .skip(rows.first)
                .take(rows.end - rows.first)
            {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), Sense::hover());
                let accessible_label = format!(
                    "{}, parameter {}, normalized sensitivity {}, raw sensitivity {}",
                    if row.normalized.value().is_some() {
                        format!("Rank {}", rank + 1)
                    } else {
                        "Unranked".to_owned()
                    },
                    row.parameter,
                    exact_sensitivity(row.normalized),
                    exact_sensitivity(row.raw),
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
                    ui.label(format!(
                        "Normalized sensitivity: {}",
                        exact_sensitivity(row.normalized)
                    ));
                    ui.label(format!("Raw: {}", exact_sensitivity(row.raw)));
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
                    if row.normalized.value().is_some() {
                        (rank + 1).to_string()
                    } else {
                        "—".to_owned()
                    },
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

                let sensitivity_cell = column_rect(rect, sensitivity_offset, sensitivity_width);
                if let SensitivityValue::Available(normalized) = row.normalized {
                    let bar_area = sensitivity_cell.shrink2(egui::vec2(CELL_INSET, 7.0));
                    let center_x = bar_area.center().x;
                    let half_width = bar_area.width() * 0.5;
                    let ratio = if max_magnitude > 0.0 {
                        (normalized.abs() / max_magnitude).clamp(0.0, 1.0) as f32
                    } else {
                        0.0
                    };
                    let signed_width = half_width * ratio;
                    let (left, right) = match normalized.total_cmp(&0.0) {
                        Ordering::Less => (center_x - signed_width, center_x),
                        Ordering::Equal => (center_x - 0.5, center_x + 0.5),
                        Ordering::Greater => (center_x, center_x + signed_width),
                    };
                    let bar_color = if normalized < 0.0 {
                        c.traces[2]
                    } else {
                        c.accent
                    };
                    let sensitivity_painter = ui.painter().with_clip_rect(sensitivity_cell);
                    sensitivity_painter.vline(
                        center_x,
                        rect.y_range(),
                        egui::Stroke::new(1.0, c.border_strong),
                    );
                    sensitivity_painter.rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(left, bar_area.top()),
                            egui::pos2(right, bar_area.bottom()),
                        ),
                        2.0,
                        bar_color.gamma_multiply(if hovered { 0.92 } else { 0.72 }),
                    );
                } else if let SensitivityValue::Unavailable { unavailable } = row.normalized {
                    paint_cell(
                        ui,
                        sensitivity_cell,
                        unavailable_reason(unavailable),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    );
                }

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
            ui.allocate_space(egui::vec2(width, rows.trailing));
        });
}
pub fn right_panel(ui: &mut Ui, source: ActiveSensitivity<'_>, plan: Option<&SensitivityPlan>) {
    let view = match source {
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
    let ranked = plan.map_or(&[][..], SensitivityPlan::order);
    let highest_magnitude = highest_magnitude_sensitivity_label(view.rows, ranked);
    measurement_table(
        ui,
        &[
            ("Analysis", view.analysis_label),
            ("Reference metric", view.output),
            ("Basis", basis.as_str()),
            // What this payload actually differentiated against. It was
            // recorded before the Studio ran the engine's own filter, so its
            // variables are the deck's design parameters and its rows carry
            // bare names rather than the engine's `PARAM:` spelling. Saying
            // so is the whole migration: a reader can tell this report from a
            // current one without guessing from the names.
            ("Variables", "design parameters · recorded before filters"),
            ("Method", NOT_RETAINED),
            ("Normalization", "Retained per parameter"),
            ("Parameters retained", count.as_str()),
            ("Highest magnitude", highest_magnitude.as_str()),
            ("Cross-terms", NOT_RETAINED),
        ],
    );

    section_header(ui, "Ranked sensitivity", None);
    if view.rows.is_empty() {
        panel_note(ui, "No parameter sensitivities were retained.");
        return;
    }

    // The panel ranks the same thousands of parameters the chart does, so it
    // lays out only the rows its own viewport can show.
    let header_color = Tokens::get(ui.ctx()).color.text_faint;
    egui::ScrollArea::both()
        .id_salt("rspice.results.sensitivity-ranked-table")
        .auto_shrink([false, true])
        .show_rows(ui, PANEL_ROW_HEIGHT, ranked.len() + 1, |ui, visible| {
            ui.set_min_width(470.0);
            egui::Grid::new("rspice.results.sensitivity-ranked-grid")
                .num_columns(4)
                .striped(true)
                .min_col_width(48.0)
                .spacing(egui::vec2(12.0, 6.0))
                .show(ui, |ui| {
                    let heading = |text: &str| {
                        egui::RichText::new(text)
                            .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                            .color(header_color)
                    };
                    if visible.start == 0 {
                        ui.label(heading("RANK"));
                        ui.label(heading("PARAMETER"));
                        ui.label(heading("NORMALIZED"));
                        ui.label(heading("RAW"));
                        ui.end_row();
                    }
                    // Row 0 of the virtual list is the heading, so the data
                    // rows start one behind it.
                    let first = visible.start.saturating_sub(1);
                    let last = visible.end.saturating_sub(1).min(ranked.len());
                    for (rank, row) in ranked[first..last]
                        .iter()
                        .map(|index| &view.rows[*index])
                        .enumerate()
                        .map(|(offset, row)| (first + offset, row))
                    {
                        ui.label(
                            egui::RichText::new(if row.normalized.value().is_some() {
                                (rank + 1).to_string()
                            } else {
                                "—".to_owned()
                            })
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(row.parameter.as_str())
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(exact_sensitivity(row.normalized))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular)),
                        );
                        ui.label(
                            egui::RichText::new(exact_sensitivity(row.raw))
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
    #[test]
    fn ranking_is_deterministic_by_magnitude_then_parameter() {
        let rows = vec![
            SensitivityResultRow {
                parameter: "zeta".to_owned(),
                raw: (3.0).into(),
                normalized: (-2.0).into(),
            },
            SensitivityResultRow {
                parameter: "alpha".to_owned(),
                raw: (2.0).into(),
                normalized: (2.0).into(),
            },
            SensitivityResultRow {
                parameter: "middle".to_owned(),
                raw: (1.0).into(),
                normalized: (0.5).into(),
            },
        ];

        let names: Vec<_> = ranked_rows(&rows)
            .into_iter()
            .map(|index| rows[index].parameter.as_str())
            .collect();

        assert_eq!(names, ["alpha", "zeta", "middle"]);
    }
    #[test]
    fn unavailable_rows_remain_visible_without_ranking_as_zero() {
        let rows = vec![
            SensitivityResultRow {
                parameter: "a_unavailable".to_owned(),
                raw: 1.0.into(),
                normalized: SensitivityValue::unavailable(SensitivityUnavailability::ZeroOutput),
            },
            SensitivityResultRow {
                parameter: "b_zero".to_owned(),
                raw: 0.0.into(),
                normalized: 0.0.into(),
            },
            SensitivityResultRow {
                parameter: "c_largest".to_owned(),
                raw: 2.0.into(),
                normalized: 3.0.into(),
            },
        ];
        let plan = SensitivityPlan::new(&rows);
        assert_eq!(plan.order(), &[2, 1, 0]);
        assert_eq!(plan.max_magnitude, 3.0);
        assert_eq!(chart_value(rows[0].normalized), "Unavailable");
        assert!(exact_sensitivity(rows[0].normalized).contains("zero output"));
        assert_eq!(chart_value(rows[1].normalized), "0");
        assert!(highest_magnitude_sensitivity_label(&rows, plan.order()).starts_with("c_largest"));
        let unavailable = vec![rows[0].clone()];
        assert_eq!(
            highest_magnitude_sensitivity_label(&unavailable, &[0]),
            "No normalized sensitivity available"
        );
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
    #[test]
    fn highest_magnitude_label_uses_retained_normalized_ranking() {
        let rows = vec![
            SensitivityResultRow {
                parameter: "small".to_owned(),
                raw: (100.0).into(),
                normalized: (0.25).into(),
            },
            SensitivityResultRow {
                parameter: "largest".to_owned(),
                raw: (1.0).into(),
                normalized: (-0.75).into(),
            },
        ];

        let label = highest_magnitude_sensitivity_label(&rows, &ranked_rows(&rows));

        assert!(label.starts_with("largest · "));
        assert!(label.contains(&exact_value(-0.75)));
        assert_eq!(
            highest_magnitude_sensitivity_label(&[], &[]),
            "Not retained"
        );
    }
}
