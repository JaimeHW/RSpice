//! SENS - retained, ranked normalized parameter sensitivities.
//!
//! This viewer is intentionally dataset-bound. It reads the exact
//! [`AnalysisResultPayload::Sensitivity`] attached to the selected analysis
//! and never falls back to console text or the legacy mutable analysis cache.

use std::cmp::Ordering;

use egui::{Sense, Ui};

use crate::state::{AnalysisResultPayload, SensitivityResultMode, SensitivityResultRow};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

use std::sync::Arc;

use super::AnalysisPresentationKey;
use super::frame_work::{self, DatasetWalk};
use super::strip::StripHeader;
use super::virtual_rows::RowOffsets;
use super::{panel_note, well_hint};

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

/// Rank by normalized-sensitivity magnitude. Equal magnitudes are ordered by the
/// retained parameter identity, giving identical results across platforms.
fn ranked_rows(rows: &[SensitivityResultRow]) -> Vec<usize> {
    frame_work::note(DatasetWalk::SensitivityRank);
    let mut ranked: Vec<usize> = (0..rows.len()).collect();
    ranked.sort_by(|left, right| {
        let (left, right) = (&rows[*left], &rows[*right]);
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

/// The ranked order of one retained sensitivity result.
///
/// A real design ranks thousands of parameters, and the sort ran on every
/// frame — twice, because the panel ranked them again beside the chart.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SensitivityPlan {
    version: u64,
    analysis: AnalysisPresentationKey,
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
    pub(super) fn order(&self) -> &[usize] {
        &self.order
    }

    pub(super) fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }
}

/// The ranking for the active analysis, sorted once per dataset generation.
fn sensitivity_plan(state: &mut AppState) -> Option<Arc<SensitivityPlan>> {
    let version = state.simulation.data_version;
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    if let Some(plan) = state.ui.results.plans.sensitivity.as_ref()
        && plan.version == version
        && plan.analysis == analysis_key
    {
        return Some(Arc::clone(plan));
    }
    let ActiveSensitivity::Ready(view) = active_sensitivity(state) else {
        return None;
    };
    let order = ranked_rows(view.rows);
    let max_magnitude = order
        .iter()
        .map(|index| view.rows[*index].normalized.abs())
        .fold(0.0_f64, f64::max);
    let built = Arc::new(SensitivityPlan {
        version,
        analysis: analysis_key,
        offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, order.len())),
        order,
        max_magnitude,
    });
    state.ui.results.plans.sensitivity = Some(Arc::clone(&built));
    Some(built)
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

fn highest_magnitude_sensitivity_label(rows: &[SensitivityResultRow], ranked: &[usize]) -> String {
    ranked.first().map_or_else(
        || "Not retained".to_owned(),
        |index| {
            let row = &rows[*index];
            format!(
                "{} · {} normalized sensitivity",
                row.parameter,
                exact_value(row.normalized)
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

/// Render the retained normalized-sensitivity chart.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    // Ranked before the payload is borrowed, so the sort happens once per
    // dataset generation rather than once per frame per surface.
    let plan = sensitivity_plan(state);
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
                    "Rank {}, parameter {}, normalized sensitivity {}, raw sensitivity {}",
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
                    ui.label(format!(
                        "Normalized sensitivity: {}",
                        exact_value(row.normalized)
                    ));
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

                let sensitivity_cell = column_rect(rect, sensitivity_offset, sensitivity_width);
                let bar_area = sensitivity_cell.shrink2(egui::vec2(CELL_INSET, 7.0));
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

/// Render output/basis context and the exact ranked sensitivity table.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let plan = sensitivity_plan(state);
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
    let ranked = plan.as_deref().map_or(&[][..], SensitivityPlan::order);
    let highest_magnitude = highest_magnitude_sensitivity_label(view.rows, ranked);
    measurement_table(
        ui,
        &[
            ("Analysis", view.analysis_label),
            ("Reference metric", view.output),
            ("Basis", basis.as_str()),
            ("Method", NOT_RETAINED),
            ("Normalization", "Retained per parameter"),
            ("Parameters ranked", count.as_str()),
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
            .map(|index| rows[index].parameter.as_str())
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

    #[test]
    fn highest_magnitude_label_uses_retained_normalized_ranking() {
        let rows = vec![
            SensitivityResultRow {
                parameter: "small".to_owned(),
                raw: 100.0,
                normalized: 0.25,
            },
            SensitivityResultRow {
                parameter: "largest".to_owned(),
                raw: 1.0,
                normalized: -0.75,
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

    #[test]
    fn unavailable_solver_contracts_are_explicit() {
        assert_eq!(NOT_RETAINED, "Not retained by sensitivity result");
    }

    fn ranked_state(parameters: usize) -> AppState {
        let rows = (0..parameters)
            .map(|index| SensitivityResultRow {
                parameter: format!("p{index:05}"),
                raw: index as f64,
                normalized: (index as f64).sin(),
            })
            .collect();
        let mut state = state_with_analyses(vec![sensitivity_result(1, "SENS", rows)]);
        assert!(state.simulation.select_analysis(0));
        state
    }

    /// The ranking is the sort it replaced, done once. Both the chart and the
    /// panel read it, and neither may see a ranking of a dataset that moved.
    #[test]
    fn the_ranking_is_sorted_once_per_dataset_generation() {
        let mut state = ranked_state(64);
        let first = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
        let again = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
        assert!(Arc::ptr_eq(&first, &again));

        let ActiveSensitivity::Ready(view) = active_sensitivity(&state) else {
            panic!("the fixture retains a sensitivity payload");
        };
        assert_eq!(first.order(), ranked_rows(view.rows).as_slice());
        assert_eq!(
            first.max_magnitude,
            view.rows
                .iter()
                .map(|row| row.normalized.abs())
                .fold(0.0_f64, f64::max)
        );

        state.simulation.runs[0].analyses[0].result_payload =
            Some(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: SensitivityResultMode::Dc,
                rows: vec![SensitivityResultRow {
                    parameter: "only".to_owned(),
                    raw: 1.0,
                    normalized: 2.0,
                }],
            });
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = sensitivity_plan(&mut state).expect("a ranked sensitivity plan");
        assert_eq!(after.order(), [0]);
        assert_eq!(after.max_magnitude, 2.0);
    }

    /// The panel ranks the same thousands of parameters the chart does; it
    /// must lay out only the rows its own viewport can show.
    #[test]
    fn the_panel_lists_only_the_rows_its_viewport_shows() {
        let mut state = ranked_state(4_000);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(520.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| right_panel(ui, &mut state));
            },
        );
        let drawn = output
            .platform_output
            .accesskit_update
            .expect("the sensitivity panel publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| {
                // A Label-role node carries its text in `value`, not `label`.
                node.value().is_some_and(|text| text.starts_with("p0"))
            })
            .count();
        assert!(drawn > 0, "the panel listed no parameters at all");
        assert!(
            drawn < 400,
            "the panel listed {drawn} of 4000 ranked parameters for a 900 px viewport"
        );
    }

    /// The row offsets are part of the plan, not of the frame.
    ///
    /// The prefix sum is `O(parameters)` and it was rebuilt inside the scroll
    /// area on every frame, on the one sheet whose premise is that a real
    /// design ranks thousands of them. It addresses exactly the ranking the
    /// plan already holds, so it belongs beside it and moves only when the
    /// ranking does.
    #[test]
    fn the_chart_row_offsets_are_built_once_beside_the_ranking_they_address() {
        let mut state = ranked_state(2_000);
        let plan = sensitivity_plan(&mut state).expect("a retained sensitivity result");

        assert_eq!(plan.offsets().rows(), plan.order().len());
        assert!(
            (plan.offsets().total_height() - 2_000.0 * ROW_HEIGHT).abs() < 1.0e-3,
            "the plan's extent is {}",
            plan.offsets().total_height()
        );
        assert!(
            std::sync::Arc::ptr_eq(
                &plan,
                &sensitivity_plan(&mut state).expect("the memo is served")
            ),
            "a second frame rebuilt the plan the offsets travel in"
        );
    }
}
