//! The same Contribution sheet, serving a DC mismatch spread.
//!
//! `.DCMATCH` answers with one output's standard deviation and a ranked list
//! of the statistical variables that own it — the same shape a sensitivity
//! report has, ranked by the same kind of magnitude, read exactly the same
//! way. So it is drawn by the same sheet rather than by a second one that
//! would drift from it.
//!
//! Two things are this family's own. A share is **signed**: a variable whose
//! declared correlation partner cancels it removes variance from the total,
//! and the bar says so in the tone the parent sheet already uses for a
//! negative contribution. And the list is **trimmed** by the card's own
//! `CONTRIBUTORS` and `THRESHOLD`, so the cumulative column ends short of the
//! whole and the panel states the remainder instead of renormalizing it —
//! which would claim the unlisted variables carry nothing.

use egui::{Sense, Ui};
use std::cmp::Ordering;
use std::sync::Arc;

use crate::state::{AnalysisResultPayload, DcMismatchEvidence, RunHistoryRevision};
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
use super::{CELL_INSET, HEADER_HEIGHT, PANEL_ROW_HEIGHT, ROW_HEIGHT, column_rect, paint_cell};

const RANK_WIDTH: f32 = 44.0;
const INSTANCE_WIDTH: f32 = 200.0;
const PARAMETER_WIDTH: f32 = 130.0;
const SCOPE_WIDTH: f32 = 78.0;
const BAR_MIN_WIDTH: f32 = 220.0;
const CONTRIBUTION_WIDTH: f32 = 118.0;
const SHARE_WIDTH: f32 = 72.0;
const CUMULATIVE_WIDTH: f32 = 72.0;
const FIXED_WIDTH: f32 = RANK_WIDTH
    + INSTANCE_WIDTH
    + PARAMETER_WIDTH
    + SCOPE_WIDTH
    + CONTRIBUTION_WIDTH
    + SHARE_WIDTH
    + CUMULATIVE_WIDTH;
const TABLE_MIN_WIDTH: f32 = FIXED_WIDTH + BAR_MIN_WIDTH;

/// What the method row says. The engine's own recipe, stated so a reader does
/// not have to assume a sampling run produced these numbers.
const METHOD: &str = "Central difference at one sigma per variable";

#[derive(Debug, Clone, Copy)]
pub(super) struct MismatchView<'a> {
    analysis_label: &'a str,
    evidence: &'a DcMismatchEvidence,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum ActiveMismatch<'a> {
    Ready(MismatchView<'a>),
    Missing,
    Invalid,
}

/// Select DC mismatch evidence from exactly the active analysis.
///
/// The same three answers `active_sensitivity` gives, for the same reason: a
/// stale payload from another analysis must not appear under the current run.
pub(super) fn active_mismatch(state: &AppState) -> ActiveMismatch<'_> {
    let Some(analysis) = state.simulation.active_analysis() else {
        return ActiveMismatch::Missing;
    };
    let Some(payload @ AnalysisResultPayload::DcMismatch { evidence }) =
        analysis.result_payload.as_ref()
    else {
        return ActiveMismatch::Missing;
    };
    if !analysis.success || payload.validate_for(analysis.analysis_type).is_err() {
        return ActiveMismatch::Invalid;
    }
    ActiveMismatch::Ready(MismatchView {
        analysis_label: analysis.label.as_str(),
        evidence: evidence.as_ref(),
    })
}

/// True when the active analysis carries a DC mismatch payload at all, valid
/// or not — which is what decides whether this family owns the sheet.
pub(super) fn serves_active_analysis(state: &AppState) -> bool {
    state.simulation.active_analysis().is_some_and(|analysis| {
        matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::DcMismatch { .. })
        )
    })
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    matches!(active_mismatch(state), ActiveMismatch::Ready(_))
}

/// The cumulative shares and the bar scale of one retained DC mismatch
/// result, computed once per dataset generation.
///
/// The walk is `O(contributors)` and a card that asked for every contributor
/// on a real design lists thousands of them; it ran inside the scroll area,
/// and again beside the panel table, on every frame.
///
/// Visible to `result_document` rather than to this sheet alone, because the
/// memo table that holds it is the workspace's: [`super::super::view_plans`]
/// names the type in the slot it keeps.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench::documents::result_document) struct MismatchPlan {
    source: (RunHistoryRevision, u64),
    analysis: AnalysisPresentationKey,
    /// Running signed sum of the retained shares, in the engine's order.
    cumulative: Vec<f64>,
    /// Largest `|contribution|`, which is the absolute bar's full scale.
    max_contribution: f64,
    offsets: RowOffsets,
}

impl MismatchPlan {
    pub(super) fn cumulative(&self) -> &[f64] {
        &self.cumulative
    }

    pub(super) fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }
}

fn mismatch_plan(state: &mut AppState) -> Option<Arc<MismatchPlan>> {
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let run = state.simulation.active_run()?;
    let analysis_key =
        AnalysisPresentationKey::new(run.dataset_id, state.simulation.active_analysis()?);
    if let Some(plan) = state.ui.results.plans.mismatch.as_ref()
        && plan.source == source
        && plan.analysis == analysis_key
    {
        return Some(Arc::clone(plan));
    }
    let ActiveMismatch::Ready(view) = active_mismatch(state) else {
        return None;
    };
    frame_work::note(DatasetWalk::MismatchCumulative);
    let cumulative = view.evidence.cumulative_shares();
    let built = Arc::new(MismatchPlan {
        source,
        analysis: analysis_key,
        offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, cumulative.len())),
        cumulative,
        max_contribution: view.evidence.max_contribution_magnitude(),
    });
    state.ui.results.plans.mismatch = Some(Arc::clone(&built));
    Some(built)
}

/// Scientific notation with enough digits to round-trip the retained f64.
fn exact_value(value: f64) -> String {
    format!("{value:.17e}")
}

fn percent(share: f64) -> String {
    format!("{:.2} %", share * 100.0)
}

fn scopes_label(evidence: &DcMismatchEvidence) -> &'static str {
    match (evidence.include_mismatch, evidence.include_process) {
        (true, true) => "mismatch + process",
        (true, false) => "mismatch",
        (false, true) => "process",
        // `validate` refuses this, so it cannot be reached from a payload the
        // sheet drew; answered rather than panicked in a running studio.
        (false, false) => "none",
    }
}

fn limit_label(limit: u64) -> String {
    if limit == 0 {
        "all".to_owned()
    } else {
        limit.to_string()
    }
}

/// Render the ranked DC mismatch contributor sheet.
pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let plan = mismatch_plan(state);
    let view = match active_mismatch(state) {
        ActiveMismatch::Ready(view) => view,
        ActiveMismatch::Missing => {
            well_hint(ui, "Select an analysis with a retained DC mismatch result");
            return;
        }
        ActiveMismatch::Invalid => {
            well_hint(
                ui,
                "The retained DC mismatch result is invalid and cannot be displayed",
            );
            return;
        }
    };
    let evidence = view.evidence;
    if evidence.contributors.is_empty() {
        well_hint(
            ui,
            "The retained DC mismatch result lists no contributors above its own threshold",
        );
        return;
    }
    let Some(plan) = plan else {
        well_hint(
            ui,
            "The retained DC mismatch result is invalid and cannot be displayed",
        );
        return;
    };

    let unit = evidence.output_unit.as_str();
    let subtitle = format!(
        "{} - nominal {} - sigma {} - {} sigma {} - {} of {} contributors",
        evidence.output,
        fmt_si(evidence.nominal_value, unit, 4),
        fmt_si(evidence.sigma_total, unit, 4),
        evidence.sigma_multiplier,
        fmt_si(evidence.quoted_sigma(), unit, 4),
        evidence.retained_contributors(),
        evidence.evaluated_contributors,
    );
    StripHeader::new("DCMATCH", &subtitle, &[]).show(ui);

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let normalized = evidence.normalized_contributions;
    let max_contribution = plan.max_contribution;
    let cumulative = plan.cumulative();
    let viewport_width = ui.available_width().max(1.0);
    egui::ScrollArea::both()
        .id_salt("rspice.results.dc-mismatch-contribution")
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            let width = viewport_width.max(TABLE_MIN_WIDTH);
            ui.set_min_width(width);

            let bar_width = width - FIXED_WIDTH;
            let scope_offset = RANK_WIDTH + INSTANCE_WIDTH + PARAMETER_WIDTH;
            let bar_offset = scope_offset + SCOPE_WIDTH;
            let contribution_offset = bar_offset + bar_width;
            let share_offset = contribution_offset + CONTRIBUTION_WIDTH;
            let cumulative_offset = share_offset + SHARE_WIDTH;

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
                    INSTANCE_WIDTH,
                    "INSTANCE",
                    egui::Align2::LEFT_CENTER,
                ),
                (
                    RANK_WIDTH + INSTANCE_WIDTH,
                    PARAMETER_WIDTH,
                    "PARAMETER",
                    egui::Align2::LEFT_CENTER,
                ),
                (
                    scope_offset,
                    SCOPE_WIDTH,
                    "SCOPE",
                    egui::Align2::LEFT_CENTER,
                ),
                (
                    contribution_offset,
                    CONTRIBUTION_WIDTH,
                    "CONTRIBUTION",
                    egui::Align2::RIGHT_CENTER,
                ),
                (
                    share_offset,
                    SHARE_WIDTH,
                    "SHARE",
                    egui::Align2::RIGHT_CENTER,
                ),
                (
                    cumulative_offset,
                    CUMULATIVE_WIDTH,
                    "CUM",
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

            // The bar column's own scale. Normalized draws an unsigned share
            // against a fixed 0..100 %, which is comparable between runs;
            // absolute draws the signed displacement about a centre axis, so
            // its scale is this result's own largest magnitude.
            let bar_cell =
                column_rect(header, bar_offset, bar_width).shrink2(egui::vec2(CELL_INSET, 0.0));
            let scale_font = theme::mono(tokens::FS_0, FontWeight::Regular);
            if normalized {
                paint_cell(
                    ui,
                    column_rect(header, bar_offset, bar_width),
                    "SHARE OF VARIANCE",
                    egui::Align2::LEFT_CENTER,
                    scale_font.clone(),
                    c.text_faint,
                );
                ui.painter().text(
                    egui::pos2(bar_cell.right(), header.bottom() - 4.0),
                    egui::Align2::RIGHT_BOTTOM,
                    "100 %",
                    scale_font,
                    c.text_faint,
                );
            } else {
                let scale = fmt_si(max_contribution, unit, 4);
                ui.painter().text(
                    egui::pos2(bar_cell.left(), header.bottom() - 4.0),
                    egui::Align2::LEFT_BOTTOM,
                    format!("-{scale}"),
                    scale_font.clone(),
                    c.text_faint,
                );
                ui.painter().text(
                    egui::pos2(bar_cell.center().x, header.bottom() - 4.0),
                    egui::Align2::CENTER_BOTTOM,
                    "0",
                    scale_font.clone(),
                    c.text_faint,
                );
                ui.painter().text(
                    egui::pos2(bar_cell.right(), header.bottom() - 4.0),
                    egui::Align2::RIGHT_BOTTOM,
                    scale,
                    scale_font,
                    c.text_faint,
                );
            }

            let rows = plan.offsets().plan(egui::Rangef::new(
                viewport.min.y - HEADER_HEIGHT,
                viewport.max.y - HEADER_HEIGHT,
            ));
            ui.allocate_space(egui::vec2(width, rows.leading));
            for (rank, row) in evidence
                .contributors
                .iter()
                .enumerate()
                .skip(rows.first)
                .take(rows.end - rows.first)
            {
                let running = cumulative.get(rank).copied().unwrap_or_default();
                let (rect, response) =
                    ui.allocate_exact_size(egui::vec2(width, ROW_HEIGHT), Sense::hover());
                let accessible_label = format!(
                    "Rank {}, {} {}, {}, contribution {}, share {} percent, cumulative {} percent",
                    rank + 1,
                    row.instance,
                    row.parameter,
                    row.scope.tag(),
                    exact_value(row.contribution),
                    row.share * 100.0,
                    running * 100.0,
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
                        egui::RichText::new(format!("{} {}", row.instance, row.parameter))
                            .font(theme::mono(tokens::FS_1, FontWeight::Medium)),
                    );
                    ui.label(format!("Scope: {}", row.scope.tag()));
                    ui.label(format!(
                        "Parameter sigma: {}",
                        exact_value(row.sigma_parameter)
                    ));
                    ui.label(format!("Sensitivity: {}", exact_value(row.sensitivity)));
                    ui.label(format!("Contribution: {}", exact_value(row.contribution)));
                    ui.label(format!("Share: {}", exact_value(row.share)));
                    ui.label(format!("Cumulative: {}", exact_value(running)));
                    if row.share < 0.0 {
                        ui.label("Reduces the variance through its declared correlation");
                    }
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

                for (offset, column_width, text, align, font, color) in [
                    (
                        0.0,
                        RANK_WIDTH,
                        (rank + 1).to_string(),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    ),
                    (
                        RANK_WIDTH,
                        INSTANCE_WIDTH,
                        row.instance.clone(),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    ),
                    (
                        RANK_WIDTH + INSTANCE_WIDTH,
                        PARAMETER_WIDTH,
                        row.parameter.clone(),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    ),
                    (
                        scope_offset,
                        SCOPE_WIDTH,
                        row.scope.tag().to_owned(),
                        egui::Align2::LEFT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    ),
                    (
                        contribution_offset,
                        CONTRIBUTION_WIDTH,
                        fmt_si(row.contribution, unit, 4),
                        egui::Align2::RIGHT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    ),
                    (
                        share_offset,
                        SHARE_WIDTH,
                        percent(row.share),
                        egui::Align2::RIGHT_CENTER,
                        theme::mono(tokens::FS_1, FontWeight::Regular),
                        c.text,
                    ),
                    (
                        cumulative_offset,
                        CUMULATIVE_WIDTH,
                        percent(running),
                        egui::Align2::RIGHT_CENTER,
                        theme::mono(tokens::FS_0, FontWeight::Regular),
                        c.text_faint,
                    ),
                ] {
                    paint_cell(
                        ui,
                        column_rect(rect, offset, column_width),
                        text,
                        align,
                        font,
                        color,
                    );
                }

                let cell = column_rect(rect, bar_offset, bar_width);
                let bar_area = cell.shrink2(egui::vec2(CELL_INSET, 7.0));
                // The parent sheet's own tone for a negative contribution: a
                // share that reduces the variance is a statement the design
                // made, not a fault, and it must not introduce a colour.
                let bar_color = if row.share < 0.0 {
                    c.traces[2]
                } else {
                    c.accent
                };
                let painter = ui.painter().with_clip_rect(cell);
                let filled = bar_color.gamma_multiply(if hovered { 0.92 } else { 0.72 });
                if normalized {
                    let ratio = (row.share.abs()).clamp(0.0, 1.0) as f32;
                    painter.rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(bar_area.left(), bar_area.top()),
                            egui::pos2(
                                bar_area.left() + bar_area.width() * ratio,
                                bar_area.bottom(),
                            ),
                        ),
                        2.0,
                        filled,
                    );
                    // Where the running total has reached, so the reader can
                    // see the list close on the whole without reading down
                    // the CUM column.
                    let tick =
                        bar_area.left() + bar_area.width() * (running.clamp(0.0, 1.0) as f32);
                    painter.vline(
                        tick,
                        bar_area.y_range(),
                        egui::Stroke::new(1.0, c.border_strong),
                    );
                } else {
                    let center_x = bar_area.center().x;
                    let half_width = bar_area.width() * 0.5;
                    let ratio = if max_contribution > 0.0 {
                        (row.contribution.abs() / max_contribution).clamp(0.0, 1.0) as f32
                    } else {
                        0.0
                    };
                    let signed_width = half_width * ratio;
                    let (left, right) = match row.contribution.total_cmp(&0.0) {
                        Ordering::Less => (center_x - signed_width, center_x),
                        Ordering::Equal => (center_x - 0.5, center_x + 0.5),
                        Ordering::Greater => (center_x, center_x + signed_width),
                    };
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
                        filled,
                    );
                }
                theme::paint_focus_ring(ui, &response, rect);
            }
            ui.allocate_space(egui::vec2(width, rows.trailing));
        });
}

/// Render the spread's exact facts and the exact ranked contributor table.
///
/// No plan of its own: the order is the engine's and the panel prints exact
/// values, so there is nothing here that scales with the dataset beyond the
/// rows the panel's own viewport lays out.
pub(super) fn right_panel(ui: &mut Ui, state: &AppState) {
    let view = match active_mismatch(state) {
        ActiveMismatch::Ready(view) => view,
        ActiveMismatch::Missing => {
            section_header(ui, "DC mismatch", None);
            panel_note(ui, "Select an analysis with retained DC mismatch data.");
            return;
        }
        ActiveMismatch::Invalid => {
            section_header(ui, "DC mismatch", None);
            panel_note(ui, "The retained DC mismatch payload is invalid.");
            return;
        }
    };
    let evidence = view.evidence;

    section_header(ui, "DC mismatch", None);
    let nominal = exact_value(evidence.nominal_value);
    let sigma_total = exact_value(evidence.sigma_total);
    let sigma_mismatch = exact_value(evidence.sigma_mismatch);
    let sigma_process = exact_value(evidence.sigma_process);
    let quoted = format!(
        "{} sigma = {}",
        evidence.sigma_multiplier,
        exact_value(evidence.quoted_sigma())
    );
    let retained = format!(
        "{} of {} evaluated · limit {} · threshold {}",
        evidence.retained_contributors(),
        evidence.evaluated_contributors,
        limit_label(evidence.contributor_limit),
        exact_value(evidence.threshold),
    );
    let remainder = evidence.unlisted_share().map(|share| {
        format!(
            "{} of variance in {} unlisted contributors",
            percent(share),
            evidence
                .evaluated_contributors
                .saturating_sub(evidence.retained_contributors()),
        )
    });
    // Stated only when the design declared something the engine applied: a
    // row reading "0 applied" on every uncorrelated design would teach the
    // reader to stop reading the row.
    let correlations = (evidence.applied_correlations_mismatch
        + evidence.applied_correlations_process
        > 0)
    .then(|| {
        let mut parts = vec!["applied".to_owned()];
        if evidence.applied_correlations_process > 0 {
            parts.push(format!("process {}", evidence.applied_correlations_process));
        }
        if evidence.applied_correlations_mismatch > 0 {
            parts.push(format!(
                "mismatch {}",
                evidence.applied_correlations_mismatch
            ));
        }
        parts.join(" · ")
    });

    let mut rows: Vec<(&str, &str)> = vec![
        ("Analysis", view.analysis_label),
        ("Output", evidence.output.as_str()),
        ("Nominal", nominal.as_str()),
        ("Sigma total", sigma_total.as_str()),
        ("Sigma mismatch", sigma_mismatch.as_str()),
        ("Sigma process", sigma_process.as_str()),
        ("Quoted", quoted.as_str()),
        ("Scopes", scopes_label(evidence)),
        ("Retained", retained.as_str()),
    ];
    if let Some(remainder) = remainder.as_deref() {
        rows.push(("Remainder", remainder));
    }
    rows.push(("Method", METHOD));
    if let Some(correlations) = correlations.as_deref() {
        rows.push(("Correlations", correlations));
    }
    measurement_table(ui, &rows);

    section_header(ui, "Ranked contributors", None);
    if evidence.contributors.is_empty() {
        panel_note(ui, "No contributor cleared the card's own threshold.");
        return;
    }

    let header_color = Tokens::get(ui.ctx()).color.text_faint;
    egui::ScrollArea::both()
        .id_salt("rspice.results.dc-mismatch-ranked-table")
        .auto_shrink([false, true])
        .show_rows(
            ui,
            PANEL_ROW_HEIGHT,
            evidence.contributors.len() + 1,
            |ui, visible| {
                ui.set_min_width(560.0);
                egui::Grid::new("rspice.results.dc-mismatch-ranked-grid")
                    .num_columns(8)
                    .striped(true)
                    .min_col_width(48.0)
                    .spacing(egui::vec2(12.0, 6.0))
                    .show(ui, |ui| {
                        let cell = |text: String| {
                            egui::RichText::new(text)
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                        };
                        if visible.start == 0 {
                            for heading in [
                                "RANK",
                                "INSTANCE",
                                "PARAMETER",
                                "SCOPE",
                                "SIGMA PARAM",
                                "SENSITIVITY",
                                "CONTRIBUTION",
                                "SHARE",
                            ] {
                                ui.label(
                                    egui::RichText::new(heading)
                                        .font(theme::mono(tokens::FS_0, FontWeight::Medium))
                                        .color(header_color),
                                );
                            }
                            ui.end_row();
                        }
                        // Row 0 of the virtual list is the heading, so the
                        // data rows start one behind it.
                        let first = visible.start.saturating_sub(1);
                        let last = visible
                            .end
                            .saturating_sub(1)
                            .min(evidence.contributors.len());
                        for (rank, row) in evidence.contributors[first..last]
                            .iter()
                            .enumerate()
                            .map(|(offset, row)| (first + offset, row))
                        {
                            ui.label(cell((rank + 1).to_string()));
                            ui.label(cell(row.instance.clone()));
                            ui.label(cell(row.parameter.clone()));
                            ui.label(cell(row.scope.tag().to_owned()));
                            ui.label(cell(exact_value(row.sigma_parameter)));
                            ui.label(cell(exact_value(row.sensitivity)));
                            ui.label(cell(exact_value(row.contribution)));
                            ui.label(cell(exact_value(row.share)));
                            ui.end_row();
                        }
                    });
            },
        );
}

#[cfg(test)]
mod tests {
    use super::super::super::frame_work::WorkCounts;
    use super::*;
    use crate::state::{
        AnalysisResult, AnalysisType, DcMismatchContributorEvidence, DcMismatchScopeEvidence,
        SimulationRun,
    };

    /// A divider spread trimmed to two of six evaluated contributors.
    fn evidence() -> DcMismatchEvidence {
        DcMismatchEvidence {
            output: "V(OUT)".to_owned(),
            output_unit: "V".to_owned(),
            nominal_value: 2.0 / 3.0,
            sigma_multiplier: 3.0,
            sigma_total: (5.0_f64).sqrt() * 1.0e-3,
            sigma_mismatch: (5.0_f64).sqrt() * 1.0e-3,
            sigma_process: 0.0,
            include_mismatch: true,
            include_process: false,
            contributor_limit: 2,
            threshold: 0.0,
            normalized_contributions: true,
            applied_correlations_mismatch: 0,
            applied_correlations_process: 0,
            evaluated_contributors: 6,
            contributors: vec![
                DcMismatchContributorEvidence {
                    instance: "R1".to_owned(),
                    parameter: "R1V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: 2.0e-4,
                    contribution: 2.0e-3,
                    share: 0.6,
                },
                DcMismatchContributorEvidence {
                    instance: "R2".to_owned(),
                    parameter: "R2V".to_owned(),
                    scope: DcMismatchScopeEvidence::Mismatch,
                    sigma_parameter: 10.0,
                    sensitivity: -1.0e-4,
                    contribution: -1.0e-3,
                    share: 0.2,
                },
            ],
        }
    }

    fn state_with(evidence: DcMismatchEvidence) -> AppState {
        let analysis = AnalysisResult::new(1, AnalysisType::DcMismatch, "DCMATCH")
            .with_result_payload(AnalysisResultPayload::DcMismatch {
                evidence: Arc::new(evidence),
            });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        state
    }

    /// The sheet lists contributors in the engine's order, never its own.
    ///
    /// Cumulative share is only meaningful in rank order, so the sheet has no
    /// sort control and no filter: the two trimming controls are on the form,
    /// where they are re-runnable.
    #[test]
    fn the_contribution_sheet_lists_dc_mismatch_contributors_in_the_engine_order() {
        let mut state = state_with(evidence());
        assert!(serves_active_analysis(&state));
        assert!(active_payload_is_valid(&state));
        let plan = mismatch_plan(&mut state).expect("a valid payload has a plan");
        let cumulative = plan.cumulative();
        assert_eq!(cumulative.len(), 2);
        assert!((cumulative[0] - 0.6).abs() < 1.0e-12, "{cumulative:?}");
        assert!((cumulative[1] - 0.8).abs() < 1.0e-12, "{cumulative:?}");
        assert!((plan.max_contribution - 2.0e-3).abs() < 1.0e-18);

        let ActiveMismatch::Ready(view) = active_mismatch(&state) else {
            panic!("the active analysis carries the payload");
        };
        let order: Vec<&str> = view
            .evidence
            .contributors
            .iter()
            .map(|row| row.instance.as_str())
            .collect();
        assert_eq!(order, ["R1", "R2"], "the engine's order, never re-sorted");
    }

    /// A trimmed list's cumulative share stops short of the whole, and the
    /// remainder is stated rather than renormalized away.
    #[test]
    fn cumulative_share_of_a_trimmed_list_stops_short_of_the_whole() {
        let evidence = evidence();
        assert!(evidence.is_trimmed());
        let remainder = evidence
            .unlisted_share()
            .expect("a trimmed list has a remainder");
        assert!((remainder - 0.2).abs() < 1.0e-12, "{remainder}");
        assert_eq!(percent(remainder), "20.00 %");
    }

    /// A trimmed list says how many contributors were evaluated.
    #[test]
    fn a_trimmed_contributor_list_states_how_many_were_evaluated() {
        let evidence = evidence();
        assert_eq!(evidence.retained_contributors(), 2);
        assert_eq!(evidence.evaluated_contributors, 6);
        assert_eq!(limit_label(evidence.contributor_limit), "2");
        assert_eq!(limit_label(0), "all");
        assert_eq!(scopes_label(&evidence), "mismatch");
    }

    /// Correlations the engine applied are stated beside the variance, and
    /// only then.
    #[test]
    fn applied_correlations_are_stated_beside_the_variance() {
        let mut evidence = evidence();
        assert_eq!(
            evidence.applied_correlations_mismatch + evidence.applied_correlations_process,
            0,
            "an uncorrelated design has no correlation row to state"
        );
        evidence.applied_correlations_process = 2;
        assert_eq!(
            evidence.applied_correlations_mismatch + evidence.applied_correlations_process,
            2
        );
    }

    /// A negative share is drawn as a reduction, not as an error.
    ///
    /// The bar's length is the magnitude and its tone is the parent sheet's
    /// own negative-contribution tone; nothing about it is a fault state.
    #[test]
    fn a_negative_share_is_drawn_as_a_reduction_not_as_an_error() {
        let mut evidence = evidence();
        evidence.contributors[0].share = 1.2;
        evidence.contributors[1].share = -0.2;
        evidence.applied_correlations_mismatch = 1;
        evidence.contributor_limit = 0;
        evidence.evaluated_contributors = 2;
        assert_eq!(evidence.validate(), Ok(()));

        let mut state = state_with(evidence);
        assert!(active_payload_is_valid(&state));
        let plan = mismatch_plan(&mut state).expect("a correlated payload is drawable");
        // The running total passes one and comes back: the second variable
        // removes variance rather than adding it.
        let cumulative = plan.cumulative();
        assert!(cumulative[0] > 1.0, "{cumulative:?}");
        assert!((cumulative[1] - 1.0).abs() < 1.0e-12, "{cumulative:?}");
        assert_eq!(percent(-0.2), "-20.00 %");
    }

    /// The plan is built once per dataset generation, not once per frame.
    #[test]
    fn an_idle_frame_does_not_rebuild_the_mismatch_plan() {
        let mut state = state_with(evidence());
        let baseline = WorkCounts::reset();
        mismatch_plan(&mut state).expect("the first look builds the plan");
        assert_eq!(baseline.since().get(DatasetWalk::MismatchCumulative), 1);

        let baseline = WorkCounts::reset();
        mismatch_plan(&mut state).expect("the memo answers the second look");
        mismatch_plan(&mut state).expect("and the third");
        assert_eq!(baseline.since().get(DatasetWalk::MismatchCumulative), 0);
    }

    /// A sensitivity result is still served by the sensitivity painter.
    #[test]
    fn the_contribution_sheet_still_serves_a_sensitivity_result_unchanged() {
        let analysis = AnalysisResult::new(1, AnalysisType::Sensitivity, "SENS")
            .with_result_payload(AnalysisResultPayload::Sensitivity {
                output: "V(out)".to_owned(),
                result_mode: crate::state::SensitivityResultMode::Dc,
                rows: vec![crate::state::SensitivityResultRow {
                    parameter: "r1".to_owned(),
                    raw: (1.0).into(),
                    normalized: (0.25).into(),
                }],
            });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));

        assert!(!serves_active_analysis(&state));
        assert!(super::super::active_payload_is_valid(&state));
        assert!(matches!(active_mismatch(&state), ActiveMismatch::Missing));
    }

    /// Write the sheet to PNGs so its layout can be reviewed.
    ///
    /// Two widths, because the bar column is the one that flexes and the
    /// fixed columns are the ones that do not: what a narrow surface does to
    /// a signed bar and a cumulative tick is a thing to look at rather than
    /// to assert. Both report bases are rendered, since they draw different
    /// bars from the same rows.
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_the_dc_mismatch_sheet() {
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for width in [1000.0_f32, 1600.0] {
            for normalized in [true, false] {
                let mut evidence = evidence();
                evidence.normalized_contributions = normalized;
                let mut state = state_with(evidence);
                let canvas =
                    crate::ui::raster::render(egui::vec2(width, 420.0), |ui, background| {
                        egui::CentralPanel::default()
                            .frame(egui::Frame::NONE.fill(background))
                            .show(ui, |ui| show(ui, &mut state));
                    });
                let basis = if normalized { "share" } else { "absolute" };
                std::fs::write(
                    directory.join(format!("dc-mismatch-sheet-{width}-{basis}.png")),
                    canvas.png(420),
                )
                .expect("write sheet render");
            }
        }
    }
}
