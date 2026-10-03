//! Retained DC mismatch charts and signed contributor readouts.

use super::{CELL_INSET, HEADER_HEIGHT, PANEL_ROW_HEIGHT, ROW_HEIGHT, column_rect, paint_cell};
use crate::{
    presentation::{panel_note, well_hint},
    strip::StripHeader,
    virtual_rows::RowOffsets,
};
use egui::{Sense, Ui};
use rspice_results::dc_mismatch::DcMismatchEvidence;
use rspice_ui_kit::{
    plot::fmt_si,
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{measurement_table, section_header},
};
use std::cmp::Ordering;

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
pub struct MismatchView<'a> {
    pub analysis_label: &'a str,
    pub evidence: &'a DcMismatchEvidence,
}

#[derive(Debug, Clone, Copy)]
pub enum ActiveMismatch<'a> {
    Ready(MismatchView<'a>),
    Missing,
    Invalid,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MismatchPlan {
    /// Running signed sum of the retained shares, in the engine's order.
    cumulative: Vec<f64>,
    /// Largest `|contribution|`, which is the absolute bar's full scale.
    max_contribution: f64,
    offsets: RowOffsets,
}

impl MismatchPlan {
    pub fn max_contribution(&self) -> f64 {
        self.max_contribution
    }

    pub fn cumulative(&self) -> &[f64] {
        &self.cumulative
    }

    pub fn offsets(&self) -> &RowOffsets {
        &self.offsets
    }

    pub fn new(evidence: &DcMismatchEvidence) -> Self {
        let cumulative = evidence.cumulative_shares();
        Self {
            offsets: RowOffsets::from_heights(std::iter::repeat_n(ROW_HEIGHT, cumulative.len())),
            cumulative,
            max_contribution: evidence.max_contribution_magnitude(),
        }
    }
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

pub fn show(ui: &mut Ui, source: ActiveMismatch<'_>, plan: Option<&MismatchPlan>) {
    let view = match source {
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
pub fn right_panel(ui: &mut Ui, source: ActiveMismatch<'_>) {
    let view = match source {
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

#[cfg(any(test, feature = "test-support"))]
pub fn evidence_fixture() -> DcMismatchEvidence {
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
            rspice_results::dc_mismatch::DcMismatchContributorEvidence {
                instance: "R1".to_owned(),
                parameter: "R1V".to_owned(),
                scope: rspice_results::dc_mismatch::DcMismatchScopeEvidence::Mismatch,
                sigma_parameter: 10.0,
                sensitivity: 2.0e-4,
                contribution: 2.0e-3,
                share: 0.6,
            },
            rspice_results::dc_mismatch::DcMismatchContributorEvidence {
                instance: "R2".to_owned(),
                parameter: "R2V".to_owned(),
                scope: rspice_results::dc_mismatch::DcMismatchScopeEvidence::Mismatch,
                sigma_parameter: 10.0,
                sensitivity: -1.0e-4,
                contribution: -1.0e-3,
                share: 0.2,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::evidence_fixture as evidence;
    use super::*;
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
    #[test]
    fn a_trimmed_contributor_list_states_how_many_were_evaluated() {
        let evidence = evidence();
        assert_eq!(evidence.retained_contributors(), 2);
        assert_eq!(evidence.evaluated_contributors, 6);
        assert_eq!(limit_label(evidence.contributor_limit), "2");
        assert_eq!(limit_label(0), "all");
        assert_eq!(scopes_label(&evidence), "mismatch");
    }
    #[test]
    fn a_negative_share_is_drawn_as_a_reduction_not_as_an_error() {
        let mut evidence = evidence();
        evidence.contributors[0].share = 1.2;
        evidence.contributors[1].share = -0.2;
        evidence.applied_correlations_mismatch = 1;
        evidence.contributor_limit = 0;
        evidence.evaluated_contributors = 2;
        assert_eq!(evidence.validate(), Ok(()));

        let plan = MismatchPlan::new(&evidence);
        // The running total passes one and comes back: the second variable
        // removes variance rather than adding it.
        let cumulative = plan.cumulative();
        assert!(cumulative[0] > 1.0, "{cumulative:?}");
        assert!((cumulative[1] - 1.0).abs() < 1.0e-12, "{cumulative:?}");
        assert_eq!(percent(-0.2), "-20.00 %");
    }
}
