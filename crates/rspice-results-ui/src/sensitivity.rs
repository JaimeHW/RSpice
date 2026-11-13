//! Retained single-point sensitivity charts and exact readouts.

use crate::{
    op_inspector::op_column_rect as column_rect,
    presentation::{panel_note, table_row_height, well_hint},
    strip::StripHeader,
    virtual_rows::RowOffsets,
};
use egui::{Sense, Ui};
use rspice_results::sensitivity::{
    SensitivityResultMode, SensitivityResultRow, SensitivityUnavailability, SensitivityValue,
};
use rspice_ui_kit::{
    panels::{panel_body, section_header},
    plot::fmt_si,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::measurement_table,
};
use std::cmp::Ordering;
pub mod mismatch;
pub mod study;

const RANK_WIDTH: f32 = 44.0;
const PARAMETER_WIDTH: f32 = 230.0;
const SENSITIVITY_WIDTH: f32 = 360.0;
const VALUE_WIDTH: f32 = 118.0;
const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + SENSITIVITY_WIDTH + VALUE_WIDTH;
const CELL_INSET: f32 = 10.0;
/// Height of one row of the panel's ranked table, spacing included.
const PANEL_ROW_HEIGHT: f32 = 20.0;

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

    /// The ranking of `rows`, its scale and its row offsets. The offsets
    /// count rows: the chart's row height follows the density preference,
    /// which the plan is built without, so the chart scales them to points.
    pub fn new(rows: &[SensitivityResultRow]) -> Self {
        let order = ranked_rows(rows);
        let max_magnitude = order
            .iter()
            .filter_map(|index| rows[*index].normalized.value().map(f64::abs))
            .fold(0.0_f64, f64::max);
        Self {
            offsets: RowOffsets::from_heights(std::iter::repeat_n(1.0, order.len())),
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
            format!(
                "AC at {}",
                rspice_ui_kit::plot::fmt_si_readout(frequency_hz, "Hz", 4)
            )
        }
    }
}

/// A sensitivity as the panel reads it: engineering notation at the precision
/// a ranked list is read at, its prefix on the number as every unitless value
/// in the product reads (`1.23457n`, never `1.23457 n`). The CSV export
/// carries the retained f64.
fn exact_value(value: f64) -> String {
    unitless(value, 6)
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

/// A chart row's value in engineering notation at the reader's significant
/// digits, as the sheets' other tables state theirs. A normalized
/// sensitivity has no unit, so its prefix rides the number (`-1.25m`, never
/// `-1.25 m`, which reads as metres). It is signed: the bars beside it grow
/// both ways from zero, and the sign is what tells a reader which way the
/// output moves. The row's hover keeps the panel's exact reading.
fn chart_value(value: SensitivityValue<f64>, significant_digits: usize) -> String {
    let Some(value) = value.value() else {
        return "Unavailable".to_owned();
    };
    let shown = unitless(value, significant_digits);
    if value > 0.0 {
        format!("+{shown}")
    } else {
        shown
    }
}

/// The chart's full scale, as one magnitude both ends of its bars share.
fn chart_scale(max_magnitude: f64, significant_digits: usize) -> String {
    format!("±{}", unitless(max_magnitude, significant_digits))
}

/// A unitless value in engineering notation, its prefix on the number.
fn unitless(value: f64, significant_digits: usize) -> String {
    rspice_ui_kit::plot::fmt_si_readout(value, "", significant_digits).replace(' ', "")
}

/// The Value column, wide enough for any value at `significant_digits`: a
/// sign, three integer digits, the rest as decimals and a prefix. It never
/// narrows below the width the column was drawn at.
fn value_width(ui: &Ui, significant_digits: usize) -> f32 {
    let widest = format!(
        "+888.{}m",
        "8".repeat(significant_digits.saturating_sub(3).max(1))
    );
    let text = ui
        .painter()
        .layout_no_wrap(
            widest,
            theme::mono(tokens::FS_1, FontWeight::Regular),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    (text + 2.0 * CELL_INSET + 4.0).max(VALUE_WIDTH)
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

/// The ranked chart. Its values are stated at `significant_digits`, the
/// reader's Results preference.
pub fn show(
    ui: &mut Ui,
    source: ActiveSensitivity<'_>,
    plan: Option<&SensitivityPlan>,
    significant_digits: usize,
) {
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
    // The dense row of the sheet's other tables, and the header is one of
    // them. The rows abut, each drawing its own rule, so a row's pitch is its
    // height and the plan's offsets place it exactly.
    let row_height = table_row_height(ui);
    egui::ScrollArea::both()
        .id_salt("rspice.results.sensitivity-contribution")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            ui.spacing_mut().item_spacing.y = 0.0;
            // The viewport is what the rows can use: measured before the
            // scroll area, the width still held the vertical bar's track,
            // and a sheet long enough to scroll cut its Value column off by
            // the bar's width.
            let value_width = value_width(ui, significant_digits);
            let width = viewport
                .width()
                .max(TABLE_MIN_WIDTH - VALUE_WIDTH + value_width);
            ui.set_min_width(width);

            let sensitivity_width = width - RANK_WIDTH - PARAMETER_WIDTH - value_width;
            let sensitivity_offset = RANK_WIDTH + PARAMETER_WIDTH;
            let value_offset = sensitivity_offset + sensitivity_width;
            let (header, _) = ui.allocate_exact_size(egui::vec2(width, row_height), Sense::hover());
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
                "Parameter",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_cell(
                ui,
                column_rect(header, sensitivity_offset, sensitivity_width),
                "Normalized sensitivity",
                egui::Align2::LEFT_CENTER,
                header_font.clone(),
                c.text_faint,
            );
            paint_cell(
                ui,
                column_rect(header, value_offset, value_width),
                "Value",
                egui::Align2::RIGHT_CENTER,
                header_font,
                c.text_faint,
            );

            // The chart's full scale shares the column's header row, at its
            // right edge: the bars grow both ways from the zero rule every row
            // draws through its middle, so one magnitude states both ends.
            paint_cell(
                ui,
                column_rect(header, sensitivity_offset, sensitivity_width),
                chart_scale(max_magnitude, significant_digits),
                egui::Align2::RIGHT_CENTER,
                theme::mono(tokens::FS_0, FontWeight::Regular),
                c.text_faint,
            );

            // One row per swept parameter: a real design ranks thousands, and
            // only the ones on screen are worth laying out. The offsets come
            // from the plan, built with the ranking they address.
            let rows = plan.offsets().plan(egui::Rangef::new(
                (viewport.min.y - row_height) / row_height,
                (viewport.max.y - row_height) / row_height,
            ));
            ui.allocate_space(egui::vec2(width, rows.leading * row_height));
            for (rank, row) in ranked
                .iter()
                .map(|index| &view.rows[*index])
                .enumerate()
                .skip(rows.first)
                .take(rows.end - rows.first)
            {
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, row_height), Sense::hover());
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
                    let bar_area = sensitivity_cell
                        .shrink2(egui::vec2(CELL_INSET, (row_height * 0.25).round()));
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
                    column_rect(rect, value_offset, value_width),
                    chart_value(row.normalized, significant_digits),
                    egui::Align2::RIGHT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );
                theme::paint_focus_ring(ui, &response, rect);
            }
            ui.allocate_space(egui::vec2(width, rows.trailing * row_height));
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
    let ranked = plan.map_or(&[][..], SensitivityPlan::order);
    // The analysis is the pane's own name, the parameter count is the length
    // of the table below, rank 1 of that table is the highest magnitude, and
    // the method, normalization and cross-term rows were constants.
    measurement_table(
        ui,
        &[("Reference metric", view.output), ("Basis", basis.as_str())],
    );

    section_header(ui, "Ranked sensitivity", None);
    if view.rows.is_empty() {
        panel_note(ui, "No sensitivities.");
        return;
    }

    // The panel ranks the same thousands of parameters the chart does, so it
    // lays out only the rows its own viewport can show.
    let header_color = Tokens::get(ui.ctx()).color.text_faint;
    // The table scrolls inside the rows' inset, so its first column starts
    // where every label above it does rather than on the panel edge.
    panel_body(ui, |ui| {
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
                            ui.label(heading("Rank"));
                            ui.label(heading("Parameter"));
                            ui.label(heading("Normalized"));
                            ui.label(heading("Raw"));
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
        assert_eq!(chart_value(rows[0].normalized, 7), "Unavailable");
        assert!(exact_sensitivity(rows[0].normalized).contains("zero output"));
        assert_eq!(chart_value(rows[1].normalized, 7), "0");
    }

    /// A chart value reads in engineering notation at the reader's digits,
    /// signed, its prefix on the number; the scale reads the same way.
    #[test]
    fn chart_values_read_in_engineering_notation_at_the_readers_digits() {
        assert_eq!(chart_value(3.0.into(), 7), "+3");
        assert_eq!(chart_value((-1.234_567_89e-3).into(), 7), "-1.234568m");
        assert_eq!(chart_value(2.718_281_828e4.into(), 3), "+27.2k");
        assert_eq!(chart_scale(0.012_5, 7), "±12.5m");
    }

    /// A panel value reads in engineering notation with its prefix on the
    /// number, as the chart's values do, and an AC basis names
    /// the frequency it was read at in the reader's own units rather than to
    /// seventeen digits.
    #[test]
    fn panel_values_read_in_engineering_notation() {
        assert_eq!(exact_value(1.234_567_890_123_456_7e-9), "1.23457n");
        assert_eq!(exact_value(-2.5e3), "-2.5k");
        assert_eq!(exact_value(0.75), "750m");
        assert_eq!(
            exact_basis_label(SensitivityResultMode::Ac {
                frequency_hz: 2.5e6,
            }),
            "AC at 2.5 MHz"
        );
    }
}
