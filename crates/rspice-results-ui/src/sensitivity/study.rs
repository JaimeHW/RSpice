//! Retained sensitivity-study charts, frequency selection and exact readouts.

use super::{
    CELL_INSET, HEADER_HEIGHT, NOT_RETAINED, PANEL_ROW_HEIGHT, PARAMETER_WIDTH, RANK_WIDTH,
    ROW_HEIGHT, VALUE_WIDTH, chart_value, column_rect, exact_sensitivity, exact_value, paint_cell,
    unavailable_reason,
};
use crate::{
    presentation::{panel_note, well_hint},
    strip::StripHeader,
    virtual_rows::RowOffsets,
};
use egui::{Sense, Ui};
use rspice_results::sensitivity::{
    SensitivityBasisEvidence, SensitivityStudyEvidence, SensitivityStudyRow, SensitivityValue,
};
use rspice_ui_kit::{
    plot::fmt_si,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{measurement_table, section_header},
};
use std::cmp::Ordering;

const TABLE_MIN_WIDTH: f32 = RANK_WIDTH + PARAMETER_WIDTH + 360.0 + VALUE_WIDTH + PROFILE_WIDTH;

/// What a row's own measured value is worth without a reference to divide by.
///
/// Stated once so the panel and the ranked table say the same thing about a
/// study whose nominal output was zero at the point being read.
const NO_SCALE: &str = "No normalized sensitivity available";

#[derive(Debug, Clone, Copy)]
pub struct StudyView<'a> {
    pub analysis_label: &'a str,
    pub evidence: &'a SensitivityStudyEvidence,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveStudy<'a> {
    Ready(StudyView<'a>),
    Missing,
    Invalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudyPlan {
    /// The point the ranking below was built at. A ranking of one frequency
    /// says nothing about another, so it is part of the key.
    index: usize,
    order: Vec<usize>,
    /// The chart's row offsets, built once with the order they address.
    offsets: RowOffsets,
    /// Largest normalized magnitude at the read point, which is the bar scale.
    max_magnitude: f64,
    /// Largest normalized magnitude anywhere in the sweep.
    ///
    /// The profile column is scaled to this rather than to each row's own
    /// range, so a row that matters nowhere paints flat beside one that
    /// dominates the band — which is the comparison the column exists to make.
    sweep_max_magnitude: f64,
    /// Where each solved point sits across the profile cell, in `[0, 1]`.
    /// Uses a linear axis when the band contains zero. One walk rather than one
    /// per painted row.
    profile_x: Vec<f32>,
}

/// Width of the per-row frequency profile.
const PROFILE_WIDTH: f32 = 132.0;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct SensitivitySheetState {
    pub frequency_index: usize,
}
impl SensitivitySheetState {
    pub fn read_index(&self, evidence: &SensitivityStudyEvidence) -> usize {
        self.frequency_index
            .min(evidence.point_count().saturating_sub(1))
    }
}
pub fn domain_bar(
    ui: &mut Ui,
    evidence: &SensitivityStudyEvidence,
    state: &mut SensitivitySheetState,
) -> bool {
    let SensitivityBasisEvidence::Ac { frequencies_hz, .. } = &evidence.basis else {
        return false;
    };
    if frequencies_hz.len() < 2 {
        return false;
    }
    let options: Vec<String> = frequencies_hz
        .iter()
        .map(|frequency| fmt_si(*frequency, "Hz", 4))
        .collect();
    let selected = state.frequency_index.min(options.len() - 1);
    if let Some(picked) = rspice_ui_kit::widgets::select(
        ui,
        "rspice.results.sensitivity.frequency",
        "Sensitivity frequency",
        &options[selected],
        &options,
        216.0,
    ) {
        state.frequency_index = picked;
    }
    true
}
impl StudyPlan {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn max_magnitude(&self) -> f64 {
        self.max_magnitude
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }

    pub fn new(evidence: &SensitivityStudyEvidence, index: usize) -> Self {
        let order = ranked_rows(evidence, index);
        let max_magnitude = order
            .iter()
            .filter_map(|row| {
                evidence.rows[*row]
                    .normalized
                    .get(index)
                    .copied()
                    .and_then(SensitivityValue::value)
                    .map(f64::abs)
            })
            .fold(0.0_f64, f64::max);
        // The sweep-wide scale and the x positions do not depend on the point the
        // table is read at, so they are rebuilt only when the dataset moves. The
        // memo is keyed by the index as well, so they ride along; a second memo
        // keyed without it would hold the same two walks twice.
        let sweep_max_magnitude = evidence
            .rows
            .iter()
            .flat_map(|row| &row.normalized)
            .filter_map(|value| value.value().map(f64::abs))
            .fold(0.0_f64, f64::max);
        Self {
            index,
            offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, order.len())),
            order,
            max_magnitude,
            sweep_max_magnitude,
            profile_x: profile_positions(&evidence.basis),
        }
    }
}
fn profile_positions(basis: &SensitivityBasisEvidence) -> Vec<f32> {
    let SensitivityBasisEvidence::Ac { frequencies_hz, .. } = basis else {
        return vec![0.0];
    };
    let (Some(first), Some(last)) = (frequencies_hz.first(), frequencies_hz.last()) else {
        return Vec::new();
    };
    let coordinate = |frequency: f64| {
        if *first == 0.0 {
            frequency
        } else {
            frequency.log10()
        }
    };
    let (low, high) = (coordinate(*first), coordinate(*last));
    let span = high - low;
    frequencies_hz
        .iter()
        .map(|frequency| {
            if span > 0.0 {
                (((coordinate(*frequency) - low) / span) as f32).clamp(0.0, 1.0)
            } else {
                0.0
            }
        })
        .collect()
}
fn ranked_rows(evidence: &SensitivityStudyEvidence, index: usize) -> Vec<usize> {
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

/// One row's normalized sensitivity across the whole band.
///
/// The question a table read at one frequency cannot answer is "where in the
/// band does this variable matter", and this answers it for every visible row
/// at once, without a second sheet and without publishing thousands of
/// synthetic waveforms the saved-output contract could not type.
///
/// Drawn as ONE chained polyline rather than a stroke per segment: coincident
/// stroke edges blend to hairlines in this renderer, and a per-segment chain
/// would paint every interior point twice.
fn paint_frequency_profile(
    ui: &Ui,
    cell: egui::Rect,
    plan: &StudyPlan,
    row: &SensitivityStudyRow,
    index: usize,
    color: rspice_ui_kit::palette::Palette,
) {
    let area = cell.shrink2(egui::vec2(CELL_INSET, 6.0));
    if area.width() <= 1.0 || area.height() <= 1.0 || plan.sweep_max_magnitude <= 0.0 {
        return;
    }
    let painter = ui.painter().with_clip_rect(cell);
    let zero_y = area.center().y;
    let half = area.height() * 0.5;
    painter.hline(
        area.x_range(),
        zero_y,
        egui::Stroke::new(1.0, color.border.gamma_multiply(0.7)),
    );

    // Gaps where a value is unavailable: the polyline is broken there rather
    // than bridged, because a line drawn through a point the engine refused
    // would state a derivative it never computed.
    let mut chain: Vec<egui::Pos2> = Vec::new();
    let flush = |chain: &mut Vec<egui::Pos2>| {
        if chain.len() >= 2 {
            painter.add(egui::Shape::line(
                std::mem::take(chain),
                egui::Stroke::new(1.0, color.accent.gamma_multiply(0.85)),
            ));
        } else if let Some(point) = chain.pop() {
            painter.circle_filled(point, 1.25, color.accent.gamma_multiply(0.85));
        }
    };
    for (point, x) in plan.profile_x.iter().enumerate() {
        match row
            .normalized
            .get(point)
            .copied()
            .and_then(|value| value.value())
        {
            Some(value) => {
                let ratio = (value / plan.sweep_max_magnitude).clamp(-1.0, 1.0) as f32;
                chain.push(egui::pos2(
                    area.left() + area.width() * x,
                    zero_y - half * ratio,
                ));
            }
            None => flush(&mut chain),
        }
    }
    flush(&mut chain);

    // A tick at the frequency the table beside it is read at, so the row and
    // the profile are visibly one statement about one band.
    if let Some(x) = plan.profile_x.get(index) {
        let tick = area.left() + area.width() * x;
        painter.vline(
            tick,
            egui::Rangef::new(area.top(), area.bottom()),
            egui::Stroke::new(1.0, color.border_strong),
        );
    }
}

pub fn show(ui: &mut Ui, source: ActiveStudy<'_>, plan: Option<&StudyPlan>) {
    let view = match source {
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

            // The profile column exists only where there is a band to
            // profile: a one-point study would paint a column of single dots
            // that answered nothing.
            let profile_width = if view.evidence.is_swept() {
                PROFILE_WIDTH
            } else {
                0.0
            };
            let bar_width = width - RANK_WIDTH - PARAMETER_WIDTH - VALUE_WIDTH - profile_width;
            let bar_offset = RANK_WIDTH + PARAMETER_WIDTH;
            let value_offset = bar_offset + bar_width;
            let profile_offset = value_offset + VALUE_WIDTH;
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
            if profile_width > 0.0 {
                paint_cell(
                    ui,
                    column_rect(header, profile_offset, profile_width),
                    "VS FREQUENCY",
                    egui::Align2::LEFT_CENTER,
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
                if profile_width > 0.0 {
                    paint_frequency_profile(
                        ui,
                        column_rect(rect, profile_offset, profile_width),
                        plan,
                        row,
                        index,
                        c,
                    );
                }
                theme::paint_focus_ring(ui, &response, rect);
            }
            ui.allocate_space(egui::vec2(width, rows.trailing));
        });
}
pub fn right_panel(ui: &mut Ui, source: ActiveStudy<'_>, plan: Option<&StudyPlan>) {
    let view = match source {
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
    let index = plan.map_or(0, |plan| plan.index);

    section_header(ui, "Sensitivity", None);
    let basis = basis_label(view.evidence);
    let read_at = read_at_label(view.evidence, index);
    let filter = view.evidence.filter_label().to_owned();
    let count = view.evidence.rows.len().to_string();
    let ranked = plan.map_or(&[][..], StudyPlan::order);
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

#[cfg(any(test, feature = "test-support"))]
pub fn swept_evidence() -> SensitivityStudyEvidence {
    SensitivityStudyEvidence {
        output: "V(OUT)".to_owned(),
        filter: "R* PARAM:*".to_owned(),
        basis: SensitivityBasisEvidence::Ac {
            frequencies_hz: vec![10.0, 100.0, 1000.0],
            output: vec![
                rspice_results::simulation_values::ComplexResultValue {
                    real: 1.0,
                    imaginary: 0.0,
                },
                rspice_results::simulation_values::ComplexResultValue {
                    real: 0.5,
                    imaginary: -0.5,
                },
                rspice_results::simulation_values::ComplexResultValue {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::simulation_values::ComplexResultValue;
    #[test]
    fn the_frequency_profile_is_scaled_to_the_sweep_not_to_the_row() {
        let evidence = swept_evidence();
        let plan = StudyPlan::new(&evidence, 0);
        assert_eq!(plan.sweep_max_magnitude, 9.0);
        assert!(plan.max_magnitude < plan.sweep_max_magnitude);

        // Log-frequency positions across a decade-spaced band.
        assert_eq!(plan.profile_x, vec![0.0, 0.5, 1.0]);
        for frequencies in [vec![0.0, 250.0, 1000.0], vec![0.0]] {
            let basis = SensitivityBasisEvidence::Ac {
                frequencies_hz: frequencies.clone(),
                output: Vec::new(),
            };
            let expected = if frequencies.len() == 1 {
                vec![0.0]
            } else {
                vec![0.0, 0.25, 1.0]
            };
            assert_eq!(profile_positions(&basis), expected);
        }

        // A band that covers no span puts everything at the left rather than
        // dividing by zero.
        let degenerate = SensitivityBasisEvidence::Ac {
            frequencies_hz: vec![100.0, 100.0],
            output: vec![
                ComplexResultValue {
                    real: 1.0,
                    imaginary: 0.0,
                },
                ComplexResultValue {
                    real: 1.0,
                    imaginary: 0.0,
                },
            ],
        };
        assert_eq!(profile_positions(&degenerate), vec![0.0, 0.0]);
        assert_eq!(
            profile_positions(&SensitivityBasisEvidence::Dc { output: 1.0 }),
            vec![0.0]
        );
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
    fn the_highest_magnitude_row_is_the_one_ranked_first_at_that_point() {
        let evidence = swept_evidence();
        let plan = StudyPlan::new(&evidence, 0);
        let label = highest_magnitude_label(&evidence, plan.order(), 0);
        assert!(label.starts_with("R1 · "), "{label}");

        let empty = SensitivityStudyEvidence {
            rows: Vec::new(),
            ..swept_evidence()
        };
        assert_eq!(highest_magnitude_label(&empty, &[], 0), "Not retained");
    }
}
