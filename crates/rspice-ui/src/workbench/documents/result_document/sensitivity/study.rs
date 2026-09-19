//! The same Contribution sheet, serving one `.SENS` study.
//!
//! A study is what the engine's complete entries answer: one output,
//! differentiated against the variables one filter selected, at one operating
//! point or at every frequency of an AC sweep. The ranked table is the same
//! table the frozen single-point payload above draws, read at one point of the
//! study, so it is drawn by the same sheet rather than by a second one that
//! would drift from it.
//!
//! Three things are this family's own. The variables carry the engine's own
//! names, so `R1`, `M1_W`, `MOD:VTO` and `PARAM:GAIN` sit in one list and a
//! reader can tell a device value from a model card from a design parameter
//! by the name alone. The filter that produced the list travels with it: four
//! rows chosen out of two thousand is not the report four rows out of four is.
//! And an AC study keeps every frequency it solved, so the table states which
//! one it is read at instead of implying there was only ever one.

use egui::{Sense, Ui};
use rspice_core::analysis::sensitivity::SensitivityValue;
use std::cmp::Ordering;
use std::sync::Arc;

use crate::state::{
    AnalysisResultPayload, RunHistoryRevision, SensitivityBasisEvidence, SensitivityStudyEvidence,
};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

use super::super::AnalysisPresentationKey;
use super::super::frame_work::{self, DatasetWalk};
use super::super::strip::StripHeader;
use super::super::virtual_rows::RowOffsets;
use super::super::{panel_note, well_hint};
use super::{
    CELL_INSET, HEADER_HEIGHT, NOT_RETAINED, PANEL_ROW_HEIGHT, PARAMETER_WIDTH, RANK_WIDTH,
    ROW_HEIGHT, VALUE_WIDTH, chart_value, column_rect, exact_sensitivity, exact_value, paint_cell,
    unavailable_reason,
};

const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + 360.0 + VALUE_WIDTH;

/// What a row's own measured value is worth without a reference to divide by.
///
/// Stated once so the panel and the ranked table say the same thing about a
/// study whose nominal output was zero at the point being read.
const NO_SCALE: &str = "No normalized sensitivity available";

#[derive(Debug, Clone, Copy)]
pub(super) struct StudyView<'a> {
    analysis_label: &'a str,
    evidence: &'a SensitivityStudyEvidence,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ActiveStudy<'a> {
    Ready(StudyView<'a>),
    Missing,
    Invalid,
}

/// Select study evidence from exactly the active analysis.
///
/// The same three answers `active_sensitivity` gives, for the same reason: a
/// stale payload from another analysis must not appear under the current run.
pub(super) fn active_study(state: &AppState) -> ActiveStudy<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
        return ActiveStudy::Missing;
    };
    let Some(payload @ AnalysisResultPayload::SensitivityStudy { evidence }) =
        analysis.result_payload.as_ref()
    else {
        return ActiveStudy::Missing;
    };
    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveStudy::Invalid;
    }
    ActiveStudy::Ready(StudyView {
        analysis_label: analysis.label.as_str(),
        evidence: evidence.as_ref(),
    })
}

/// True when the active analysis carries a study payload at all, valid or
/// not — which is what decides whether this family owns the sheet.
pub(super) fn serves_active_analysis(state: &AppState) -> bool {
    state.simulation.active_analysis().is_some_and(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::SensitivityStudy { .. })
        )
    })
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    matches!(active_study(state), ActiveStudy::Ready(_))
}

/// The point of the study every table on this sheet is read at.
///
/// A study of one point has one answer; a sweep is read at the card's start
/// frequency, which is what the Studio showed for a swept deck before it kept
/// the rest of the sweep.
const READ_INDEX: usize = 0;

/// The ranking of one retained study at the point it is read at.
///
/// A real design selects thousands of variables, and the sort ran on every
/// frame — twice, because the panel ranked them again beside the chart.
///
/// Visible to `result_document` rather than to this sheet alone, because the
/// memo table that holds it is the workspace's: [`super::super::view_plans`]
/// names the type in the slot it keeps.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench::documents::result_document) struct StudyPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    /// The point the ranking below was built at. A ranking of one frequency
    /// says nothing about another, so it is part of the key.
    index: usize,
    order: Vec<usize>,
    /// The chart's row offsets, built once with the order they address.
    offsets: RowOffsets,
    /// Largest normalized magnitude at the read point, which is the bar scale.
    max_magnitude: f64,
}

impl StudyPlan {
    pub(super) fn order(&self) -> &[usize] {
        &self.order
    }

    pub(super) fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }
}

/// Rank by normalized magnitude at one point. Equal magnitudes are ordered by
/// the engine's vector name, giving identical results across platforms.
fn ranked_rows(evidence: &SensitivityStudyEvidence, index: usize) -> Vec<usize> {
    frame_work::note(DatasetWalk::SensitivityRank);
    let magnitude = |row: usize| {
        evidence.rows[row]
            .normalized
            .get(index)
            .copied()
            .and_then(SensitivityValue::value)
    };
    let mut ranked: Vec<usize> = (0..evidence.rows.len()).collect();
    ranked.sort_by(|left, right| {
        match (magnitude(*left), magnitude(*right)) {
            (Some(left), Some(right)) => right.abs().total_cmp(&left.abs()),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
        .then_with(|| {
            evidence.rows[*left]
                .parameter
                .cmp(&evidence.rows[*right].parameter)
        })
    });
    ranked
}

/// The ranking for the active analysis, sorted once per dataset generation
/// and per point read.
fn study_plan(state: &mut AppState) -> Option<Arc<StudyPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    let ActiveStudy::Ready(view) = active_study(state) else {
        return None;
    };
    let index = READ_INDEX;
    if let Some(plan) = state.ui.results.plans.study.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
        && plan.index == index
    {
        return Some(Arc::clone(plan));
    }
    let order = ranked_rows(view.evidence, index);
    let max_magnitude = order
        .iter()
        .filter_map(|row| {
            view.evidence.rows[*row]
                .normalized
                .get(index)
                .copied()
                .and_then(SensitivityValue::value)
                .map(f64::abs)
        })
        .fold(0.0_f64, f64::max);
    let built = Arc::new(StudyPlan {
        source,
        analysis: analysis_key,
        index,
        offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, order.len())),
        order,
        max_magnitude,
    });
    state.ui.results.plans.study = Some(Arc::clone(&built));
    Some(built)
}

/// The whole grid in one sentence: what was solved, over what band.
pub(super) fn basis_label(evidence: &SensitivityStudyEvidence) -> String {
    match &evidence.basis {
        SensitivityBasisEvidence::Dc { .. } => "DC operating point".to_owned(),
        SensitivityBasisEvidence::Ac { frequencies_hz, .. } => match frequencies_hz.as_slice() {
            [] => "AC sweep with no frequencies".to_owned(),
            [only] => format!("AC at {}", fmt_si(*only, "Hz", 4)),
            [first, .., last] => format!(
                "AC sweep {} to {} · {} points",
                fmt_si(*first, "Hz", 4),
                fmt_si(*last, "Hz", 4),
                frequencies_hz.len()
            ),
        },
    }
}

/// The one point the tables are read at, exactly enough to reproduce it.
fn read_at_label(evidence: &SensitivityStudyEvidence, index: usize) -> String {
    evidence.frequency_at(index).map_or_else(
        || "DC operating point".to_owned(),
        |frequency| format!("{} Hz", exact_value(frequency)),
    )
}

fn column_at(column: &[SensitivityValue<f64>], index: usize) -> SensitivityValue<f64> {
    column
        .get(index)
        .copied()
        .unwrap_or(SensitivityValue::unavailable(
            rspice_core::analysis::sensitivity::SensitivityUnavailability::OutOfRange,
        ))
}

fn highest_magnitude_label(
    evidence: &SensitivityStudyEvidence,
    ranked: &[usize],
    index: usize,
) -> String {
    ranked
        .iter()
        .find(|row| {
            column_at(&evidence.rows[**row].normalized, index)
                .value()
                .is_some()
        })
        .map_or_else(
            || {
                if evidence.rows.is_empty() {
                    "Not retained"
                } else {
                    NO_SCALE
                }
                .to_owned()
            },
            |row| {
                let row = &evidence.rows[*row];
                format!(
                    "{} · {} normalized sensitivity",
                    row.parameter,
                    exact_sensitivity(column_at(&row.normalized, index))
                )
            },
        )
}

/// Render the ranked normalized-sensitivity chart of one study.
pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    // Ranked before the payload is borrowed, so the sort happens once per
    // dataset generation rather than once per frame per surface.
    let plan = study_plan(state);
    let view = match active_study(state) {
        ActiveStudy::Ready(view) => view,
        ActiveStudy::Missing => {
            well_hint(ui, "Select an analysis with a retained sensitivity result");
            return;
        }
        ActiveStudy::Invalid => {
            well_hint(
                ui,
                "The retained sensitivity result is invalid and cannot be displayed",
            );
            return;
        }
    };
    if view.evidence.rows.is_empty() {
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
    let index = plan.index;
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
        "{} - {} - {} variables",
        view.evidence.output,
        basis_label(view.evidence),
        ranked.len()
    );
    StripHeader::new("SENS", &subtitle, &[]).show(ui);

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let viewport_width = ui.available_width().max(1.0);
    egui::ScrollArea::both()
        .id_salt("rspice.results.sensitivity-study")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            let width = viewport_width.max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);

            let bar_width = width - RANK_WIDTH - PARAMETER_WIDTH - VALUE_WIDTH;
            let bar_offset = RANK_WIDTH + PARAMETER_WIDTH;
            let value_offset = bar_offset + bar_width;
            let (header, _) =
                ui.allocate_exact_size(egui::vec2(width, HEADER_HEIGHT), Sense::hover());
            ui.painter().hline(
                header.x_range(),
                header.bottom() - 0.5,
                egui::Stroke::new(1.0, c.border),
            );

            let header_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            for (offset, column_width, text, align) in [
                (0.0, RANK_WIDTH, "#", egui::Align2::LEFT_CENTER),
                (
                    RANK_WIDTH,
                    PARAMETER_WIDTH,
                    "VARIABLE",
                    egui::Align2::LEFT_CENTER,
                ),
                (
                    bar_offset,
                    bar_width,
                    "NORMALIZED SENSITIVITY",
                    egui::Align2::LEFT_CENTER,
                ),
                (
                    value_offset,
                    VALUE_WIDTH,
                    "VALUE",
                    egui::Align2::RIGHT_CENTER,
                ),
            ] {
                paint_cell(
                    ui,
                    column_rect(header, offset, column_width),
                    text,
                    align,
                    header_font.clone(),
                    c.text_faint,
                );
            }

            let chart_cell =
                column_rect(header, bar_offset, bar_width).shrink2(egui::vec2(CELL_INSET, 0.0));
            let zero_x = chart_cell.center().x;
            let scale_text = format!("{max_magnitude:.6e}");
            for (x, align, text) in [
                (
                    chart_cell.left(),
                    egui::Align2::LEFT_BOTTOM,
                    format!("-{scale_text}"),
                ),
                (zero_x, egui::Align2::CENTER_BOTTOM, "0".to_owned()),
                (chart_cell.right(), egui::Align2::RIGHT_BOTTOM, scale_text),
            ] {
                ui.painter().text(
                    egui::pos2(x, header.bottom() - 4.0),
                    align,
                    text,
                    theme::mono(tokens::FS_0, FontWeight::Regular),
                    c.text_faint,
                );
            }

            // One row per selected variable: a real design selects thousands,
            // and only the ones on screen are worth laying out.
            let rows = plan.offsets().plan(egui::Rangef::new(
                viewport.min.y - HEADER_HEIGHT,
                viewport.max.y - HEADER_HEIGHT,
            ));
            ui.allocate_space(egui::vec2(width, rows.leading));
            for (rank, row) in ranked
                .iter()
                .map(|row| &view.evidence.rows[*row])
                .enumerate()
                .skip(rows.first)
                .take(rows.end - rows.first)
            {
                let normalized = column_at(&row.normalized, index);
                let raw = column_at(&row.raw, index);
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), Sense::hover());
                let accessible_label = format!(
                    "{}, variable {}, normalized sensitivity {}, raw sensitivity {}",
                    if normalized.value().is_some() {
                        format!("Rank {}", rank + 1)
                    } else {
                        "Unranked".to_owned()
                    },
                    row.parameter,
                    exact_sensitivity(normalized),
                    exact_sensitivity(raw),
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
                let phase = row.phase.get(index).copied();
                let response = response.on_hover_ui(|ui| {
                    ui.label(
                        egui::RichText::new(row.parameter.as_str())
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
                    );
                    ui.label(format!(
                        "Normalized sensitivity: {}",
                        exact_sensitivity(normalized)
                    ));
                    ui.label(format!("Raw: {}", exact_sensitivity(raw)));
                    if let Some(phase) = phase {
                        ui.label(format!("Phase, rad: {}", exact_sensitivity(phase)));
                    }
                    ui.label(format!("Nominal: {}", exact_value(row.nominal_value)));
                    ui.label(format!("Output: {}", view.evidence.output));
                    ui.label(format!("Read at: {}", read_at_label(view.evidence, index)));
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
                    if normalized.value().is_some() {
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

                let bar_cell = column_rect(rect, bar_offset, bar_width);
                match normalized {
                    SensitivityValue::Available(normalized) => {
                        let bar_area = bar_cell.shrink2(egui::vec2(CELL_INSET, 7.0));
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
                        let painter = ui.painter().with_clip_rect(bar_cell);
                        painter.vline(
                            center_x,
                            rect.y_range(),
                            egui::Stroke::new(1.0, c.border_strong),
                        );
                        painter.rect_filled(
                            egui::Rect::from_min_max(
                                egui::pos2(left, bar_area.top()),
                                egui::pos2(right, bar_area.bottom()),
                            ),
                            2.0,
                            bar_color.gamma_multiply(if hovered { 0.92 } else { 0.72 }),
                        );
                    }
                    SensitivityValue::Unavailable { unavailable } => paint_cell(
                        ui,
                        bar_cell,
                        unavailable_reason(unavailable),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    ),
                }

                paint_cell(
                    ui,
                    column_rect(rect, value_offset, VALUE_WIDTH),
                    chart_value(normalized),
                    egui::Align2::RIGHT_CENTER,
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    c.text,
                );
                theme::paint_focus_ring(ui, &response, rect);
            }
            ui.allocate_space(egui::vec2(width, rows.trailing));
        });
}

/// Render the study's context and its exact ranked table.
pub(super) fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let plan = study_plan(state);
    let view = match active_study(state) {
        ActiveStudy::Ready(view) => view,
        ActiveStudy::Missing => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "Select an analysis with retained sensitivity data.");
            return;
        }
        ActiveStudy::Invalid => {
            section_header(ui, "Sensitivity", None);
            panel_note(ui, "The retained sensitivity payload is invalid.");
            return;
        }
    };
    let index = plan.as_ref().map_or(0, |plan| plan.index);

    section_header(ui, "Sensitivity", None);
    let basis = basis_label(view.evidence);
    let read_at = read_at_label(view.evidence, index);
    let filter = view.evidence.filter_label().to_owned();
    let count = view.evidence.rows.len().to_string();
    let ranked = plan.as_deref().map_or(&[][..], StudyPlan::order);
    let highest = highest_magnitude_label(view.evidence, ranked, index);
    measurement_table(
        ui,
        &[
            ("Analysis", view.analysis_label),
            ("Reference metric", view.evidence.output.as_str()),
            ("Basis", basis.as_str()),
            ("Read at", read_at.as_str()),
            ("Filter", filter.as_str()),
            ("Method", NOT_RETAINED),
            ("Normalization", "Retained per variable"),
            ("Variables retained", count.as_str()),
            ("Highest magnitude", highest.as_str()),
        ],
    );

    section_header(ui, "Ranked sensitivity", None);
    if view.evidence.rows.is_empty() {
        panel_note(ui, "No parameter sensitivities were retained.");
        return;
    }

    // The panel ranks the same thousands of variables the chart does, so it
    // lays out only the rows its own viewport can show.
    let has_phase = matches!(view.evidence.basis, SensitivityBasisEvidence::Ac { .. });
    let columns = if has_phase { 5 } else { 4 };
    let header_color = Tokens::get(ui.ctx()).color.text_faint;
    egui::ScrollArea::both()
        .id_salt("rspice.results.sensitivity-study-ranked-table")
        .auto_shrink([false, true])
        .show_rows(ui, PANEL_ROW_HEIGHT, ranked.len() + 1, |ui, visible| {
            ui.set_min_width(470.0);
            egui::Grid::new("rspice.results.sensitivity-study-ranked-grid")
                .num_columns(columns)
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
                        ui.label(heading("VARIABLE"));
                        ui.label(heading("NORMALIZED"));
                        ui.label(heading("RAW"));
                        if has_phase {
                            ui.label(heading("PHASE"));
                        }
                        ui.end_row();
                    }
                    // Row 0 of the virtual list is the heading, so the data
                    // rows start one behind it.
                    let first = visible.start.saturating_sub(1);
                    let last = visible.end.saturating_sub(1).min(ranked.len());
                    for (rank, row) in ranked[first..last]
                        .iter()
                        .map(|row| &view.evidence.rows[*row])
                        .enumerate()
                        .map(|(offset, row)| (first + offset, row))
                    {
                        let normalized = column_at(&row.normalized, index);
                        let cell = |text: String| {
                            egui::RichText::new(text)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        };
                        ui.label(cell(if normalized.value().is_some() {
                            (rank + 1).to_string()
                        } else {
                            "—".to_owned()
                        }));
                        ui.label(cell(row.parameter.clone()));
                        ui.label(cell(exact_sensitivity(normalized)));
                        ui.label(cell(exact_sensitivity(column_at(&row.raw, index))));
                        if has_phase {
                            ui.label(cell(exact_sensitivity(column_at(&row.phase, index))));
                        }
                        ui.end_row();
                    }
                });
        });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, ComplexResultValue, SensitivityStudyRow, SimulationRun,
    };

    pub(super) fn swept_evidence() -> SensitivityStudyEvidence {
        SensitivityStudyEvidence {
            output: "V(OUT)".to_owned(),
            filter: "R* PARAM:*".to_owned(),
            basis: SensitivityBasisEvidence::Ac {
                frequencies_hz: vec![10.0, 100.0, 1000.0],
                output: vec![
                    ComplexResultValue {
                        real: 1.0,
                        imaginary: 0.0,
                    },
                    ComplexResultValue {
                        real: 0.5,
                        imaginary: -0.5,
                    },
                    ComplexResultValue {
                        real: 0.0,
                        imaginary: -0.25,
                    },
                ],
            },
            rows: vec![
                SensitivityStudyRow {
                    parameter: "PARAM:GAIN".to_owned(),
                    nominal_value: 2.0,
                    raw: vec![
                        SensitivityValue::Available(0.25),
                        SensitivityValue::Available(9.0),
                        SensitivityValue::Available(0.5),
                    ],
                    normalized: vec![
                        SensitivityValue::Available(0.25),
                        SensitivityValue::Available(9.0),
                        SensitivityValue::Available(0.5),
                    ],
                    phase: vec![
                        SensitivityValue::Available(0.0),
                        SensitivityValue::Available(0.1),
                        SensitivityValue::Available(0.2),
                    ],
                },
                SensitivityStudyRow {
                    parameter: "R1".to_owned(),
                    nominal_value: 1000.0,
                    raw: vec![
                        SensitivityValue::Available(-1.0),
                        SensitivityValue::Available(-2.0),
                        SensitivityValue::Available(-3.0),
                    ],
                    normalized: vec![
                        SensitivityValue::Available(-1.0),
                        SensitivityValue::Available(-2.0),
                        SensitivityValue::Available(-3.0),
                    ],
                    phase: vec![
                        SensitivityValue::Available(0.0),
                        SensitivityValue::Available(-0.1),
                        SensitivityValue::Available(-0.2),
                    ],
                },
            ],
        }
    }

    pub(super) fn state_with(evidence: SensitivityStudyEvidence) -> AppState {
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS").with_result_payload(
                AnalysisResultPayload::SensitivityStudy {
                    evidence: Arc::new(evidence),
                },
            ),
        );
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));
        state
    }

    #[test]
    fn a_study_is_ranked_once_per_dataset_generation() {
        let mut state = state_with(swept_evidence());
        let first = study_plan(&mut state).expect("a retained study ranks");
        let again = study_plan(&mut state).expect("the memo is served");
        assert!(Arc::ptr_eq(&first, &again));
        // Read at the sweep's first frequency, where R1 has the larger
        // magnitude even though PARAM:GAIN dominates further up the band.
        assert_eq!(first.order(), [1, 0]);
        assert_eq!(first.max_magnitude, 1.0);
        assert_eq!(first.offsets().rows(), 2);
    }

    #[test]
    fn the_sheet_names_the_band_it_solved_and_the_point_it_is_read_at() {
        let evidence = swept_evidence();
        let basis = basis_label(&evidence);
        assert!(basis.contains("AC sweep"), "{basis}");
        assert!(basis.contains("3 points"), "{basis}");
        assert_eq!(
            read_at_label(&evidence, 0),
            format!("{} Hz", exact_value(10.0))
        );

        let dc = SensitivityStudyEvidence {
            basis: SensitivityBasisEvidence::Dc { output: 2.0 },
            rows: evidence
                .rows
                .iter()
                .map(|row| SensitivityStudyRow {
                    parameter: row.parameter.clone(),
                    nominal_value: row.nominal_value,
                    raw: vec![row.raw[0]],
                    normalized: vec![row.normalized[0]],
                    phase: Vec::new(),
                })
                .collect(),
            ..evidence
        };
        dc.validate().expect("the DC projection is valid evidence");
        assert_eq!(basis_label(&dc), "DC operating point");
        assert_eq!(read_at_label(&dc, 0), "DC operating point");
    }

    #[test]
    fn a_study_states_the_filter_that_chose_its_variables() {
        let mut evidence = swept_evidence();
        assert_eq!(evidence.filter_label(), "R* PARAM:*");
        evidence.filter = String::new();
        assert_eq!(evidence.filter_label(), "every device and model parameter");
    }

    #[test]
    fn only_the_active_analysis_and_only_a_valid_payload_is_drawn() {
        let mut state = state_with(swept_evidence());
        assert!(serves_active_analysis(&state));
        assert!(active_payload_is_valid(&state));

        state.simulation.runs[0].analyses[0].success = false;
        assert!(serves_active_analysis(&state), "the payload is still there");
        assert!(matches!(active_study(&state), ActiveStudy::Invalid));
        assert!(study_plan(&mut state).is_none());
    }

    #[test]
    fn the_highest_magnitude_row_is_the_one_ranked_first_at_that_point() {
        let mut state = state_with(swept_evidence());
        let plan = study_plan(&mut state).expect("a retained study ranks");
        let ActiveStudy::Ready(view) = active_study(&state) else {
            panic!("the fixture retains a study");
        };
        let label = highest_magnitude_label(view.evidence, plan.order(), 0);
        assert!(label.starts_with("R1 · "), "{label}");

        let empty = SensitivityStudyEvidence {
            rows: Vec::new(),
            ..swept_evidence()
        };
        assert_eq!(highest_magnitude_label(&empty, &[], 0), "Not retained");
    }

    /// The panel ranks the same thousands of variables the chart does; it must
    /// lay out only the rows its own viewport can show.
    #[test]
    fn the_panel_lists_only_the_rows_its_viewport_shows() {
        let rows = (0..2_000)
            .map(|index| SensitivityStudyRow {
                parameter: format!("R{index:05}"),
                nominal_value: 1000.0,
                raw: vec![SensitivityValue::Available(index as f64)],
                normalized: vec![SensitivityValue::Available((index as f64).sin())],
                phase: Vec::new(),
            })
            .collect();
        let mut state = state_with(SensitivityStudyEvidence {
            output: "V(OUT)".to_owned(),
            filter: String::new(),
            basis: SensitivityBasisEvidence::Dc { output: 1.0 },
            rows,
        });
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
            .expect("the study panel publishes an accessibility tree")
            .nodes
            .iter()
            // A Label-role node carries its text in `value`, not `label`.
            .filter(|(_, node)| node.value().is_some_and(|text| text.starts_with("R0")))
            .count();
        assert!(drawn > 0, "the panel listed no variables at all");
        assert!(
            drawn < 400,
            "the panel listed {drawn} of 2000 ranked variables for a 900 px viewport"
        );
    }

    #[test]
    fn a_swept_study_paints_without_borrowing_its_own_plan_twice() {
        let mut state = state_with(swept_evidence());
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 700.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    show(ui, &mut state);
                    right_panel(ui, &mut state);
                });
            },
        );
    }
}
