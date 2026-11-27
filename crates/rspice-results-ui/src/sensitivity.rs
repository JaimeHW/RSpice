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
    panels::section_header,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{
        measurement_table,
        table::{Cell, RowKey, SortKey},
    },
};
use rspice_vocabulary::quantity::QuantityPresentationPolicy;
use std::cmp::Ordering;
pub mod mismatch;
mod ranked_table;
pub mod study;
#[cfg(test)]
mod table_tests;

const RANK_WIDTH: f32 = 44.0;
const PARAMETER_WIDTH: f32 = 230.0;
const SENSITIVITY_WIDTH: f32 = 360.0;
const VALUE_WIDTH: f32 = 118.0;
const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + SENSITIVITY_WIDTH + VALUE_WIDTH;
const CELL_INSET: f32 = 10.0;

#[derive(Debug, Clone, Copy)]
pub struct SensitivityView<'a> {
    pub analysis_label: &'a str,
    pub output: &'a str,
    pub result_mode: SensitivityResultMode,
    pub rows: &'a [SensitivityResultRow],
    /// How a copied row writes its values.
    pub quantity_policy: QuantityPresentationPolicy,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveSensitivity<'a> {
    Ready(SensitivityView<'a>),
    Missing,
    Invalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SensitivityPlan {
    /// The rows the sheet's filter keeps, ranked.
    order: Vec<usize>,
    /// Each listed row's place in the whole ranking, the first 0: a filter
    /// narrows the list, never renumbers it.
    ranks: Vec<usize>,
    /// How many rows the whole ranking holds.
    total: usize,
    /// The chart's row offsets, built once with the order they address.
    ///
    /// The rows are uniform, but the prefix sum over them is `O(parameters)`
    /// and it was rebuilt inside the scroll area on every frame — on the same
    /// sheet whose whole point is that a real design ranks thousands of them.
    offsets: RowOffsets,
    max_magnitude: f64,
    /// The listed rows' identities, so the table's selection follows its row
    /// through a sort or a filter.
    keys: Vec<RowKey>,
    revision: u64,
}

impl SensitivityPlan {
    pub fn max_magnitude(&self) -> f64 {
        self.max_magnitude
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn ranks(&self) -> &[usize] {
        &self.ranks
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }

    /// The ranking of `rows`, the ones `filter` keeps listed, the chart's
    /// scale and its row offsets. The scale is the whole ranking's, so a
    /// bar keeps its length as the reader narrows the list. The offsets
    /// count rows: the chart's row height follows the density preference,
    /// which the plan is built without, so the chart scales them to points.
    pub fn new(rows: &[SensitivityResultRow], filter: &str) -> Self {
        let ranking = ranked_rows(rows);
        let max_magnitude = ranking
            .iter()
            .filter_map(|index| rows[*index].normalized.value().map(f64::abs))
            .fold(0.0_f64, f64::max);
        let (ranks, order): (Vec<usize>, Vec<usize>) = ranking
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, index)| kept_by_filter(filter, &[&rows[*index].parameter]))
            .unzip();
        let keys: Vec<RowKey> = order
            .iter()
            .map(|index| RowKey::Text(rows[*index].parameter.clone()))
            .collect();
        let revision = listed_revision(
            &keys,
            order.iter().map(|index| {
                let row = &rows[*index];
                [row.normalized.value(), row.raw.value()]
            }),
        );
        Self {
            offsets: RowOffsets::from_heights(std::iter::repeat_n(1.0, order.len())),
            total: rows.len(),
            order,
            ranks,
            max_magnitude,
            keys,
            revision,
        }
    }
}

/// Whether a row named by `names` stays listed under a sheet's filter: one
/// of its names holds the filter's words, ignoring case. An empty filter
/// keeps every row.
pub(crate) fn kept_by_filter(filter: &str, names: &[&str]) -> bool {
    let filter = filter.trim();
    if filter.is_empty() {
        return true;
    }
    let filter = filter.to_lowercase();
    names
        .iter()
        .any(|name| name.to_lowercase().contains(&filter))
}

/// How many rows a sheet lists, and of how many when its filter hides some.
pub(crate) fn listed_count(listed: usize, total: usize, noun: &str) -> String {
    if listed == total {
        format!("{total} {noun}")
    } else {
        format!("{listed} of {total} {noun}")
    }
}

/// What a ranked table's listed rows were built from, so the table sorts
/// and filters them again only when it changes.
pub(crate) fn listed_revision<const N: usize>(
    keys: &[RowKey],
    values: impl Iterator<Item = [Option<f64>; N]>,
) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::hash::DefaultHasher::new();
    keys.hash(&mut hasher);
    for values in values {
        for value in values {
            value.map(f64::to_bits).hash(&mut hasher);
        }
    }
    hasher.finish()
}

/// The sheet's filter, on its bar: it narrows the chart and the table to
/// the rows a parameter or instance name matches.
pub fn filter_field(ui: &mut Ui, filter: &mut String) {
    crate::chrome::bars::sheet_filter_field(
        ui,
        egui::Id::new("rspice.results.sensitivity.filter"),
        filter,
    );
}

/// The sensitivity sheet's bar: a frequency study's read point, then the
/// filter. A frequency study names its evidence and quantity policy.
pub fn domain_bar(
    ui: &mut Ui,
    study: Option<(
        &rspice_results::sensitivity::SensitivityStudyEvidence,
        &rspice_vocabulary::quantity::QuantityPresentationPolicy,
    )>,
    state: &mut study::SensitivitySheetState,
) -> bool {
    if let Some((evidence, quantity_policy)) = study {
        study::frequency_select(ui, evidence, quantity_policy, state);
    }
    filter_field(ui, &mut state.filter);
    true
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
fn exact_basis_label(result_mode: SensitivityResultMode) -> String {
    match result_mode {
        SensitivityResultMode::Dc => "DC operating point".to_owned(),
        SensitivityResultMode::Ac { frequency_hz } => {
            format!(
                "AC at {}",
                rspice_ui_kit::plot::fmt_si_significant(frequency_hz, "Hz", 4)
            )
        }
    }
}

/// A sensitivity as the panel reads it: engineering notation at the precision
/// a ranked list is read at, its prefix on the number as every unitless value
/// in the product reads (`1.23457n`, never `1.23457 n`). The CSV export
/// carries the retained f64.
fn exact_value(value: f64, significant_digits: usize) -> String {
    unitless(value, significant_digits)
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

fn exact_sensitivity(value: SensitivityValue<f64>, significant_digits: usize) -> String {
    match value {
        SensitivityValue::Available(value) => exact_value(value, significant_digits),
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

/// A unit-less value: plain digits, never a prefix.
fn unitless(value: f64, significant_digits: usize) -> String {
    rspice_ui_kit::plot::fmt_column_reading(value, "", significant_digits)
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
            well_hint(ui, "Pick a sensitivity analysis in the data browser");
            return;
        }
        ActiveSensitivity::Invalid => {
            well_hint(
                ui,
                "This sensitivity result is invalid and cannot be displayed",
            );
            return;
        }
    };
    if view.rows.is_empty() {
        well_hint(ui, "This sensitivity result has no parameter sensitivities");
        return;
    }

    let Some(plan) = plan else {
        well_hint(
            ui,
            "This sensitivity result is invalid and cannot be displayed",
        );
        return;
    };
    let ranked = plan.order();
    let max_magnitude = plan.max_magnitude;
    if !max_magnitude.is_finite() {
        well_hint(
            ui,
            "This sensitivity result is invalid and cannot be displayed",
        );
        return;
    }

    let subtitle = format!(
        "{} \u{b7} {} \u{b7} {}",
        view.output,
        exact_basis_label(view.result_mode),
        listed_count(ranked.len(), plan.total(), "parameters")
    );
    StripHeader::new("SENS", &subtitle, &[]).show(ui);
    if ranked.is_empty() {
        well_hint(ui, "No parameter matches the filter");
        return;
    }

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
                .zip(plan.ranks())
                .map(|(index, rank)| (*rank, &view.rows[*index]))
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
                    exact_sensitivity(row.normalized, significant_digits),
                    exact_sensitivity(row.raw, significant_digits),
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
                        exact_sensitivity(row.normalized, significant_digits)
                    ));
                    ui.label(format!(
                        "Raw: {}",
                        exact_sensitivity(row.raw, significant_digits)
                    ));
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
pub fn right_panel(
    ui: &mut Ui,
    source: ActiveSensitivity<'_>,
    plan: Option<&SensitivityPlan>,
    significant_digits: usize,
) {
    let view = match source {
        ActiveSensitivity::Ready(view) => view,
        ActiveSensitivity::Missing => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "Pick a sensitivity analysis in the data browser.");
            return;
        }
        ActiveSensitivity::Invalid => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "This sensitivity result is invalid.");
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
    let Some(plan) = plan else {
        return;
    };
    if ranked.is_empty() {
        panel_note(ui, "No parameter matches the filter.");
        return;
    }
    let listed = |row: usize| (plan.ranks[row], &view.rows[plan.order[row]]);
    ranked_table::ranked_table(
        ui,
        &ranked_table::RankedTable {
            salt: "ranked",
            columns: &[
                ("#", false),
                ("Parameter", true),
                ("Normalized", false),
                ("Raw", false),
            ],
            keys: &plan.keys,
            revision: plan.revision ^ significant_digits as u64,
        },
        |row| {
            let (rank, row) = listed(row);
            vec![
                rank_cell(rank, row.normalized),
                Cell::text(row.parameter.as_str()).sort_by(SortKey::Text(row.parameter.clone())),
                sensitivity_cell(row.normalized, significant_digits),
                sensitivity_cell(row.raw, significant_digits),
            ]
        },
        |row| {
            let (rank, row) = listed(row);
            exact_row(&[
                exact_rank(rank, row.normalized),
                row.parameter.clone(),
                full_precision(row.normalized, &view.quantity_policy),
                full_precision(row.raw, &view.quantity_policy),
            ])
        },
    );
}

/// A row's place in the ranking, from 1, or a dash for a row that ranks
/// nowhere since its sensitivity is unavailable.
fn rank_cell(rank: usize, normalized: SensitivityValue<f64>) -> Cell {
    if normalized.value().is_some() {
        Cell::number((rank + 1).to_string()).sort_by(SortKey::Number(rank as f64))
    } else {
        Cell::absent().sort_by(SortKey::Number(f64::INFINITY))
    }
}

/// A sensitivity in a ranked table at the reader's digits, sorting by its
/// signed value; an unavailable one is a dash saying why on hover.
fn sensitivity_cell(value: SensitivityValue<f64>, significant_digits: usize) -> Cell {
    match value {
        SensitivityValue::Available(value) => {
            Cell::number(exact_value(value, significant_digits)).sort_by(SortKey::Number(value))
        }
        SensitivityValue::Unavailable { unavailable } => {
            Cell::absent().hint(format!("Unavailable: {}", unavailable_reason(unavailable)))
        }
    }
}

/// A row's rank as a copied row states it: from 1, empty when unranked.
fn exact_rank(rank: usize, normalized: SensitivityValue<f64>) -> String {
    normalized
        .value()
        .map_or_else(String::new, |_| (rank + 1).to_string())
}

/// A sensitivity at the precision the run holds it, as Copied values
/// writes it; an unavailable one is empty.
fn full_precision(value: SensitivityValue<f64>, policy: &QuantityPresentationPolicy) -> String {
    value
        .value()
        .map_or_else(String::new, |value| exact_copy(value, "", policy))
}

/// `value` in `unit` as Copied values writes it: every digit the double
/// holds.
fn exact_copy(value: f64, unit: &str, policy: &QuantityPresentationPolicy) -> String {
    policy.copy_si_value(value, unit).trim_end().to_owned()
}

/// A copied row: its fields between tabs, as a spreadsheet pastes them.
fn exact_row(fields: &[String]) -> String {
    fields.join("\t")
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
        let plan = SensitivityPlan::new(&rows, "");
        assert_eq!(plan.order(), &[2, 1, 0]);
        assert_eq!(plan.max_magnitude, 3.0);
        assert_eq!(chart_value(rows[0].normalized, 7), "Unavailable");
        assert!(exact_sensitivity(rows[0].normalized, 6).contains("zero output"));
        assert_eq!(chart_value(rows[1].normalized, 7), "0");
    }

    /// A chart value reads in engineering notation at the reader's digits,
    /// zeros kept, signed, its prefix on the number; the scale reads the
    /// same way.
    #[test]
    fn chart_values_read_in_engineering_notation_at_the_readers_digits() {
        assert_eq!(chart_value(3.0.into(), 4), "+3.000");
        assert_eq!(
            chart_value((-1.234_567_89e-3).into(), 7),
            "\u{2212}0.001234568"
        );
        assert_eq!(chart_value(2.718_281_828e4.into(), 3), "+27200");
        assert_eq!(chart_scale(0.012_5, 4), "±0.01250");
    }

    /// A panel value reads in engineering notation with its prefix on the
    /// number, as the chart's values do, and an AC basis names
    /// the frequency it was read at in the reader's own units rather than to
    /// seventeen digits.
    #[test]
    fn panel_values_read_in_engineering_notation() {
        assert_eq!(
            exact_value(1.234_567_890_123_456_7e-9, 6),
            "1.23457e\u{2212}9"
        );
        assert_eq!(exact_value(-2.5e3, 6), "\u{2212}2500.00");
        assert_eq!(exact_value(0.75, 6), "0.750000");
        assert_eq!(
            exact_basis_label(SensitivityResultMode::Ac {
                frequency_hz: 2.5e6,
            }),
            "AC at 2.5 MHz"
        );
    }
}
